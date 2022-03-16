# User's Manual of the Future

![I always dreamed of seeing the future](https://media.giphy.com/media/ZhESFK96NxbuO1yDgy/giphy.gif)

This text is written from the perspective of async Rust's "shiny future". It describes the async Rust that future users will experience. Embedded within are links of the form "deliv_xxx" that connect to the specific deliverables that are being described.

*Note:* Not everything in the future is great. Search for "Caveat" and you'll find a few notes of problems that we don't expect to fix.

## Introduction: Async I/O as a user

### What is async I/O?

These days, most Rust code that interacts with the network or does high-performance I/O is Async I/O. Async I/O is, in some sense, an implementation detail. It is a set of language extensions that make it easy to run many asynchronous tasks using only a small number of underlying *operating system threads*. This means that you can scale up to a very large number of tasks using only a small amount of resources. To be frank, for many applications, async I/O is overkill. However, there are some for which it is absolutely essential, and that's why most of the high quality libraries are using asynchronous interfaces. Fortunately, async Rust is quite easy to use, so even if you don't really *need* the power right now, that's not a problem.

### Choosing a runtime

When you use sync Rust, operations like I/O and so forth are taken care of by your operating system (or your libc implementation, in any case). When you use *async* Rust, though, the mapping between asynchronous tasks is performed by a *library*, called a runtime. One of Rust's key distinguishing features is that it doesn't bake in the choice of a runtime. This means that people are free to develop libaries which use a variety of different strategies to schedule tasks, I/O, and so forth. The choice of runtime can in some cases make a big difference to your overall performance, or what kind of environments you can run in.

If this seems overwhelming, don't worry. Rust makes it easy to experiment with runtimes and try different ones ([deliv_portable]). Here is a list of some of the popular runtimes, and the sorts of applications where they are suitable:

* General purpose, good for just about anything: tokio, async-std
* High-performance file I/O, thread-per-core architecture: glommio
* Focused on reliability: bastion
* Embedded environments: embassy

If you are not sure what's best for you, we recommend picking any of the general purpose runtimes.

### Async fn: where it all starts

Getting started with async Rust is easy. Most anywhere that you write `fn` in Rust, you can now write `async fn` (exception: extern blocks), starting with the main function:

```rust
#[tokio::main] // or async_std::main, glommio::main, etc
async fn main() {
    println!("Hello, world!"); // <-- expect a warning here
}
```

You can see that we decorated `main` with a `#[tokio::main]` attribute. This is how we select the runtime we will use: most runtimes emit a similar decorator, so you could change this to `#[async_std::main]`, `#[glommio::main]`, or `#[embassy::main]` and all the examples and code we talk about in this document would work just the same. ([deliv_portable])

Whichever runtime you choose, if you actually try to compile this, you're going to see that you get a warning ([deliv_lint_blocking_fn]):

```
    println!("Hello, world!");
    ^^^^^^^ synchronous I/O in async fn
```

This is because macros like `println!` expand to *blocking* operations, that take control of the underlying thread and don't allow the scheduler to continue. You need to use the async equivalent ([deliv_portable_stdlib]), then `await` the result:

```rust
async fn main() {
    async_println!("Hello, world!").await;
}
```

When you `await` on something, you are pausing execution and waiting for it to complete before you continue. *Under the hood*, an await corresponds to giving up the current thread of control so that the runtime can do something else instead while you wait (e.g., process another task).

### Documentation and common patterns

This document is a survey of some of the major aspects of writing async functions. If you'd like a deeper introduction, the async book both explains how to get started in async but also common patterns, mistakes to avoid, and some of the details of the various runtimes you can choose from. ([deliv_documentation])

### Async iterators

So far, using `async` seems like mostly more work to accomplish the same thing, since you have to add `await` keywords everywhere. But async functions are like synchronous functions with superpowers: they have the ability to easily compose complex schedules of parallel and concurrent workloads. This is particularly true when you start messing around with asynchronous iterators.

Consider this example. Imagine that you have a bunch of networking requests coming in. For each one, you have to do a bit of lightweight preparation, and then some heavyweight processing. This processing can take up a lot of RAM, and takes a while, so you can only process one request at a time, but you would like to do up to 5 instances of that lightweight preparation in parallel while you wait, so that things are all queued up and ready to go. You want a schedule like this, in other words:

```
   ┌───────────────┐
   │ Preparation 1 │ ─────┐
   └───────────────┘      │
                          │
   ┌───────────────┐      │     ┌───────────────┐
   │ Preparation 2 │ ─────┼────►│ Process item  │ ─────►
   └───────────────┘      │     └───────────────┘
                          │
     ...                  │
                          │
   ┌───────────────┐      │
   │ Preparation 5 │ ─────┘
   └───────────────┘
```

You can create that quite easily:

```rust
async fn do_work(database: &Database) {
    let work = do_select(database, FIND_WORK_QUERY)?;
    stream::iter(work)
        .map(async |item| preparation(database, item).await)
        .buffered(5)
        .for_each(async |work_item| process_work_item(database, work_item).await)
        .await;
}
```

The `buffered` combinator on async iterators creates a schedule that does up to 5 items in parallel, but still produces one item at a time as the result. Thus `for_each` executes on only one item at a time.

How does all this work? The basic `AsyncIterator` trait ([deliv_async_iter]) looks quite similar to the standard `Iterator` trait, except that it has an `async fn` (this fn also has a `#[repr]` annotation; you can ignore it for now, but we discuss it later).

```rust
trait AsyncIter {
    type Item;

    #[repr(inline)]
    async fn next(&mut self) -> Self::Item;
}
```

However, when you use combinators like `buffered` that introduce parallelism, you are now using a *parallel* async iterator ([deliv_async_iter]), similar to the parallel iterators offered by [rayon]. The core operation here is `for_each` (which processes each item in the iterator):

```rust
trait ParAsyncIter {
    type Item;

    async fn for_each(&mut self, op: impl AsyncFn(Self::Item));
}
```

*Editor's note:* There's a subtle difference between `for_each` here and Rayon's `for_each`. It might actually be nice to rework Rayon's approach too. Detail hammering still required!

### Scopes

Parallel async iterators are implemented atop of something called *scopes* ([deliv_scope_api]). Scopes are a way of structuring your async tasks into a hierarchy. In this hierarchy, every parent task waits for its children to complete before it itself is complete. Scopes are also connected to cancellation: when you mark a parent task as cancelled, it propagates that cancellation down to its children as well (but still waits for them to finish up) ([deliv_cancellation]).

Scopes allow you to spawn parallel tasks that access borrowed data ([deliv_borrowed_data]). For example, you could rewrite the parallel iterator above using scopes. For simplicity, we'll ignore the "up to 5 items being prepared" and just spawn a task for all items at once:

```rust
async fn do_work(database: &Database) {
    std::async_thread::scope(async |s| {
        // Channel to send prepared items over to the
        // task that processes them one at a time:
        let mut (tx, rx) = std::async_sync::mpsc::channel();

        // Spawn a task to spawn tasks:
        s.spawn(async move || {
            let work = do_select(database, FIND_WORK_QUERY)?;
            work.for_each(|item| {
                // Spawn a task processing each item and then
                // sending it on the channel:
                s.spawn(async |item| {
                    let prepared_item = preparation(database, item).await
                    tx.send(prepared_item).await;
                });
            });
        });

        // Spawn a task to spawn tasks:
        s.spawn(async move || {
            while let Some(item) = rx.next().await {
                process_item(item).await;
            }
        });
    });
}
```

### Cancellation

Cancelling a task is a common operation in async code. Often this is because of a dropped connection, but it could also be because of non-error conditions, such as waiting for the first of two requests to complete and taking whichever finished first. ([deliv_cancellation])

*Editor's note:* Clearly, this needs to be elaborated. Topics:

* Ambient cancellation flag vs explicit passing
* Connecting to I/O operations so they produce errors
* Opt-in synchronous cancellation, select

### Async read and write traits

The `AsyncRead` and `AsyncWrite` traits are the most common way to do I/O. They are the async equivalent of the `std::io::Read` and `std::io::Write` traits. They are used in a similar way. [deliv_async_read_write]

*Editor's note:* This requires elaboration. The challenge is that the best design for these traits is unclear.

### Async fns in traits, overview

Async functions work in traits, too ([deliv_async_fundamentals]):

```rust
trait HttpRequest {
    async fn request(&self, url: &Url) -> HttpResponse;
}
```

#### Desugaring async fn in traits into `impl Trait` and generic associated types

Async functions actually desugar into functions that return an `impl Future`. When you use an async function in a trait ([deliv_impl_trait_in_trait]), that is desugared into a (generic) associated type in the trait ([deliv_gats]) whose value is inferred by the compiler ([deliv_tait]):

```rust
trait SomeTrait {
    async fn foo(&mut self);
}

// becomes:

trait SomeTrait {
    fn foo<(&mut self) -> impl Future<Output = ()> + '_;
}

// becomes something like:
//
// Editor's note: The name of the associated type is under debate;
// it may or may not be something user can name, though they should
// have *some* syntax for referring to it.

trait SomeTrait {
    type Foo<'me>: Future<Output = ()> + '_
    where
        Self: 'me;

    async fn foo(&mut self) -> Self::Foo<'_>;
}
```

What this means is that the future type `SomeTrait::Foo` is going to be a generated type returned by the compiler that is speciic to that future.

#### Caveat: Gritty details around dyn Trait and no-std

However, there is a catch here. When a trait contains `async fn`, using `dyn` types (e.g., `dyn HttpRequest`, for the trait above) can get a bit complicated. ([deliv_dyn_async_trait]) By default, we assume that folks using `dyn HttpRequest` are doing so in a multithreaded, standard environment. This means that, by default:

* A reference like `&T` can only be cast to `&dyn HttpRequest` if all the `async fn` in its impl are `Send`
    * Note that you can still write impls whose `async fn` are not send, but you cannot use them with `dyn` (again, by default).
* Async calls that go through a `dyn HttpRequest` will allocate a `Box` to store their data
    * This is usually fine, but in particularly tight loops can be a performance hazard.
    * Note that this *only applies* when you use `dyn HttpRequest`; most tight loops tend to use generics like `T: HttpRequest` anyway, and here there is no issue.

These assumptions don't work for everyone, so there are some knobs you can turn:

* You can request that the futures not be assumed to be Send.
* You can change the "smart pointer" type used to allocate data; for example, instead of `Box`, a choice like `Stack<32>` would stack allocate up to 32 bytes (compilation errors will result if more than 32 bytes are required), and `SmallBox<32>` would stack allocate up to 32 bytes but heap allocate after that. ([deliv_dyn_async_trait])
* You can use 'inline' async functions, though these are not always suitable. (These are covered under "Diving into the details".)

The way that all of this is implemented is that users can define their own impls of the form `impl Trait for dyn Trait` ([deliv_dyn_trait]). This permits us to supply a number of derives that can be used to implement the above options.

## Tooling

There are a number of powerful development tools available for debugging, profiling, and tuning your Async Rust applications ([deliv_tooling]). These tools allow you to easily view the current tasks in your application, find out what they are blocked on, and do profiling to see where they spend their time.

Async Rust includes profiling tools that are sufficiently lightweight that you can run them in your production runs, giving very accurate data about what is really happening in your system. They also allow you to process the data in a number of ways, such as viewing profiles per request, or for requests coming from a specific source.

The tools also include "hazard detection" that uncovers potential bugs or performance problems that you may not have noticed. For example, they can identify functions that run too long with any form of await or yield, which can lead to "hogging" the CPU and preventing other tasks from running.

Finally, the tools can make suggestions to help you to tune your async code performance. They can identify code that ought to be outlined into separate functions, for example, or instances where the size of futures can be reduced through judicious use of heap allocation ([deliv_boxable]). These edits come in the form of suggestions, much like the compiler, which can be automatically applied with `cargo fix`.

### Bridging the sync and async worlds

One of the challenges of async programming is how to embed *synchronous* snippets of code. A synchronous snippet is anything that may occupy the thread for a long period of time without executing an await. This might be because it is a very long-running long loop, or it may be because of it invokes blocking primitives (like synchronous I/O). For efficiency, the async runtimes are setup to assume that this doesn't happen. This means that it is your responsibility to mark any piece of synchronous code with a call to `blocking`. This is a signal to the runtime that the code may block, and it allows the runtime to execute the code on another thread or take other forms of action:

```rust
std::async::blocking(|| ...).await;
```

Note that `blocking` is an async function. Interally, it is built on the scope method `spawn_blocking`, which spawns out a task into an inner scope ([deliv_scope_api]):

```rust
async fn blocking<R>(f: impl FnOnce() -> R) -> R {
    scope(|s| s.spawn_blocking(f).await).await
}
```

#### Caveat: Beware the async sandwich

One challenge with integrating sync and async code is called the "async sandwich". This occurs when you have async code that calls into sync code which in turn wishes to invoke async code:

* an `async fn` A that calls ..
* a synchronous `fn` B that wishes to block on ..
* an `async fn` C doing some I/O

The problem here is that, for this to work, the `async fn` A really needs to call the synchronous function with `blocking`, but that may not be apparent, and A may not be in your control (that is, you may be authoring B and/or C, and not be able to modify A). This is a difficult situation without a great answer. Some runtimes offer methods that can help in this situation, but deadlocks may result.

We hope to address this with 'overloaded async' functions, but more work is needed to flesh out that design ([deliv_async_overloading]).

## Diving into the details

The previous topics covered the "high-level view" of async. This section dives a bit more into some of the details of how things work.

### "Inline" async functions

Inline async functions ([deliv_inline_async_fn]) are an optimization that is useful for traits where the trait represents the *primary purpose* of the type that implements it; typically such traits are implemented by dedicated types that exist just for that purpose. Examples include:

* The read and write traits.
* Async iterators.
* Async functions.

Inline async functions are also crucial to `AsyncDrop` ([deliv_async_drop]), discussed below.

Inline async functions are declared within a trait body. They indicate that all intermediate state for the function is stored within the struct itself:

```rust
trait AsyncIter {
    type Item;

    #[repr(inline)]
    async fn next(&mut self) -> Self::Item;
}
```

This implies some limitations, but it has some benefits as well. For example, traits that contain only inline async functions are purely `dyn` safe without any overhead or limitations.

### Boxable heap allocation

One of the challening parts of writing a system that juggles many concurrent requests is deciding how much stack to allocate. Pthread-based systems solve this problem by reserving a very large portion of memory for the stack, but this doesn't scale up well when you have very large numbers of requests. A better alternative is to start with a small stack and grow dynamically: but that can be tricky to do without hitting potential performance hazards.

Rust occupies an interesting spot in the design space. For simple Rust futures, we will allocate *exactly as much stack space as is needed*. This is done by analyzing the future and seeing what possible calls it may make.

Sometimes, though, this analysis is not possible. For example, a recursive function can use infinite stack. In cases like this, you can annotate your async function to indicate that its stack space should be allocated on the heap where it is called ([deliv_boxable]):

```rust
box async fn foo() { .. }
```

These annotations are also useful for tuning performance. The tooling ([deliv_tooling]) can be used to suggest inserting `box` keywords on cold code paths, thus avoiding allocating stack space that is rarely used.

### Async drop

Cleaning up resources in async code is done using destructors, just as in synchronous Rust. Simply implement the `AsyncDrop` trait ([deliv_async_drop]) instead of `Drop`, and you're good to go:

```rust
impl AsyncDrop for MyType {
    async fn drop(&mut self) {
        ...
    }
}
```

Just as in synchronous Rust, you are advised to keep destructors limited in their effects.

#### Caveat: Synchronous drop

One thing to be aware of when you implement `AsyncDrop` is that, because any Rust value can be dropped at any point, the type system will allow your type to be dropped synchronously as well. We do however have a lint that detects the most common cases and gives you a warning, so this is rare in practice.

**Note:** If a type that implements `AsyncDrop` *but not `Drop`* is dropped synchronously, the program will abort!

#### Caveat: Implicit await points

One other thing to be aware of is that async drop will trigger implicit awaits each time a value is dropped (e.g., when a block is exited). This is rarely an issue.

[deliv_portable]: ../roadmap/portable.md
[deliv_lint_blocking_fn]: ../roadmap/polish/lint_blocking_fns.md
[deliv_portable_stdlib]: ../roadmap/portable/stdlib.md
[deliv_documentation]: ../roadmap/documentation.md
[deliv_async_iter]: ../roadmap/async_iter.md
[deliv_cancellation]: ../roadmap/borrowed_data_and_cancellation.md
[deliv_borrowed_data]: ../roadmap/borrowed_data_and_cancellation.md
[deliv_async_read_write]: ../../design_docs/async_read_write.md
[deliv_impl_trait_in_trait]: https://rust-lang.github.io/async-fundamentals-initiative/roadmap/impl_trait_in_traits.html
[deliv_gats]: https://github.com/rust-lang/generic-associated-types-initiative
[deliv_tait]: https://github.com/rust-lang/rust/issues/63063
[deliv_dyn_async_trait]: https://rust-lang.github.io/async-fundamentals-initiative/roadmap/dyn_async_trait.html
[deliv_dyn_trait]: https://rust-lang.github.io/async-fundamentals-initiative/roadmap/dyn_trait.html
[deliv_scope_api]: ../roadmap/scopes/scope_api.md
[deliv_inline_async_fn]: ../roadmap/async_fn/inline_async_fn.md
[deliv_async_drop]: https://rust-lang.github.io/async-fundamentals-initiative/roadmap/async_drop.html
[deliv_async_fundamentals]: https://rust-lang.github.io/async-fundamentals-initiative/
[deliv_async_overloading]: ../roadmap/async_overloading.md
[deliv_tooling]: ../roadmap/tooling.md
[deliv_boxable]: ../roadmap/async_fn/boxable.md
