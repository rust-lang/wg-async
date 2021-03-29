# 😱 Status quo stories: Barbara trims a stacktrace

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

Barbara is triaging the reported bugs for her SLOW library. For each bug, she tries to quickly see if she can diagnose the basic area of code that is affected so she knows which people to ping to help fix it. She opens a bug report from a user complaining about a panic when too many connections arrive at the same time. The bug report includes a backtrace from the user's code, and it looks like this:

```
thread 'main' panicked at 'something bad happened here', src/main.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:519:12
   1: stacks_rs::process_one::{{closure}}
             at ./src/main.rs:16:5
   2: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:80:19
   3: stacks_rs::process_many::{{closure}}
             at ./src/main.rs:10:5
   4: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:80:19
   5: stacks_rs::main::{{closure}}::{{closure}}
             at ./src/main.rs:4:9
   6: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:80:19
   7: stacks_rs::main::{{closure}}
             at ./src/main.rs:3:5
   8: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:80:19
   9: tokio::park::thread::CachedParkThread::block_on::{{closure}}
             at /home/serg/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.3.0/src/park/thread.rs:263:54
  10: tokio::coop::with_budget::{{closure}}
             at /home/serg/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.3.0/src/coop.rs:106:9
  11: std::thread::local::LocalKey<T>::try_with
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:272:16
  12: std::thread::local::LocalKey<T>::with
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:248:9
  13: tokio::coop::with_budget
             at /home/serg/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.3.0/src/coop.rs:99:5
  14: tokio::coop::budget
             at /home/serg/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.3.0/src/coop.rs:76:5
  15: tokio::park::thread::CachedParkThread::block_on
             at /home/serg/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.3.0/src/park/thread.rs:263:31
  16: tokio::runtime::enter::Enter::block_on
             at /home/serg/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.3.0/src/runtime/enter.rs:151:13
  17: tokio::runtime::thread_pool::ThreadPool::block_on
             at /home/serg/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.3.0/src/runtime/thread_pool/mod.rs:71:9
  18: tokio::runtime::Runtime::block_on
             at /home/serg/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.3.0/src/runtime/mod.rs:452:43
  19: stacks_rs::main
             at ./src/main.rs:1:1
  20: core::ops::function::FnOnce::call_once
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Barbara finds the text overwhelming. She can't just browse it to figure out what code is affected. Instead, she pops up a new tab with gist.github.com copies the text into that handy text box and starts deleting stuff. To start, she deletes the first few lines until her code appears, then she deletes:

* the extra lines from calls to `poll` that are introduced by the async fn machinery;
* the bits of code that come from tokio that don't affect her;
* the intermediate wrappers from the standard library pertaining to thread-local variables.

She's a bit confused by the `::{closure}` lines on her symbols but she learned by now that this is normal for `async fn`. After some work, she has reduced her stack to this:

```
thread 'main' panicked at 'something bad happened here', src/main.rs:16:5
stack backtrace:
   1: stacks_rs::process_one::{{closure}} at ./src/main.rs:16:5
   3: stacks_rs::process_many::{{closure}} at ./src/main.rs:10:5
   5: stacks_rs::main::{{closure}}::{{closure}} at ./src/main.rs:4:9
   7: stacks_rs::main::{{closure}} at ./src/main.rs:3:5
  13: <tokio stuff> 
  19: stacks_rs::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Based on this, she is able to figure out who to ping about the problem. She pastes her reduced stack trace into the issue pings Alan, who is responsible that module. Alan thanks her for reducing the stack trace and mentions, "Oh, when I used to work in C#, this is what the stack traces always looked like. I miss those days."

Fin.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

* **What are the morals of the story?**
    * Rust stack traces -- but async stack traces in particular -- reveal lots of implementation details to the user:
        * Bits of the runtime and intermediate libraries whose source code is likely not of interest to the user (but it might be);
        * Intermediate frames from the stdlib;
        * `::{closure}` symbols on async functions and blocks (even though they don't appear to be closures to the user);
        * calls to `poll`.
* **What are the sources for this story?**
    * [Sergey Galich](https://github.com/rust-lang/wg-async-foundations/issues/69#issuecomment-803208049) reported this problem, among many others.
* **Why did you choose Barbara to tell this story?**
    * She knows about the desugarings that give rise to symbols like `::{closure}`, but she still finds them annoying to deal with in practice.
* **How would this story have played out differently for the other characters?**
    * Other characters might have wasted a lot of time trying to read through the stack trace in place before editing it.
    * They might not have known how to trim down the stack trace to something that focused on their code, or it might have taken them much longer to do so.
* **How does this compare to other languages?**
    * Rust's async model does have some advantages, because the complete stack trace is available unless there is an intermediate `spawn`.
    * Other languages have developed special tools to connect async functions to their callers, however, which gives them a nice experience. For example, Chrome has a [UI for enabling stacktraces that cross await points](https://www.html5rocks.com/en/tutorials/developertools/async-call-stack/#toc-enable).
* **Why doesn't Barbara view this in a debugger?**
    * Because it came in an issue report (or, freqently, as a crash report or email).
* **Doesn't Rust have backtrace trimming support?**
    * Yes, this **is** the reduced backtrace. You don't even want to know what the [full one](https://gist.github.com/eminence/0b3e697b7c4e686451ff0d37c169c89d) looks like. Don't click it. Don't!
    
[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
