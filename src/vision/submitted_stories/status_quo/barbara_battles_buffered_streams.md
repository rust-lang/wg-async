# 😱 Status quo stories: Barbara battles buffered streams

[How To Vision: Status Quo]: ../status_quo.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async-foundations/master/src/vision/status_quo/template.md
[`status_quo`]: https://github.com/rust-lang/wg-async-foundations/tree/master/src/vision/status_quo
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async-foundations/blob/master/src/SUMMARY.md
[open issues]: https://github.com/rust-lang/wg-async-foundations/issues?q=is%3Aopen+is%3Aissue+label%3Astatus-quo-story-ideas
[open an issue of your own]: https://github.com/rust-lang/wg-async-foundations/issues/new?assignees=&labels=good+first+issue%2C+help+wanted%2C+status-quo-story-ideas&template=-status-quo--story-issue.md&title=

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

### Mysterious timeouts

Barbara is working on her [YouBuy] server and is puzzling over a strange bug report. She is encountering users reporting that their browser connection is timing out when they connect to [YouBuy]. Based on the logs, she can see that they are timing out in the `do_select` function:

```rust
async fn do_select<T>(database: &Database, query: Query) -> Result<Vec<T>> {
    let conn = database.get_conn().await?;
    conn.select_query(query).await
}
```
This is surprising, because `do_select` doesn't do much - it does a database query to claim a work item from a queue, but isn't expected to handle a lot of data or hit extreme slowdown on the database side.
Some of the time, there is some kind of massive delay in between the `get_conn` method opening a connection and the call to `select_query`. But why? She has metrics that show that the CPU is largely idle, so it's not like the cores are all occupied.

She looks at the caller of `do_select`, which is a function `do_work`:

```rust
async fn do_work(database: &Database) {
    let work = do_select(database, FIND_WORK_QUERY)?;
    stream::iter(work)
        .map(|item| do_select(database, work_from_item(item)))
        .buffered(5)
        .for_each(|work_item| process_work_item(database, work_item))
        .await;
}

async fn process_work_item(...) { }
```

The `do_work` function is invoking `do_select` as part of a stream; it is buffering up a certain number of `do_select` instances and, for each one, invoking `process_work_item`. Everything seems to be in order, and she can see that calls to `process_work_item` are completing in the logs.

Following a hunch, she adds more logging in and around the `process_work_item` function and waits a few days to accumulate new logs. She notices that shortly after each time out, there is always a log of a `process_work_item` call that takes at least 20 seconds. These calls are not related to the connections that time out, they are for other connections, but they always appear afterwards in time.

`process_work_item` is expected to be slow sometimes because it can end up handling large items, so this is not immediately surprising to Barbara. She is, however, surprised by the correlation - surely the executor ensures that `process_work_item` can't stop `do_select` from doing its job?

### Barbara thought she understood how async worked

Barbara thought she understood futures fairly well. She thought of `async fn` as basically "like a synchronous function with more advanced control flow". She knew that Rust's futures were lazy -- that they didn't start executing until they were awaited -- and she knew that could compose them using utilities like [`join`](https://docs.rs/futures/0.3/futures/future/fn.join.html), [`FuturesUnordered`], or the [`buffered`](https://docs.rs/futures/0.3/futures/stream/trait.StreamExt.html#method.buffered) method (as in this example). 

[`FuturesUnordered`]: https://docs.rs/futures/0.3.14/futures/stream/struct.FuturesUnordered.html

Barbara also knows that every future winds up associated with a task, and that if you have multiple futures on the same task (in this case, the futures in the stream, for example) then they would run concurrently, but not in parallel. Based on this, she thinks perhaps that `process_work_item` is a CPU hog that takes too long to complete, and so she needs to add a call to `spawn_blocking`. But when she looks more closely, she realizes that `process_work_item` is an async function, and those 20 seconds that it spends executing are mostly spent waiting on I/O. Huh, that's confusing, because the task ought to be able to execute other futures in that case -- so why are her connections stalling out without making progress?

### Barbara goes deep into how poll works

She goes to read the Rust async book and tries to think about the model, but she can't quite see the problem. Then she asks on the rust-lang Discord and someone explains to her what is going on, with the catchphrase "remember, `async` is about waiting in parallel, not working in parallel". Finally, after reading over what they wrote a few times, and reading some chapters in the async book, she sees the problem.

It turns out that, to Rust, a task is kind of a black box with a "poll" function. When the executor thinks a task can make progress, it calls poll. The task itself then delegates this call to poll down to all the other futures that are composed together. In the case of her buffered stream of connections, the stream gets woken up and it would then delegate down the various buffered items in its list.

When it executes `Stream::for_each`, the task is doing something like this:

```rust
while let Some(work_item) = stream.next().await {
    process_work_item(database, work_item).await;
}
```

The task can only "wait" on one "await" at a time. It will execute that await until it completes and only then move on to the rest of the function. When the task is blocked on the first `await`, it will process all the futures that are part of the stream, and hence the various buffered connections all make progress. 

But once a work item is produced, the task will block on the *second* `await` -- the one that resulted from `process_work_item`. This means that, until `process_work_item` completes, control will *never return to the first `await`*. As a result, none of the futures in the stream will make progress, even if they could do so!

### The fix

Once Barbara understands the problem, she considers the fix. The most obvious fix is to spawn out tasks for the `do_select` calls, like so:

```rust
async fn do_work(database: &Database) {
    let work = do_select(database, FIND_WORK_QUERY)?;
    stream::iter(work)
        .map(|item| task::spawn(do_select(database, work_from_item(item))))
        .buffered(5)
        .for_each(|work_item| process_work_item(database, work_item))
        .await;
}
```

Spawning a task will allow the runtime to keep moving those tasks along independently of the `do_work` task. Unfortunately, this change results in a compilation error:

```
error[E0759]: `database` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
  --> src/main.rs:8:18
   |
8  | async fn do_work(database: &Database) {
   |                  ^^^^^^^^  --------- this data with an anonymous lifetime `'_`...
   |                  |
   |                  ...is captured here...
   |        .map(|item| task::spawn(do_select(database, work_from_item(item))))
   |                    ----------- ...and is required to live as long as `'static` here
```

"Ah, right," she says, "spawned tasks can't use borrowed data. I wish I had [rayon] or the scoped threads from [crossbeam]."

"Let me see," Barbara thinks. "What else could I do?" She has the idea that she doesn't have to process the work items immediately. She could buffer up the work into a [`FuturesUnordered`] and process it after everything is ready:

```rust
async fn do_work(database: &Database) {
    let work = do_select(database, FIND_WORK_QUERY)?;
    let mut results = FuturesUnordered::new();
    stream::iter(work)
        .map(|item| do_select(database, work_from_item(item)))
        .buffered(5)
        .for_each(|work_item| {
            results.push(process_work_item(database, work_item));
            futures::future::ready(())
        })
        .await;

    while let Some(_) = results.next().await { }
}
```

This changes the behavior of her program quite a bit though. The original goal was to have at most 5 `do_select` calls occurring concurrently with exactly one `process_work_item`, but now she has all of the `process_work_item` calls executing at once. Nonetheless, the hack solves her immediate problem. Buffering up work into a `FuturesUnordered` becomes a kind of "fallback" for those cases where can't readily insert a `task::spawn`.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

* Rust's future model is a 'leaky abstraction' that works quite differently from futures in other languages. It is prone to some subtle bugs that require a relatively deep understanding of its inner works to understand and fix.
* "Nested awaits" -- where the task blocks on an inner await while there remains other futures that are still awaiting results -- are easy to do but can cause a lot of trouble.
* Lack of scoped futures makes it hard to spawn items into separate tasks for independent processing sometimes.

### **What are the sources for this story?**

This is based on the bug report [Footgun with Future Unordered](https://github.com/rust-lang/futures-rs/issues/2387) but the solution that Barbara came up with is something that was relayed by [farnz](https://github.com/farnz) vision doc writing session. [farnz] mentioned at the time that this pattern was frequently used in their codebase to work around this sort of hazard.

### **Why did you choose Barbara to tell this story?**

To illustrate that knowing Rust -- and even having a decent handle on async Rust's basic model -- is not enough to make it clear what is going on in this particular case.

### **How would this story have played out differently for the other characters?**

Woe be unto them! Identifying and fixing this bug required a lot of fluency with Rust and the async model. Alan in particular was probably relying on his understanding of async-await from other languages, which works very differently. In those languages, every async function is enqueued automatically for independent execution, so hazards like this do not arise (though this comes at a performance cost).

### Besides timeouts for clients, what else could go wrong?

The original bug report mentioned the possibility of deadlock:

> When using an async friendly semaphore (like Tokio provides), you can deadlock yourself by having the tasks that are waiting in the `FuturesUnordered` owning all the semaphores, while having an item in a `.for_each()` block after `buffer_unordered()` requiring a semaphore.

### Is there any way for Barbara to both produce and process work items simultaneously?

Yes, in this case, she could've. For example, she might have written

```rust
async fn do_work(database: &Database) {
    let work = do_select(database, FIND_WORK_QUERY).await?;

    stream::iter(work)
        .map(|item| async move {
            let work_item = do_select(database, work_from_item(item)).await;
            process_work_item(database, work_item).await;
        })
        .buffered(5)
        .for_each(|()| std::future::ready(()))
        .await;
}
```

This would however mean that she would have 5 calls to `process_work_item` executing at once. In the actual case that inspired this story, `process_work_item` can take as much as 10 GB of RAM, so having multiple concurrent calls is a problem.

### Is there any way for Barbara to both produce and process work items simultaneously, without the buffering and so forth?

Yes, she might use a loop with a `select!`. This would ensure that she is processing *both* the stream that produces work items and the [`FuturesUnordered`] that consumes them:

```rust
async fn do_work(database: &Database) {
    let work = do_select(database, FIND_WORK_QUERY).await?;

    let selects = stream::iter(work)
        .map(|item| do_select(database, work_from_item(item)))
        .buffered(5)
        .fuse();
    tokio::pin!(selects);

    let mut results = FuturesUnordered::new();

    loop {
        tokio::select! {
            Some(work_item) = selects.next() => {
                results.push(process_work_item(database, work_item));
            },
            Some(()) = results.next() => { /* do nothing */ },
            else => break,
        }
    }
}
```

Note that doing so is producing code that looks quite a bit different than where she started, though. :( This also behaves very differently. There can be a queue of tens of thousands of items that `do_select` grabs from, and this code will potentially pull far too many items out of the queue, which then would have to be requeued on shutdown. The intent of the `buffered(5)` call was to grab 5 work items from the queue at most, so that other hosts could pull out work items and share the load when there's a spike.

[character]: ../../characters.md
[status quo stories]: ../status_quo.md
[Alan]: ../../characters/alan.md
[Grace]: ../../characters/grace.md
[Niklaus]: ../../characters/niklaus.md
[Barbara]: ../../characters/barbara.md
[htvsq]: ../status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
[YouBuy]: ../../projects/YouBuy.md
[farnz]: https://github.com/farnz


