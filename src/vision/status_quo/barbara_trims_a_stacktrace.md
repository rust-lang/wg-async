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

```ignore
thread 'main' panicked at 'something bad happened here', src/main.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:519:12
   1: slow_rs::process_one::{{closure}}
             at ./src/main.rs:16:5
   2: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:80:19
   3: slow_rs::process_many::{{closure}}
             at ./src/main.rs:10:5
   4: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:80:19
   5: slow_rs::main::{{closure}}::{{closure}}
             at ./src/main.rs:4:9
   6: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /home/serg/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/mod.rs:80:19
   7: slow_rs::main::{{closure}}
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
  19: slow_rs::main
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

```ignore
thread 'main' panicked at 'something bad happened here', src/main.rs:16:5
stack backtrace:
   1: slow_rs::process_one::{{closure}} at ./src/main.rs:16:5
   3: slow_rs::process_many::{{closure}} at ./src/main.rs:10:5
   5: slow_rs::main::{{closure}}::{{closure}} at ./src/main.rs:4:9
   7: slow_rs::main::{{closure}} at ./src/main.rs:3:5
  13: <tokio stuff> 
  19: slow_rs::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Based on this, she is able to figure out who to ping about the problem. She pastes her reduced stack trace into the issue pings Alan, who is responsible that module. Alan thanks her for reducing the stack trace and mentions, "Oh, when I used to work in C#, this is what the stack traces always looked like. I miss those days."

Fin.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**
* Rust stack traces -- but async stack traces in particular -- reveal lots of implementation details to the user:
    * Bits of the runtime and intermediate libraries whose source code is likely not of interest to the user (but it might be);
    * Intermediate frames from the stdlib;
    * `::{closure}` symbols on async functions and blocks (even though they don't appear to be closures to the user);
    * calls to `poll`.

### **What are the sources for this story?**
[Sergey Galich](https://github.com/rust-lang/wg-async-foundations/issues/69#issuecomment-803208049) reported this problem, among many others.

### **Why did you choose Barbara to tell this story?**
She knows about the desugarings that give rise to symbols like `::{closure}`, but she still finds them annoying to deal with in practice.

### **How would this story have played out differently for the other characters?**
* Other characters might have wasted a lot of time trying to read through the stack trace in place before editing it.
* They might not have known how to trim down the stack trace to something that focused on their code, or it might have taken them much longer to do so.

### **How does this compare to other languages?**
* Rust's async model does have some advantages, because the complete stack trace is available unless there is an intermediate `spawn`.
* Other languages have developed special tools to connect async functions to their callers, however, which gives them a nice experience. For example, Chrome has a [UI for enabling stacktraces that cross await points](https://www.html5rocks.com/en/tutorials/developertools/async-call-stack/#toc-enable).

### **Why doesn't Barbara view this in a debugger?**
* Because it came in an issue report (or, freqently, as a crash report or email).
* But also, that isn't necessarily an improvement! Expand below if you would like to see what we mean.

<details>
<summary>(click to see how a backtrace looks in lldb)</summary>

```ignore
* thread #1, name = 'foo', stop reason = breakpoint 1.1
  * frame #0: 0x0000555555583d24 foo`foo::main::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h617d49d0841ffc0d((null)=closure-0 @ 0x00007fffffffae38, (null)=<unavailable>) at main.rs:11:13
    frame #1: 0x0000555555583d09 foo`_$LT$T$u20$as$u20$futures_util..fns..FnOnce1$LT$A$GT$$GT$::call_once::hc559b1f3f708a7b0(self=closure-0 @ 0x00007fffffffae68, arg=<unavailable>) at fns.rs:15:9
    frame #2: 0x000055555557f300 foo`_$LT$futures_util..future..future..map..Map$LT$Fut$C$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hebf5b295fcc0837f(self=(pointer = 0x0000555555700e00), cx=0x00007fffffffcf50) at map.rs:57:73
    frame #3: 0x00005555555836ac foo`_$LT$futures_util..future..future..Map$LT$Fut$C$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h482f253651b968e6(self=Pin<&mut futures_util::future::future::Map<tokio::time::driver::sleep::Sleep, closure-0>> @ 0x00007fffffffb268, cx=0x00007fffffffcf50)
at lib.rs:102:13
    frame #4: 0x000055555557995a foo`_$LT$futures_util..future..future..flatten..Flatten$LT$Fut$C$$LT$Fut$u20$as$u20$core..future..future..Future$GT$..Output$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hd62d2a2417c0f2ea(self=(pointer = 0x0000555555700d80), cx=0x00007fffffffcf50) at flatten.rs:48:36
    frame #5: 0x00005555555834fc foo`_$LT$futures_util..future..future..Then$LT$Fut1$C$Fut2$C$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hf60f05f9e9d6f307(self=Pin<&mut futures_util::future::future::Then<tokio::time::driver::sleep::Sleep, core::future::ready::Ready<()>, closure-0>> @ 0x00007fffffffc148, cx=0x00007fffffffcf50) at lib.rs:102:13
    frame #6: 0x000055555558474a foo`_$LT$core..pin..Pin$LT$P$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h4dad267b4f10535d(self=Pin<&mut core::pin::Pin<alloc::boxed::Box<Future, alloc::alloc::Global>>> @ 0x00007fffffffc188, cx=0x00007fffffffcf50) at future.rs:119:9
    frame #7: 0x000055555557a693 foo`_$LT$futures_util..future..maybe_done..MaybeDone$LT$Fut$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hdb6db40c2b3f2f1b(self=(pointer = 0x00005555557011b0), cx=0x00007fffffffcf50) at maybe_done.rs:95:38
    frame #8: 0x0000555555581254 foo`_$LT$futures_util..future..join_all..JoinAll$LT$F$GT$$u20$as$u20$core..future..future..Future$GT$::poll::ha2472a9a54f0e504(self=Pin<&mut futures_util::future::join_all::JoinAll<core::pin::Pin<alloc::boxed::Box<Future, alloc::alloc::Global>>>> @ 0x00007fffffffc388, cx=0x00007fffffffcf50) at join_all.rs:101:16
    frame #9: 0x0000555555584095 foo`foo::main::_$u7b$$u7b$closure$u7d$$u7d$::h6459086fc041943f((null)=ResumeTy @ 0x00007fffffffcc40) at main.rs:17:5
    frame #10: 0x0000555555580eab foo`_$LT$core..future..from_generator..GenFuture$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h272e2b5e808264a2(self=Pin<&mut core::future::from_generator::GenFuture<generator-0>> @ 0x00007fffffffccf8, cx=0x00007fffffffcf50) at mod.rs:80:19
    frame #11: 0x00005555555805a0 foo`tokio::park::thread::CachedParkThread::block_on::_$u7b$$u7b$closure$u7d$$u7d$::hbfc61d9f747eef7b at thread.rs:263:54
    frame #12: 0x00005555555795cc foo`tokio::coop::with_budget::_$u7b$$u7b$closure$u7d$$u7d$::ha229cfa0c1a2e13f(cell=0x00007ffff7c06712) at coop.rs:106:9
    frame #13: 0x00005555555773cc foo`std::thread::local::LocalKey$LT$T$GT$::try_with::h9a2f70c5c8e63288(self=0x00005555556e2a48, f=<unavailable>) at local.rs:272:16
    frame #14: 0x0000555555576ead foo`std::thread::local::LocalKey$LT$T$GT$::with::h12eeed0906b94d09(self=0x00005555556e2a48, f=<unavailable>) at local.rs:248:9
    frame #15: 0x000055555557fea6 foo`tokio::park::thread::CachedParkThread::block_on::h33b270af584419f1 [inlined] tokio::coop::with_budget::hcd477734d4970ed5(budget=(__0 = core::option::Option<u8> @ 0x00007fffffffd040), f=closure-0 @ 0x00007fffffffd048) at coop.rs:99:5
    frame #16: 0x000055555557fe73 foo`tokio::park::thread::CachedParkThread::block_on::h33b270af584419f1 [inlined] tokio::coop::budget::h410dced2a7df3ec8(f=closure-0 @ 0x00007fffffffd008) at coop.rs:76
    frame #17: 0x000055555557fe0c foo`tokio::park::thread::CachedParkThread::block_on::h33b270af584419f1(self=0x00007fffffffd078, f=<unavailable>) at thread.rs:263
    frame #18: 0x0000555555578f76 foo`tokio::runtime::enter::Enter::block_on::h4a9c2602e7b82840(self=0x00007fffffffd0f8, f=<unavailable>) at enter.rs:151:13
    frame #19: 0x000055555558482b foo`tokio::runtime::thread_pool::ThreadPool::block_on::h6b211ce19db8989d(self=0x00007fffffffd280, future=(__0 = foo::main::generator-0 @ 0x00007fffffffd200)) at mod.rs:71:9
    frame #20: 0x0000555555583324 foo`tokio::runtime::Runtime::block_on::h5f6badd2dffadf55(self=0x00007fffffffd278, future=(__0 = foo::main::generator-0 @ 0x00007fffffffd968)) at mod.rs:452:43
    frame #21: 0x0000555555579052 foo`foo::main::h3106d444f509ad81 at main.rs:5:1
    frame #22: 0x000055555557b69b foo`core::ops::function::FnOnce::call_once::hba86afc3f8197561((null)=(foo`foo::main::h3106d444f509ad81 at main.rs:6), (null)=<unavailable>) at function.rs:227:5
    frame #23: 0x0000555555580efe foo`std::sys_common::backtrace::__rust_begin_short_backtrace::h856d648367895391(f=(foo`foo::main::h3106d444f509ad81 at main.rs:6)) at backtrace.rs:125:18
    frame #24: 0x00005555555842f1 foo`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h24c58cd1e112136f at rt.rs:66:18
    frame #25: 0x0000555555670aca foo`std::rt::lang_start_internal::h965c28c9ce06ee73 [inlined] core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::hbcc915e668c7ca11 at function.rs:259:13
    frame #26: 0x0000555555670ac3 foo`std::rt::lang_start_internal::h965c28c9ce06ee73 [inlined] std::panicking::try::do_call::h6b0f430d48122ddf at panicking.rs:379
    frame #27: 0x0000555555670ac3 foo`std::rt::lang_start_internal::h965c28c9ce06ee73 [inlined] std::panicking::try::h6ba420e2e21b5afa at panicking.rs:343
    frame #28: 0x0000555555670ac3 foo`std::rt::lang_start_internal::h965c28c9ce06ee73 [inlined] std::panic::catch_unwind::h8366719d1f615eee at panic.rs:431
    frame #29: 0x0000555555670ac3 foo`std::rt::lang_start_internal::h965c28c9ce06ee73 at rt.rs:51
    frame #30: 0x00005555555842d0 foo`std::rt::lang_start::ha8694bc6fe5182cd(main=(foo`foo::main::h3106d444f509ad81 at main.rs:6), argc=1, argv=0x00007fffffffdc88) at rt.rs:65:5
    frame #31: 0x00005555555790ec foo`main + 28
    frame #32: 0x00007ffff7c2f09b libc.so.6`__libc_start_main(main=(foo`main), argc=1, argv=0x00007fffffffdc88, init=<unavailable>, fini=<unavailable>, rtld_fini=<unavailable>, stack_end=0x00007fffffffdc78) at libc-start.c:308:16
```
</details>

### **Doesn't Rust have backtrace trimming support?**
Yes, this **is** the reduced backtrace. You don't even want to know what the [full one](https://gist.github.com/eminence/0b3e697b7c4e686451ff0d37c169c89d) looks like. Don't click it. Don't!
    
[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
