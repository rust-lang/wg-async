# Scopes

## Impact

* Able to spawn parallel tasks or blocking work that accesses borrowed data
* Easily create expressive scheduler patterns that make use of borrowed data using high-level combinators and APIs
* When data is no longer needed, able to cancel work and have it reliably and promptly terminate, including any subtasks or other bits of work it may have created
* Cancellation does not leave work "half-finished", but reliably cleans up program state
* Able to use DMA, io-uring, etc to write directly into output buffers, and to recover in the case of cancellation

## Requires

* [capability](./scopes/capability.md)
* [APIs](./scopes/api.md)

## Design notes

Async functions are commonly written with borrowed references as arguments:

```rust
async fn do_something(db: &Db) { ... }
```

but important utilities like `spawn` and `spawn_blocking` require `'static` tasks. Building on non-cancelable traits, we can implement a "scope" API that allows one to introduce an async scope. This scope API should permit one to spawn tasks into a scope, but have various kinds of scopes (e.g., synchronous execution, parallel execution, and so forth). It should ultimately reside in the standard library and hook into different runtimes for scheduling. This will take some experimentation!

```rust
async fn foo(db: &Database) {
    let result = std::async_thread::scope(|s| {
        let job1 = s.spawn(async || {
            async_thing(db)
        });
        let job2 = s.spawn_blocking(|| {
            sync_thing(db)
        });

        (job1.await, job2.await)
    }).await;
}
```

### Side-stepping the nested await problem

One goal of scopes is to avoid the "nested await" problem, as described in [Barbara battles buffered streams (BBBS)][bbbs]. The idea is like this: the standard combinators which run work "in the background" and which give access to intermediate results from that work should schedule that work into a scope.[^hard] This would typically be done by using an "interior iterator" pattern, but it could also be done by taking a scope parameter. Some examples from today's APIs are `FuturesUnordered` and `Stream::buffered`.

[^hard]: This is not a hard rule. But invoking poll manually is best regarded as a risky thing to be managed with care -- not only because of the formal safety guarantees, but because of the possibility for "nested await"-style failures.

[bbbs]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo/barbara_battles_buffered_streams.html
[`buffered`]: https://docs.rs/futures/0.3.15/futures/prelude/stream/trait.StreamExt.html#method.buffered

In the case of [BBBS], the problem arises because of `buffered`, which spawns off concurrent work to process multiple connections. Under this system, the implementation of `buffered` would create an internal scope for spawn its tasks into that scope, side-stepping the problem. One could imagine also offering a variant of `buffered` like `buffered_in` that takes a scope parameter, permitting the user to choose the scope of those spawned tasks:

```rust
async fn do_work(database: &Database) {
    std::async_thread::scope(|s| {
        let work = do_select(database, FIND_WORK_QUERY).await?;
        std::async_iter::from_iter(work)
            .map(|item| do_select(database, work_from_item(item)))
            .buffered_in(5, scope)
            .for_each(|work_item| process_work_item(database, work_item))
            .await;
    }).await;
}
```

### Concurrency without scopes: Join, select, race, and friends

It is possible to introduce concurrency in ways that both (a) do not require scopes and (b) avoid the "nested await" problem. Any combinator which takes multiple `Async` instances and polls them to completion (or cancels them) before it itself returns is ok. This includes:

- `join`, because the `join(a, b)` doesn't complete until both `a` and `b` have completed;
- `select`, because selecting will cancel the alternatives that are not chosen;
- `race`, which is a variant of select.

This is important because embedded systems often avoid allocators, and the scope API implicitly requires allocation (one can spawn an unbounded number of tasks).

### Cancellation

In today's Rust, any async function can be synchronously cancelled at any await point: the code simply stops executing, and destructors are run for any extant variables. This leads to a lot of bugs. (TODO: link to stories)

Under systems like [Swift's proposed structured concurrency model](https://github.com/apple/swift-evolution/blob/main/proposals/0304-structured-concurrency.md), or with APIs like [.NET's CancellationToken](https://docs.microsoft.com/en-us/dotnet/api/system.threading.cancellationtoken?view=net-5.0), cancellation is "voluntary". What this means is that when a task is cancelled, a flag is set; the task can query this flag but is not otherwise affected. Under structured concurrency systems, this flag is propagated to all chidren (and transitively to their children).

[preemption]: https://tokio.rs/blog/2020-04-preemption

Voluntary cancellation is a requirement for [scoped access](./scoped.md). If there are parallel tasks executing within a scope, and the scope itself is canceled, those parallel tasks must be joined and halted before the memory for the scope can be freed.

One downside of such a system is that cancellation _may not_ take effect. We can make it more likely to work by integrating the cancellation flag into the standard library methods, similar to how tokio encourages ["voluntary preemption"][preemption]. This means that file reads and things will start to report errors (`Err(TaskCanceled)`) once the task has been canceled. This has the advantage that it exercises existing error paths and permits recovery.

### Cancellation and `select`

The `select` macro chooses from N futures and returns the first one that matches. Today, the others are immediately canceled. This behavior doesn't play especially well with voluntary cancellation. There are a few options here:

- We could make `select` signal cancellation for each of the things it is selecting over and then wait for them to finish.
- We could also make `select` continue to take `Future` (not `Async`) values, which effectively makes `Future` a "cancel-safe" trait (or perhaps we introduce a `CancelSafe` marker trait that extends `Async`).
  - This would mean that typical `async fn` could not be given to select, though we might allow people to mark `async fn` as "cancel-safe", in which case they would implement `Future`. They would also not have access to ordinary async fn, though.
    - Effectively, the current `Future` trait becomes the "cancel-safe" form of `Async`. This is a bit odd, since it has other distinctions, like using `Pin`, so it might be preferable to use a 'marker trait'.
  - Of course, users could spawn a task that calls the function and give the handle to select.

## Frequently asked questions

### Could there be a convenient way to access the current scope?

If we wanted to integrate the idea of scopes more deeply, we could have some way to get access to the current scope and reference its lifetime. Lots of unknowns to work out here, though. For example, suppose you have a function that creates a scope and invokes a closure within. Do we have a way to indicate to the closure that `'scope` in that closure may be different?

It starts to feel like simply passing "scope" values may be simpler, and perhaps we need a way to automate the threading of state instead. Another advantage of passing a `scope` explicitly is that it is clear when parallel tasks may be launched.

### How does cancellation work in other settings?

Many other languages use a shard flag to observe when cancellation has been requested. 

In some languages, there is also an immediate callback that is invoked when cancellation is requested which permits you to take immediate action. [Swift proposal E0304](https://github.com/apple/swift-evolution/blob/main/proposals/0304-structured-concurrency.md#cancellation-handlers), for example, includes "cancellation handlers" that are run immediately.

* [Kotlin cancellation](https://kotlinlang.org/docs/cancellation-and-timeouts.html):
    * You can invoke `cancel` on launched jobs (spawned tasks).
    * Cancelling sets a flag that the job can check for.
    * Builtin functions check for the flag and throw an exception if it is set.
        * If you need a builtin function to run post-cancellation, you can [run the code in a "non-cancelable" context](https://kotlinlang.org/docs/cancellation-and-timeouts.html#run-non-cancellable-block).

### What is the relationship between AsyncDrop and cancellation?

In async Rust today, one signals cancellation of a future by (synchronously) dropping it. This _forces_ the future to stop executing, and drops the values that are on the stack. Experience has shown that this is someting users have a lot of trouble doing correctly, particularly at fine granularities (see e.g. [Alan builds a cache](/vision/status_quo/alan_builds_a_cache.md) or [Barbara gets burned by select](/vision/status_quo/barbara_gets_burned_by_select.md)).

Given `AsyncDrop`, we could adopt a similar convention, where canceling an `Async` is done by (asynchronously) dropping it. This would presumably amend the unsafe contract of the `Async` trait so that the value must be polled to completion _or_ async-dropped. To avoid the footguns we see today, a typical future could simply continue execution from its `AsyncDrop` method (but disregard the result). It might however set an internal flag to true or otherwise allow the user to find out that it has been canceled. It's not clear, though, precisely what value is being added by `AsyncDrop` in this scenario versus the `Async` simply not implementing `AsyncDrop` -- perhaps though it serves as an elegant way to give both an immediate "cancellation" callback and an opportunity to continue.

An alternative is to use a cancellation token of some kind, so that _scopes_ can be canceled and that cancelation can be observed. The main reason to have that token or observation mechanism be "built-in" to some degree is so that it can be observed and used to drive "voluntary cancellation" from I/O routines and the like. Under that model, `AsyncDrop` would be intended more for values (like database handles) that have cleanup to be done, much like `Drop` today, and less as a way to signal cancellation.
