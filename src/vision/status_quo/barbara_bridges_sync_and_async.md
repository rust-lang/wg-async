# 😱 Status quo stories: Barbara bridges sync and async in `perf.rust-lang.org`

[How To Vision: Status Quo]: ../how_to_vision/status_quo.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async-foundations/master/src/vision/status_quo/template.md
[`status_quo`]: https://github.com/rust-lang/wg-async-foundations/tree/master/src/vision/status_quo
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async-foundations/blob/master/src/SUMMARY.md
[open issues]: https://github.com/rust-lang/wg-async-foundations/issues?q=is%3Aopen+is%3Aissue+label%3Astatus-quo-story-ideas
[open an issue of your own]: https://github.com/rust-lang/wg-async-foundations/issues/new?assignees=&labels=good+first+issue%2C+help+wanted%2C+status-quo-story-ideas&template=-status-quo--story-issue.md&title=

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

### Introducing `block_on`

Barbara is working on the code for [perf.rust-lang.org] and she wants to do a web request to load various intermediate results. She has heard that the `reqwest` crate is quite nice, so she decides to give it a try. She writes up an async function that does her web request:

[perf.rust-lang.org]: https://perf.rust-lang.org/

```rust
async fn do_web_request(url: &Url) -> Data {
    ...
}
```

She needs to apply this async function to a number of urls. She wants to use the iterator map function, like so:

```rust
async fn do_web_request(url: &Url) -> Data {...}

fn aggregate(urls: &[Url]) -> Vec<Data> {
    urls
        .iter()
        .map(|url| do_web_request(url))
        .collect()
}

fn main() {
    /* do stuff */
    let data = aggregate();
    /* do more stuff */
}
```

Of course, since `do_web_request` is an async fn, she gets a type error from the compiler:

```
error[E0277]: a value of type `Vec<Data>` cannot be built from an iterator over elements of type `impl Future`
  --> src/main.rs:11:14
   |
11 |             .collect();
   |              ^^^^^^^ value of type `Vec<Data>` cannot be built from `std::iter::Iterator<Item=impl Future>`
   |
   = help: the trait `FromIterator<impl Future>` is not implemented for `Vec<Data>`
```

"Of course," she thinks, "I can't call an async function from a closure." She decides that since she is not overly concerned about performance, so she decides she'll just use a call to [`block_on` from the `futures` crate](https://docs.rs/futures/0.3.14/futures/executor/fn.block_on.html) and execute the function synchronously:

```rust
async fn do_web_request(url: &Url) -> Data {...}

fn aggregate(urls: &[Url]) -> Vec<Data> {
    urls
        .iter()
        .map(|url| futures::executor::block_on(do_web_request(url)))
        .collect()
}

fn main() {
    /* do stuff */
    let data = aggregate();
    /* do more stuff */
}
```

The code compiles, and it seems to work.

### Introducing `block_on`

As Barbara works on [perf.rust-lang.org], she realizes that she needs to do more and more async operations. She decides to convert her synchronous `main` function into an `async main`. She's using tokio, so she is able to do this very conveniently with the `#[tokio::main]` decorator:

```rust
#[tokio::main]
async fn main() {
    /* do stuff */
    let data = aggregate();
    /* do more stuff */
}
```

Everything seems to work ok on her laptop, but when she pushes the code to production, it deadlocks immediately. "What's this?" she says. Confused, she runs the code on her laptop a few more times, but it seems to work fine.

She decides to try debugging. She fires up a debugger but finds it is isn't really giving her useful information about what is stuck (she has [basically the same problems that Alan has](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_tries_to_debug_a_hang.html)). [She wishes she could get insight into tokio's state.](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/barbara_wants_async_insights.html)

Frustrated, she starts reading the tokio docs more closely and she realizes that `tokio` runtimes offer their own `block_on` method. "What the hey," she thinks, "let's see if that works any better." She changes the `aggregate` function to use tokio's `block_on`:

```rust=
fn block_on<O>(f: impl Future<Output = O>) -> O {
    let rt  = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(f)
}

fn aggregate(urls: &[Url]) -> Vec<Data> {
    urls
        .iter()
        .map(|url| block_on(do_web_request(url)))
        .collect()
}
```

The good news is that the deadlock is gone. The bad news is that now she is getting a panic:

> thread 'main' panicked at 'Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks.'

"Well," she thinks, "I could use that `Handle` API to get the current runtime, maybe that will work?"

```rust
fn aggregate(urls: &[&str]) -> Vec<String> {
    let handle = Handle::current();
    urls.iter()
        .map(|url| handle.block_on(do_web_request(url)))
        .collect()
}
```

But it seems to give her the same panic. 

### Trying out `spawn_blocking`

Reading more into this problem, she realizes she is supposed to be using `spawn_blocking`. She tries replacing `block_on` with `tokio::task::spawn_blocking`:

```rust=
fn aggregate(urls: &[Url]) -> Vec<Data> {
    urls
        .iter()
        .map(|url| tokio::task::spawn_blocking(move || do_web_request(url)))
        .collect()
}
```


but now she gets a type error again:

```
error[E0277]: a value of type `Vec<Data>` cannot be built from an iterator over elements of type `tokio::task::JoinHandle<impl futures::Future>`
  --> src/main.rs:22:14
   |
22 |             .collect();
   |              ^^^^^^^ value of type `Vec<Data>` cannot be built from `std::iter::Iterator<Item=tokio::task::JoinHandle<impl futures::Future>>`
   |
   = help: the trait `FromIterator<tokio::task::JoinHandle<impl futures::Future>>` is not implemented for `Vec<Data>`
```

Of course! `spawn_blocking`, like `map`, just takes a regular closure, not an async closure. She's getting a bit frustrated now. "Well," she thinks, "I can use `spawn` to get into an async context!" So she adds a call to `spawn` inside the `spawn_blocking` closure:

```rust
fn aggregate(urls: &[Url]) -> Vec<Data> {
    urls
        .iter()
        .map(|url| tokio::task::spawn_blocking(move || {
            tokio::task::spawn(async move {
                do_web_request(url).await
            })
        }))
        .collect()
}
```

But this isn't really helping, as `spawn` still yields a future. She's getting the same errors.

### Async all the way

She remembers now that this whole drama started because she was converting her `main` function to be `async`. Maybe she doesn't have to bridge between sync and async? She starts digging around in the docs and finds `futures::join_all`. Using that, she can change `aggregate` to be an async function too:

```rust
async fn aggregate(urls: &[Url]) -> Vec<Data> {
    futures::join_all(
        urls
            .iter()
            .map(|url| do_web_request(url))
    ).await
}
```

Things are working again now, so she is happy.

### Filtering

Later on, she would like to apply a filter to the aggregation operation. She realizes that if she wants to use the fetched data when doing the filtering, she has to filter the vector after the join has completed; `join_all` doesn't have a way to put a filter into the iterator chain like she wants. She is annoyed, but performance isn't critical, so it's ok.

### And the cycle begins again

Later on, she wants to call `aggregate` from another binary. This one doesn't have an `async main`. This context is deep inside of an iterator chain and was previously entirely synchronous. She realizes it would be a lot of work to change all the intervening stack frames to be `async fn`, rewrite the iterators into streams, etc. She decides to just call `block_on` again, even though it make her nervous.

## 🤔 Frequently Asked Questions

### What are the morals of the story?

* Some projects don't care about max performance and just want things to work.
    * They would probably be happy with sync but as the most popular libraries for web requests, databases, etc, offer async interfaces, they may still be using async code.
* There are contexts where you can't easily add an `await`.
    * For example, inside of an iterator chain.
    * Big block of existing code.
* Mixing sync and async code (`block_on`) can cause deadlocks that are really painful to diagnose.


### Why did you choose Barbara to tell this story?

* Because Mark (who experienced most of it) is a very experienced Rust developer.
* Because you could experience this story regardless of language background or being new to Rust.

### How would this story have played out differently for the other characters?

I would expect it would work out fairly similarly, except that the type errors and things might well have been more challenging for people to figure out, assuming they aren't already familiar with Rust.

### What are other ways people could experience similar problems mixing sync and async?

* Using `std::Mutex` in async code.
* Calling the blocking version of an asynchronous API.
    * For example, `reqwest::blocking`, the synchronous `[zbus`](https://gitlab.freedesktop.org/dbus/zbus/-/blob/main/zbus/src/proxy.rs#L121) and [`rumqtt`](https://github.com/bytebeamio/rumqtt/blob/8de24cbc0484f459246251873aa6c80be8b6e85f/rumqttc/src/client.rs#L224) APIs.
    * These are commonly implemented by using some variant of `block_on` internally.
    * Therefore they can lead to panics or deadlocks depending on what async runtime they are built from and used with.

### How many variants of `block_on` are there?

* the `futures` crate offers a runtime-independent block-on (which can lead to deadlocks, as in this story)
* the `tokio` crate offers a `block_on` method (which will panic if used inside of another tokio runtime, as in this story)
* the [`pollster`](https://crates.io/crates/pollster) crate exists just to offer `block_on`
* the [`futures-lite`](https://docs.rs/futures-lite/1.11.3/futures_lite/future/fn.block_on.html) crate offers a `block_on`
* the [`aysnc-std`](https://docs.rs/async-std/1.9.0/async_std/task/fn.block_on.html) crate offers `block_on`
* the [`async-io`](https://docs.rs/async-std/1.9.0/async_std/task/fn.block_on.html) crate offers `block_on`
* ...there are probably more, but I think you get the point.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
