# 😱 Status quo stories: Barbara compares some code (and has a performance problem)

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Barbara is recreating some code that has been written in other languages they have some familiarity with. These include C++, but
also GC'd languages like Python.

This code collates a large number of requests to network services, with eash response containing a large amount of data.
To speed this up, Barbara uses `buffer_unordered`, and writes code like this:

```
let mut queries = futures::stream::iter(...)
    .map(|query| async move {
        let d: Data = self.client.request(&query).await?;
        d
     })
     .buffer_unordered(32);

use futures::stream::StreamExt;
let results = queries.collect::<Vec<Data>>().await;
```

Barbara thinks this is similar in function to things she has seen using
Python's [asyncio.wait](https://docs.python.org/3/library/asyncio-task.html#asyncio.wait),
as well as some code her coworkers have written using c++20's `coroutines`,
using [this](https://github.com/facebook/folly/blob/master/folly/experimental/coro/Collect.h#L321):

```
std::vector<folly::coro::Task<Data>> tasks;
 for (const auto& query : queries) {
    tasks.push_back(
        folly::coro::co_invoke([this, &query]() -> folly::coro::Task<Data> {
              co_return co_await client_->co_request(query);
        }
    )
}
auto results = co_await folly:coro::collectAllWindowed(
      move(tasks), 32);
```

However, *the Rust code performs quite poorly compared to the other impls,
appearing to effectively complete the requests serially, despite on the surface
looking like effectively identical code.*

Barbara goes deep into investigating this, spends time reading how `buffer_unordered` is
implemented, how its underlying `FuturesUnordered` is implemented, and even thinks about
how polling and the `tokio` runtime she is using works. She evens tries to figure out if the
upstream service is doing some sort of queueing.

Eventually Barbara starts reading more about c++20 coroutines, looking closer at the folly
implementation used above, noticing that is works primarily with *tasks*, which are not exactly
equivalent to rust `Future`'s.

Then it strikes her! `request` is implemented something like this:
```
impl Client {
    async fn request(&self) -> Result<Data> {
        let bytes = self.inner.network_request().await?
        Ok(serialization_libary::from_bytes(&bytes)?)
   }
}
```
The results from the network service are VERY large, and the `BufferedUnordered` stream is contained within 1 tokio task.
**The request future does non-trivial cpu work to deserialize the data.
This causes significant slowdowns in wall-time as the the process CAN BE bounded by the time it takes
the single thread running the tokio-task to deserialize all the data.**

The solution is to spawn tasks (note this requires `'static` futures):

```
let mut queries = futures::stream::iter(...)
    .map(|query| async move {
        let d: Data = tokio::spawn(
        self.client.request(&query)).await??;
        d
     })
     .buffer_unordered(32);

use futures::stream::StreamExt;
let results = queries.collect::<Vec<Data>>().await;
```

Barbara was able to figure this out by reading enough and trying things out, but had that not worked, it
would have probably required figuring out how to use `perf` or some similar tool.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* Producing concurrent, performant code in Rust async is not always trivial. Debugging performance
  issues can be difficult.
* Rust's async model, particularly the blocking nature of `polling`, can be complex to reason about,
  and in some cases is different from other languages choices in meaningful ways.
* CPU-bound code can be easily hidden.

### **What are the sources for this story?**
* This is a issue I personally hit while writing code required for production.

### **Why did you choose *Barbara* to tell this story?**
That's probably the person in the cast that I am most similar to, but Alan
and to some extent Grace make sense for the story as well.

### **How would this story have played out differently for the other characters?**
* Alan: May have taken longer to figure out.
* Grace: Likely would have been as interested in the details of how polling works.
* Niklaus: Depends on their experience.
