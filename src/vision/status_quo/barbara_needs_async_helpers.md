# 😱 Status quo stories: Barbara needs Async Helpers

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[Barbara], an experienced Rust user, is prototyping an async Rust service for work. To get things working quickly, she decides to prototype in [`tokio`], since it is unclear which runtime her work will use.

She starts adding warp and [`tokio`] to her dependencies list. She notices that [`warp`] [suggests](https://github.com/seanmonstar/warp/#example) using [`tokio`] with the `full` feature. She's a bit concerned about how this might affect the compile times and also that *all* of tokio is needed for her little project, but she pushes forward.

As she builds out functionality, she's pleased to see tokio provides a bunch of helpers like [`join!`](https://docs.rs/tokio/1.4.0/tokio/macro.join.html) and async versions of the standard library types like channels and mutexes.

After completing one endpoint, she moves to a new one which requires streaming http responses to the client. Barbara quickly finds out from [`tokio`] [docs](https://docs.rs/tokio/1.4.0/tokio/stream/index.html), that it does not provide a stream type, and so she adds [`tokio-stream`] to her dependencies.

Moving on she tries to make some functions generic over the web framework underneath, so she tries to abstract off the functionality to a trait. So she writes an async function inside a trait, just like a normal function.

```rust
trait Client {
    async fn get();
}
```

Then she gets a helpful error message.

```
error[E0706]: functions in traits cannot be declared `async`
 --> src/lib.rs:2:5
  |
2 |     async fn get();
  |     -----^^^^^^^^^^
  |     |
  |     `async` because of this
  |
  = note: `async` trait functions are not currently supported
  = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
```

She then realizes that Rust doesn't support async functions in traits yet, so she adds [`async-trait`] to her dependencies.

Some of her functions are recursive, and she wanted them to be async functions, so she sprinkles some `async/.await` keywords in those functions.

```rust
async fn sum(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        n + sum(n - 1).await
    }
}
```

Then she gets an error message.

```
error[E0733]: recursion in an `async fn` requires boxing
 --> src/lib.rs:1:27
  |
1 | async fn sum(n: usize) -> usize {
  |                           ^^^^^ recursive `async fn`
  |
  = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
```

So to make these functions async she starts boxing her futures. She also asks one of her peers for a code review asynchronously, and after awaiting their response, she learns about the [`async-recursion`] crate. Then she adds [`async-recursion`] to the dependencies.

As she is working, she realizes that what she really needs is to write a `Stream` of data. She starts trying to write her `Stream` implementation and spends several hours banging her head against her desk in frustration (her challenges are pretty similar to what [Alan experienced](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_hates_writing_a_stream.html)). Ultimately she's stuck trying to figure out why her `&mut self.foo` call is giving her errors:

```
error[E0277]: `R` cannot be unpinned
  --src/main.rs:52:26
   |
52 |                 Pin::new(&mut self.reader).poll_read(cx, buf)
   |                          ^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `R`
   |
   = note: required by `Pin::<P>::new`
help: consider further restricting this bound
   |
40 |     R: AsyncRead + Unpin,
   |                  ^^^^^^^
```

Fortunately, that weekend, [@fasterthanlime](https://github.com/fasterthanlime) publishes a [blog post](https://fasterthanli.me/articles/pin-and-suffering) covering the gory details of `Pin`. Reading that post, she learns about [`pin-project`], which she adds as a dependency. She's able to get her code working, but it's kind of a mess. Feeling quite proud of herself, she shows it to a friend, and they suggest that maybe she ought to try the [`async-stream`] crate. Reading that, she realizes she can use this crate to simplify some of her streams, though not all of them fit.

"Finally!", Barbara says, breathing a sigh of relief. She is done with her prototype, and shows it off at work, but to her dismay, the team decides that they need to use a custom runtime for their use case. They're building an embedded system and it has relatively limited resources. Barbara thinks, "No problem, it should be easy enough to change runtimes, right?"

So now Barbara starts the journey of replacing tokio with a myriad of off the shelf and custom helpers. She can't use warp so now she has to find an alternative. She also has to find a new channel implementations and there are a few:
* In [`futures`]
* [`async-std`] has one, but it seems to be tied to another runtime so she can't use that.
* [`smol`] has one that is independent.

This process of "figure out which alternative is an option" is repeated many times. She also tries to use the [`select!`](https://docs.rs/futures/0.3.14/futures/macro.select.html) macro from [`futures`] but it requires more pinning and workarounds (not to mention a [stack overflow](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_runs_into_stack_trouble.html) or two).

But Barbara fights through all of it. In the end, she gets it to work, but she realizes that she has a ton of random dependencies and associated compilation time. She wonders if all that dependencies will have a negative effect on the binary size. She also had to rewrite some bits of functionality on her own.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* Functionality is found either in "framework"-like crates (e.g., tokio) *and* spread around many different ecosystem crates.
* It's sometimes difficult to discover where this functionality lives.
* Additionally, the trouble of non runtime-agnostic libraries becomes very apparent.
* Helpers and utilities might have analogues across the ecosystem, but they are different in subtle ways.
* Some patterns are clean if you know the right utility crate and very painful otherwise.

### **What are the sources for this story?**
[Issue 105](https://github.com/rust-lang/wg-async-foundations/issues/105)

### **What are helper functions/macros?**
They are functions/macros that helps with certain basic pieces of functionality and features. Like to await on multiple futures concurrently (`join!` in tokio), or else race the futures and take the result of the one that finishes first.

### **Why did you choose [Barbara] to tell this story?**
This particular issue impacts all users of Rust even (and sometimes especially) experienced ones.

### **How would this story have played out differently for the other characters?**
Other characters may not know all their options and hence might have fewer problems as a result.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade

[`tokio`]: https://crates.io/crates/tokio/
[`tokio-stream`]: https://crates.io/crates/tokio-stream/
[`futures`]: https://crates.io/crates/futures/
[`async-recursion`]: https://crates.io/crates/async-recursion/
[`async-trait`]: https://crates.io/crates/async-trait/
[`async-stream`]: https://crates.io/crates/async-stream/
[`async-std`]: https://crates.io/crates/async-std/
[`pin-project`]: https://crates.io/crates/pin-project/
[`smol`]: https://crates.io/crates/smol/
[`warp`]: https>//crates.io/crates/warp/
