# Async Stack Trace Design Notes

This page has notes on the state of [async stack traces], highlights specific issues with current stack traces, and suggests changes to improve these issues.

The two main suggestions are:

1. Allow async runtimes to control where the short backtrace cutoff happens
2. Expand the options allowed in `RUST_BACKTRACE` to support including/excluding frames from certain crates or module paths in the backtrace.

[async stack traces]: ../vision/roadmap/polish/stacktraces.md

## The Current State of Things

The current state of stack traces was captured pretty well in the story [Barbara Trims a Stack Trace][barbara-trims-stack-trace]. We've recreated a similar example to the one in the story here. We'll look at several executors.

[barbara-trims-stack-trace]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_trims_a_stacktrace.html

### Tokio

<details><summary>Short Backtrace</summary>

```
thread 'main' panicked at 'explicit panic', C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10:5
stack backtrace:
   0: std::panicking::begin_panic_handler
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:517
   1: core::panicking::panic_fmt
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:101
   2: core::panicking::panic
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:50
   3: common::baz::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10
   4: core::future::from_generator::impl$1::poll<common::baz::generator$0>
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
   5: common::bar::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:6
   6: core::future::from_generator::impl$1::poll<common::bar::generator$0>
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
   7: common::foo::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:2
   8: core::future::from_generator::impl$1::poll<common::foo::generator$0>
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
   9: async_tokio::main::generator$0
            at .\src\main.rs:4
10: core::future::from_generator::impl$1::poll<async_tokio::main::generator$0>
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
11: tokio::park::thread::impl$5::block_on::closure$0<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\park\thread.rs:263
12: tokio::coop::with_budget::closure$0<enum$<core::task::poll::Poll<tuple$<> > >,tokio::park::thread::impl$5::block_on::closure$0>
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\coop.rs:106
13: std::thread::local::LocalKey<core::cell::Cell<tokio::coop::Budget> >::try_with<core::cell::Cell<tokio::coop::Budget>,tokio::coop::with_budget::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
14: std::thread::local::LocalKey<core::cell::Cell<tokio::coop::Budget> >::with<core::cell::Cell<tokio::coop::Budget>,tokio::coop::with_budget::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
15: tokio::coop::with_budget
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\coop.rs:99
16: tokio::coop::budget
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\coop.rs:76
17: tokio::park::thread::CachedParkThread::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\park\thread.rs:263
18: tokio::runtime::enter::Enter::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\runtime\enter.rs:151
19: tokio::runtime::thread_pool::ThreadPool::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\runtime\thread_pool\mod.rs:77
20: tokio::runtime::Runtime::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\runtime\mod.rs:463
21: async_tokio::main
            at .\src\main.rs:4
22: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

</details>

<details><summary>Full Backtrace</summary>

```
thread 'main' panicked at 'explicit panic', C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10:5
stack backtrace:
   0:     0x7ff7986d431e - std::backtrace_rs::backtrace::dbghelp::trace
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\..\..\backtrace\src\backtrace\dbghelp.rs:98
   1:     0x7ff7986d431e - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff7986d431e - std::sys_common::backtrace::_print_fmt
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:67
   3:     0x7ff7986d431e - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:46
   4:     0x7ff7986e4a8a - core::fmt::write
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\fmt\mod.rs:1150
   5:     0x7ff7986d22a8 - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\io\mod.rs:1667
   6:     0x7ff7986d6c96 - std::sys_common::backtrace::_print
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:49
   7:     0x7ff7986d6c96 - std::sys_common::backtrace::print
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:36
   8:     0x7ff7986d6c96 - std::panicking::default_hook::closure$1
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:210
   9:     0x7ff7986d6784 - std::panicking::default_hook
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:227
  10:     0x7ff7986d72f5 - std::panicking::rust_panic_with_hook
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:624
  11:     0x7ff7986d6eaf - std::panicking::begin_panic_handler::closure$0
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:519
  12:     0x7ff7986d4c67 - std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure$0,never$>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:141
  13:     0x7ff7986d6e39 - std::panicking::begin_panic_handler
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:517
  14:     0x7ff7986ea170 - core::panicking::panic_fmt
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:101
  15:     0x7ff7986ea0bc - core::panicking::panic
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:50
  16:     0x7ff798631d9f - common::baz::generator$0
                               at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10
  17:     0x7ff798632139 - core::future::from_generator::impl$1::poll<common::baz::generator$0>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  18:     0x7ff798631ccb - common::bar::generator$0
                               at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:6
  19:     0x7ff7986320a9 - core::future::from_generator::impl$1::poll<common::bar::generator$0>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  20:     0x7ff798631ef2 - common::foo::generator$0
                               at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:2
  21:     0x7ff798632019 - core::future::from_generator::impl$1::poll<common::foo::generator$0>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  22:     0x7ff798635718 - async_tokio::main::generator$0
                               at C:\Users\ericholk\repo\backtrace-examples\async-tokio\src\main.rs:4
  23:     0x7ff7986321c9 - core::future::from_generator::impl$1::poll<async_tokio::main::generator$0>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  24:     0x7ff798631b9a - tokio::park::thread::impl$5::block_on::closure$0<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\park\thread.rs:263
  25:     0x7ff798632df9 - tokio::coop::with_budget::closure$0<enum$<core::task::poll::Poll<tuple$<> > >,tokio::park::thread::impl$5::block_on::closure$0>
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\coop.rs:106
  26:     0x7ff798632652 - std::thread::local::LocalKey<core::cell::Cell<tokio::coop::Budget> >::try_with<core::cell::Cell<tokio::coop::Budget>,tokio::coop::with_budget::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
  27:     0x7ff79863251d - std::thread::local::LocalKey<core::cell::Cell<tokio::coop::Budget> >::with<core::cell::Cell<tokio::coop::Budget>,tokio::coop::with_budget::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
  28:     0x7ff79863165c - tokio::coop::with_budget
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\coop.rs:99
  29:     0x7ff79863165c - tokio::coop::budget
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\coop.rs:76
  30:     0x7ff79863165c - tokio::park::thread::CachedParkThread::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\park\thread.rs:263
  31:     0x7ff7986358b4 - tokio::runtime::enter::Enter::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\runtime\enter.rs:151
  32:     0x7ff798631046 - tokio::runtime::thread_pool::ThreadPool::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\runtime\thread_pool\mod.rs:77
  33:     0x7ff798632b68 - tokio::runtime::Runtime::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\runtime\mod.rs:463
  34:     0x7ff798632ca3 - async_tokio::main
                               at C:\Users\ericholk\repo\backtrace-examples\async-tokio\src\main.rs:4
  35:     0x7ff7986332ab - core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:227
  36:     0x7ff7986311fb - std::sys_common::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\sys_common\backtrace.rs:125
  37:     0x7ff798631121 - std::rt::lang_start::closure$0<tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\rt.rs:63
  38:     0x7ff7986d7886 - core::ops::function::impls::impl$2::call_once
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:259
  39:     0x7ff7986d7886 - std::panicking::try::do_call
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:403
  40:     0x7ff7986d7886 - std::panicking::try
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:367
  41:     0x7ff7986d7886 - std::panic::catch_unwind
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panic.rs:129
  42:     0x7ff7986d7886 - std::rt::lang_start_internal::closure$2
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\rt.rs:45
  43:     0x7ff7986d7886 - std::panicking::try::do_call
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:403
  44:     0x7ff7986d7886 - std::panicking::try
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:367
  45:     0x7ff7986d7886 - std::panic::catch_unwind
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panic.rs:129
  46:     0x7ff7986d7886 - std::rt::lang_start_internal
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\rt.rs:45
  47:     0x7ff7986310ef - std::rt::lang_start<tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\rt.rs:62
  48:     0x7ff798632d46 - main
  49:     0x7ff7986e8dd0 - invoke_main
                               at d:\a01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  50:     0x7ff7986e8dd0 - __scrt_common_main_seh
                               at d:\a01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  51:     0x7ffbe0a26ab0 - BaseThreadInitThunk
  52:     0x7ffbe1771dbb - RtlUserThreadStart
```

</details>

### Async-std

<details><summary>Short Backtrace</summary>

```
thread 'main' panicked at 'explicit panic', C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10:5
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:517
   1: core::panicking::panic_fmt
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:101
   2: core::panicking::panic
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:50
   3: common::baz::generator$0
             at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10
   4: core::future::from_generator::impl$1::poll<common::baz::generator$0>
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
   5: common::bar::generator$0
             at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:6
   6: core::future::from_generator::impl$1::poll<common::bar::generator$0>
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
   7: common::foo::generator$0
             at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:2
   8: core::future::from_generator::impl$1::poll<common::foo::generator$0>
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
   9: async_std::task::builder::impl$1::poll::closure$0<core::future::from_generator::GenFuture<common::foo::generator$0> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:199
  10: async_std::task::task_locals_wrapper::impl$0::set_current::closure$0<async_std::task::builder::impl$1::poll::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\task_locals_wrapper.rs:60
  11: std::thread::local::LocalKey<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> > >::try_with<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> >,async_std::task::task_locals_wrapper::im
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
  12: std::thread::local::LocalKey<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> > >::with<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> >,async_std::task::task_locals_wrapper::impl$0
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
  13: async_std::task::task_locals_wrapper::TaskLocalsWrapper::set_current<async_std::task::builder::impl$1::poll::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\task_locals_wrapper.rs:55
  14: async_std::task::builder::impl$1::poll<core::future::from_generator::GenFuture<common::foo::generator$0> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:197
  15: futures_lite::future::impl$12::poll<tuple$<>,async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> >,core::future::from_generator::GenFuture<async_executor::impl$4::run::generator$0::generator$0> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\futures-lite-1.12.0\src\future.rs:526
  16: async_executor::impl$4::run::generator$0<tuple$<>,async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> > >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-executor-1.4.1\src\lib.rs:242
  17: core::future::from_generator::impl$1::poll<async_executor::impl$4::run::generator$0>
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  18: async_executor::impl$9::run::generator$0<tuple$<>,async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> > >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-executor-1.4.1\src\lib.rs:447
  19: core::future::from_generator::impl$1::poll<async_executor::impl$9::run::generator$0>
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  20: async_io::driver::block_on<tuple$<>,core::future::from_generator::GenFuture<async_executor::impl$9::run::generator$0> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-io-1.6.0\src\driver.rs:142
  21: async_global_executor::reactor::block_on::closure$0<core::future::from_generator::GenFuture<async_executor::impl$9::run::generator$0>,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-global-executor-2.0.2\src\reactor.rs:3
  22: async_global_executor::reactor::block_on<core::future::from_generator::GenFuture<async_executor::impl$9::run::generator$0>,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-global-executor-2.0.2\src\reactor.rs:12
  23: async_global_executor::executor::block_on::closure$0<async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> >,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-global-executor-2.0.2\src\executor.rs:26
  24: std::thread::local::LocalKey<async_executor::LocalExecutor>::try_with<async_executor::LocalExecutor,async_global_executor::executor::block_on::closure$0,tuple$<> >
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
  25: std::thread::local::LocalKey<async_executor::LocalExecutor>::with<async_executor::LocalExecutor,async_global_executor::executor::block_on::closure$0,tuple$<> >
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
  26: async_global_executor::executor::block_on<async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> >,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-global-executor-2.0.2\src\executor.rs:26
  27: async_std::task::builder::impl$0::blocking::closure$0::closure$0<core::future::from_generator::GenFuture<common::foo::generator$0>,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:171
  28: async_std::task::task_locals_wrapper::impl$0::set_current::closure$0<async_std::task::builder::impl$0::blocking::closure$0::closure$0,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\task_locals_wrapper.rs:60
  29: std::thread::local::LocalKey<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> > >::try_with<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> >,async_std::task::task_locals_wrapper::im
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
  30: std::thread::local::LocalKey<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> > >::with<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> >,async_std::task::task_locals_wrapper::impl$0
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
  31: async_std::task::task_locals_wrapper::TaskLocalsWrapper::set_current<async_std::task::builder::impl$0::blocking::closure$0::closure$0,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\task_locals_wrapper.rs:55
  32: async_std::task::builder::impl$0::blocking::closure$0<core::future::from_generator::GenFuture<common::foo::generator$0>,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:168
  33: std::thread::local::LocalKey<core::cell::Cell<usize> >::try_with<core::cell::Cell<usize>,async_std::task::builder::impl$0::blocking::closure$0,tuple$<> >
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
  34: std::thread::local::LocalKey<core::cell::Cell<usize> >::with<core::cell::Cell<usize>,async_std::task::builder::impl$0::blocking::closure$0,tuple$<> >
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
  35: async_std::task::builder::Builder::blocking<core::future::from_generator::GenFuture<common::foo::generator$0>,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:161
  36: async_std::task::block_on::block_on<core::future::from_generator::GenFuture<common::foo::generator$0>,tuple$<> >
             at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\block_on.rs:33
  37: async_std::main
             at .\src\main.rs:2
  38: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\debug\async-std.exe` (exit code: 101)
```

</details>

<details><summary>Full Backtrace</summary>

```
thread 'main' panicked at 'explicit panic', C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10:5
stack backtrace:
   0:     0x7ff6d4162fee - std::backtrace_rs::backtrace::dbghelp::trace
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\..\..\backtrace\src\backtrace\dbghelp.rs:98
   1:     0x7ff6d4162fee - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff6d4162fee - std::sys_common::backtrace::_print_fmt
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:67
   3:     0x7ff6d4162fee - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:46
   4:     0x7ff6d4172dba - core::fmt::write
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\fmt\mod.rs:1150
   5:     0x7ff6d4160fa8 - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\io\mod.rs:1667
   6:     0x7ff6d4165466 - std::sys_common::backtrace::_print
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:49
   7:     0x7ff6d4165466 - std::sys_common::backtrace::print
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:36
   8:     0x7ff6d4165466 - std::panicking::default_hook::closure$1
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:210
   9:     0x7ff6d4164f54 - std::panicking::default_hook
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:227
  10:     0x7ff6d4165ac5 - std::panicking::rust_panic_with_hook
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:624
  11:     0x7ff6d416567f - std::panicking::begin_panic_handler::closure$0
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:519
  12:     0x7ff6d4163937 - std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure$0,never$>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:141
  13:     0x7ff6d4165609 - std::panicking::begin_panic_handler
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:517
  14:     0x7ff6d417c2d0 - core::panicking::panic_fmt
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:101
  15:     0x7ff6d417c21c - core::panicking::panic
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:50
  16:     0x7ff6d40c47df - common::baz::generator$0
                               at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10
  17:     0x7ff6d40c7749 - core::future::from_generator::impl$1::poll<common::baz::generator$0>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  18:     0x7ff6d40c470b - common::bar::generator$0
                               at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:6
  19:     0x7ff6d40c7869 - core::future::from_generator::impl$1::poll<common::bar::generator$0>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  20:     0x7ff6d40c4932 - common::foo::generator$0
                               at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:2
  21:     0x7ff6d40c77d9 - core::future::from_generator::impl$1::poll<common::foo::generator$0>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  22:     0x7ff6d40c1403 - async_std::task::builder::impl$1::poll::closure$0<core::future::from_generator::GenFuture<common::foo::generator$0> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:199
  23:     0x7ff6d40c2f38 - async_std::task::task_locals_wrapper::impl$0::set_current::closure$0<async_std::task::builder::impl$1::poll::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\task_locals_wrapper.rs:60
  24:     0x7ff6d40c2212 - std::thread::local::LocalKey<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> > >::try_with<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> >,async_std::task::task_locals_wrapper::im
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
  25:     0x7ff6d40c1cbd - std::thread::local::LocalKey<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> > >::with<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> >,async_std::task::task_locals_wrapper::impl$0
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
  26:     0x7ff6d40c2e88 - async_std::task::task_locals_wrapper::TaskLocalsWrapper::set_current<async_std::task::builder::impl$1::poll::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\task_locals_wrapper.rs:55
  27:     0x7ff6d40c13a5 - async_std::task::builder::impl$1::poll<core::future::from_generator::GenFuture<common::foo::generator$0> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:197
  28:     0x7ff6d40c5aeb - futures_lite::future::impl$12::poll<tuple$<>,async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> >,core::future::from_generator::GenFuture<async_executor::impl$4::run::generator$0::generator$0> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\futures-lite-1.12.0\src\future.rs:526
  29:     0x7ff6d40c3928 - async_executor::impl$4::run::generator$0<tuple$<>,async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> > >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-executor-1.4.1\src\lib.rs:242
  30:     0x7ff6d40c7629 - core::future::from_generator::impl$1::poll<async_executor::impl$4::run::generator$0>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  31:     0x7ff6d40c3543 - async_executor::impl$9::run::generator$0<tuple$<>,async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> > >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-executor-1.4.1\src\lib.rs:447
  32:     0x7ff6d40c7599 - core::future::from_generator::impl$1::poll<async_executor::impl$9::run::generator$0>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
  33:     0x7ff6d40c623c - async_io::driver::block_on<tuple$<>,core::future::from_generator::GenFuture<async_executor::impl$9::run::generator$0> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-io-1.6.0\src\driver.rs:142
  34:     0x7ff6d40c3e84 - async_global_executor::reactor::block_on::closure$0<core::future::from_generator::GenFuture<async_executor::impl$9::run::generator$0>,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-global-executor-2.0.2\src\reactor.rs:3
  35:     0x7ff6d40c3e4f - async_global_executor::reactor::block_on<core::future::from_generator::GenFuture<async_executor::impl$9::run::generator$0>,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-global-executor-2.0.2\src\reactor.rs:12
  36:     0x7ff6d40c31ee - async_global_executor::executor::block_on::closure$0<async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> >,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-global-executor-2.0.2\src\executor.rs:26
  37:     0x7ff6d40c231a - std::thread::local::LocalKey<async_executor::LocalExecutor>::try_with<async_executor::LocalExecutor,async_global_executor::executor::block_on::closure$0,tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
  38:     0x7ff6d40c1d5d - std::thread::local::LocalKey<async_executor::LocalExecutor>::with<async_executor::LocalExecutor,async_global_executor::executor::block_on::closure$0,tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
  39:     0x7ff6d40c3198 - async_global_executor::executor::block_on<async_std::task::builder::SupportTaskLocals<core::future::from_generator::GenFuture<common::foo::generator$0> >,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-global-executor-2.0.2\src\executor.rs:26
  40:     0x7ff6d40c1b09 - async_std::task::builder::impl$0::blocking::closure$0::closure$0<core::future::from_generator::GenFuture<common::foo::generator$0>,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:171
  41:     0x7ff6d40c3074 - async_std::task::task_locals_wrapper::impl$0::set_current::closure$0<async_std::task::builder::impl$0::blocking::closure$0::closure$0,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\task_locals_wrapper.rs:60
  42:     0x7ff6d40c210a - std::thread::local::LocalKey<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> > >::try_with<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> >,async_std::task::task_locals_wrapper::im
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
  43:     0x7ff6d40c1c63 - std::thread::local::LocalKey<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> > >::with<core::cell::Cell<ptr_const$<async_std::task::task_locals_wrapper::TaskLocalsWrapper> >,async_std::task::task_locals_wrapper::impl$0
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
  44:     0x7ff6d40c2e48 - async_std::task::task_locals_wrapper::TaskLocalsWrapper::set_current<async_std::task::builder::impl$0::blocking::closure$0::closure$0,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\task_locals_wrapper.rs:55
  45:     0x7ff6d40c1a28 - async_std::task::builder::impl$0::blocking::closure$0<core::future::from_generator::GenFuture<common::foo::generator$0>,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:168
  46:     0x7ff6d40c1fea - std::thread::local::LocalKey<core::cell::Cell<usize> >::try_with<core::cell::Cell<usize>,async_std::task::builder::impl$0::blocking::closure$0,tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
  47:     0x7ff6d40c1bfd - std::thread::local::LocalKey<core::cell::Cell<usize> >::with<core::cell::Cell<usize>,async_std::task::builder::impl$0::blocking::closure$0,tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
  48:     0x7ff6d40c17a3 - async_std::task::builder::Builder::blocking<core::future::from_generator::GenFuture<common::foo::generator$0>,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\builder.rs:161
  49:     0x7ff6d40c326a - async_std::task::block_on::block_on<core::future::from_generator::GenFuture<common::foo::generator$0>,tuple$<> >
                               at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\async-std-1.10.0\src\task\block_on.rs:33
  50:     0x7ff6d40c12ae - async_std::main
                               at C:\Users\ericholk\repo\backtrace-examples\async-std\src\main.rs:2
  51:     0x7ff6d40c4b5b - core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:227
  52:     0x7ff6d40c736b - std::sys_common::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\sys_common\backtrace.rs:125
  53:     0x7ff6d40c45f1 - std::rt::lang_start::closure$0<tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\rt.rs:63
  54:     0x7ff6d4165f16 - core::ops::function::impls::impl$2::call_once
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:259
  55:     0x7ff6d4165f16 - std::panicking::try::do_call
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:403
  56:     0x7ff6d4165f16 - std::panicking::try
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:367
  57:     0x7ff6d4165f16 - std::panic::catch_unwind
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panic.rs:129
  58:     0x7ff6d4165f16 - std::rt::lang_start_internal::closure$2
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\rt.rs:45
  59:     0x7ff6d4165f16 - std::panicking::try::do_call
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:403
  60:     0x7ff6d4165f16 - std::panicking::try
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:367
  61:     0x7ff6d4165f16 - std::panic::catch_unwind
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panic.rs:129
  62:     0x7ff6d4165f16 - std::rt::lang_start_internal
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\rt.rs:45
  63:     0x7ff6d40c45bf - std::rt::lang_start<tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\rt.rs:62
  64:     0x7ff6d40c12d6 - main
  65:     0x7ff6d417ad1c - invoke_main
                               at d:\a01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  66:     0x7ff6d417ad1c - __scrt_common_main_seh
                               at d:\a01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  67:     0x7ffbe0a26ab0 - BaseThreadInitThunk
  68:     0x7ffbe1771dbb - RtlUserThreadStart
error: process didn't exit successfully: `target\debug\async-std.exe` (exit code: 101)
```

</details>

## Sync Stack Trace Trimming

Rust supports both a short and full backtraces, controlled by either `RUST_BACKTRACE=1` or `RUST_BACKTRACE=full`. The differents is that short backtraces (`RUST_BACKTRACE=1`) trims away some of the early and late frames.

Below is an example of a short backtrace from a simple program where `main` calls `foo` which calls `bar` which calls `baz` which panics.

```
thread 'main' panicked at 'explicit panic', src\main.rs:14:5
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:517
   1: core::panicking::panic_fmt
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:101
   2: core::panicking::panic
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:50
   3: sync::baz
             at .\src\main.rs:14
   4: sync::bar
             at .\src\main.rs:10
   5: sync::foo
             at .\src\main.rs:6
   6: sync::main
             at .\src\main.rs:2
   7: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Below is the same thing with `RUST_BACKTRACE=full`.

<details><summary>Full Backtrace</summary>

```
thread 'main' panicked at 'explicit panic', src\main.rs:14:5
stack backtrace:
   0:     0x7ff6aef16b6e - std::backtrace_rs::backtrace::dbghelp::trace
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\..\..\backtrace\src\backtrace\dbghelp.rs:98
   1:     0x7ff6aef16b6e - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff6aef16b6e - std::sys_common::backtrace::_print_fmt
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:67
   3:     0x7ff6aef16b6e - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:46
   4:     0x7ff6aef250ea - core::fmt::write
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\fmt\mod.rs:1150
   5:     0x7ff6aef14e18 - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\io\mod.rs:1667
   6:     0x7ff6aef18d86 - std::sys_common::backtrace::_print
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:49
   7:     0x7ff6aef18d86 - std::sys_common::backtrace::print
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:36
   8:     0x7ff6aef18d86 - std::panicking::default_hook::closure$1
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:210
   9:     0x7ff6aef18874 - std::panicking::default_hook
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:227
  10:     0x7ff6aef193e5 - std::panicking::rust_panic_with_hook
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:624
  11:     0x7ff6aef18f9f - std::panicking::begin_panic_handler::closure$0
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:519
  12:     0x7ff6aef174b7 - std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure$0,never$>
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\sys_common\backtrace.rs:141
  13:     0x7ff6aef18f29 - std::panicking::begin_panic_handler
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:517
  14:     0x7ff6aef29940 - core::panicking::panic_fmt
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:101
  15:     0x7ff6aef2988c - core::panicking::panic
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:50
  16:     0x7ff6aef1122c - sync::baz
                               at C:\Users\ericholk\repo\backtrace-examples\sync\src\main.rs:14
  17:     0x7ff6aef11209 - sync::bar
                               at C:\Users\ericholk\repo\backtrace-examples\sync\src\main.rs:10
  18:     0x7ff6aef111f9 - sync::foo
                               at C:\Users\ericholk\repo\backtrace-examples\sync\src\main.rs:6
  19:     0x7ff6aef111e9 - sync::main
                               at C:\Users\ericholk\repo\backtrace-examples\sync\src\main.rs:2
  20:     0x7ff6aef1107b - core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:227
  21:     0x7ff6aef1116b - std::sys_common::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\sys_common\backtrace.rs:125
  22:     0x7ff6aef11101 - std::rt::lang_start::closure$0<tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\rt.rs:63
  23:     0x7ff6aef19836 - core::ops::function::impls::impl$2::call_once
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:259
  24:     0x7ff6aef19836 - std::panicking::try::do_call
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:403
  25:     0x7ff6aef19836 - std::panicking::try
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:367
  26:     0x7ff6aef19836 - std::panic::catch_unwind
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panic.rs:129
  27:     0x7ff6aef19836 - std::rt::lang_start_internal::closure$2
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\rt.rs:45
  28:     0x7ff6aef19836 - std::panicking::try::do_call
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:403
  29:     0x7ff6aef19836 - std::panicking::try
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:367
  30:     0x7ff6aef19836 - std::panic::catch_unwind
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panic.rs:129
  31:     0x7ff6aef19836 - std::rt::lang_start_internal
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\rt.rs:45
  32:     0x7ff6aef110cf - std::rt::lang_start<tuple$<> >
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\rt.rs:62
  33:     0x7ff6aef11246 - main
  34:     0x7ff6aef286e4 - invoke_main
                               at d:\a01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  35:     0x7ff6aef286e4 - __scrt_common_main_seh
                               at d:\a01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  36:     0x7ffbe0a26ab0 - BaseThreadInitThunk
  37:     0x7ffbe1771dbb - RtlUserThreadStart
```

</details>

The full backtrace is much longer and includes many frames related to process startup and panic handling that the programmer is not likely to care about.

The mechanism for trimming back traces is apparent within the full backtrace. There are two functions, `__rust_begin_short_backtrace` and `__rust_end_short_backtrace`. These are set up so that they are never inlined. Then, the short printing routine simply ignores any frames that are not within these two calls.

## Problem Analysis

The main issue with async backtraces now is that they leak a number of implementation details from the async runtime.
To some extent this is true for sync backtraces as well.
For example, in the sync full backtrace there are 15 frames just related panicking after the last user frame (frame 16, `sync::baz`).
In the sync case, it is pretty easy to filter out the startup frames and the panic frames using `__rust_begin_short_backtrace` and `__rust_end_short_backtrace`.
This approach does not work as well for async code as-is because many of the internal details from the runtime are interspersed between user code frames.

For example, let's consider the short tokio backtrace to see what additional frames we'd want to remove.
At the bottom of the stack trace, we have 13 frames related to tokio startup:

```
10: core::future::from_generator::impl$1::poll<async_tokio::main::generator$0>
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
11: tokio::park::thread::impl$5::block_on::closure$0<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\park\thread.rs:263
12: tokio::coop::with_budget::closure$0<enum$<core::task::poll::Poll<tuple$<> > >,tokio::park::thread::impl$5::block_on::closure$0>
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\coop.rs:106
13: std::thread::local::LocalKey<core::cell::Cell<tokio::coop::Budget> >::try_with<core::cell::Cell<tokio::coop::Budget>,tokio::coop::with_budget::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:399
14: std::thread::local::LocalKey<core::cell::Cell<tokio::coop::Budget> >::with<core::cell::Cell<tokio::coop::Budget>,tokio::coop::with_budget::closure$0,enum$<core::task::poll::Poll<tuple$<> > > >
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\std\src\thread\local.rs:375
15: tokio::coop::with_budget
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\coop.rs:99
16: tokio::coop::budget
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\coop.rs:76
17: tokio::park::thread::CachedParkThread::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\park\thread.rs:263
18: tokio::runtime::enter::Enter::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\runtime\enter.rs:151
19: tokio::runtime::thread_pool::ThreadPool::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\runtime\thread_pool\mod.rs:77
20: tokio::runtime::Runtime::block_on<core::future::from_generator::GenFuture<async_tokio::main::generator$0> >
            at C:\Users\ericholk\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-1.13.0\src\runtime\mod.rs:463
21: async_tokio::main
            at .\src\main.rs:4
22: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\ops\function.rs:227
```

If we remove these frames, we have something pretty close to the synchronous short backtrace:

```
   0: std::panicking::begin_panic_handler
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:517
   1: core::panicking::panic_fmt
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:101
   2: core::panicking::panic
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:50
   3: common::baz::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10
   4: core::future::from_generator::impl$1::poll<common::baz::generator$0>
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
   5: common::bar::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:6
   6: core::future::from_generator::impl$1::poll<common::bar::generator$0>
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
   7: common::foo::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:2
   8: core::future::from_generator::impl$1::poll<common::foo::generator$0>
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\library\core\src\future\mod.rs:80
   9: async_tokio::main::generator$0
            at .\src\main.rs:4
```

To achieve parity with the synchronous backtrace, there are two more improvements needed.

The first is that in between each frame in user code, there is a frame with a call to `core::future::from_generator`.

The second is that rather than seeing a call to `common::foo` or similar, we see `common::foo::generator$0`.

These two are relatively minor issues.
Fixing them may not be desirable.

## Proposed Solutions

There are several improvements we could make that would improve the state of backtraces.

### Allow runtimes to trim startup code

This solution probably has the highest impact to effort ratio.
The core of the idea is to move the call to `__rust_begin_short_backtrace` when running under an async executor.
Most likely the way this would happen is to have an alternate startup path that programs can opt in to (how to actually make this work is left as an exercise for the reader).
The alternate path would not call `__rust_begin_short_backtrace` in [`rt.rs`], but would instead expect the program to make sure to call it at the appropriate time.
Then, async runtimes that provide a macro such as `#[tokio::main]` or `#[async_std::main]` would arrange to start up through this alternate path and call `__rust_begin_short_backtrace` shortly before invoking user code.

[`rt.rs`]: https://github.com/rust-lang/rust/blob/master/library/std/src/rt.rs

Note that this approach does not help in cases where users create and launch the runtime manually rather than using a library-provided macro.
It does help in the most common cases, however, and when users take a more manual approach they would also be able to control where the short stack trace starts.

### Allow trimming of internal frames

There are several ways to do this, with varying levels of implementation effort and cost.

As to what this would look like for library authors, ideally we would have something like a `#[backtrace_transparent]` attribute that is applied to a function and indicates that the function should be hidden from backtraces by default.

There are several ways we could do the underlying implementation, which are discussed below.

The conceptually simplest is to allow multiple `__rust_begin_short_backtrace`/`__rust_end_short_backtrace` pairs.
Implementing this could be done almost entirely with changes to [`_print_fmt`].
This approach has some serious drawbacks.
First, it requires a lot of work from library authors to annotate each transition point between user and library code.
Second, `__rust_begin_short_backtrace`/`__rust_end_short_backtrace` are built to defeat inlining to ensure they show up as a frame on the stack.
This is fine when the functions are only called during process startup and once when a panic starts, but it would likely be prohibitively expensive if interspersed between every async function call.

[`_print_fmt`]: https://github.com/rust-lang/rust/blob/master/library/std/src/sys_common/backtrace.rs#L52

It might be possible instead to include more information in the debugging symbols.
For example, we might be able to add a flag indicating that a certain function should be hidden from the backtrace.
To do this, we would first need to make sure existing debugging formats such as DWARF and PDB are able to encode such information.
If there is already support for this, then it's likely debuggers would benefit as well since they would also be able to display trimmed backtraces.
It is worth noting that this solution would not help much for builds without debug symbols.

A third option is to use some kind of name based heuristics.
For example, by default we may want to only show frames in the root crate, although this may be too restrictive for large projects.
Probably the best approach here is to extend the set of options allowed for the `RUST_BACKTRACE` environment variable to make it behave more like `RUST_LOG`.
We could allow options such as `RUST_BACKTRACE="short,exclude=tokio::*"` to hide all frames from the `tokio` crate, or `RUST_BACKTRACE="short,include=my_crate::*"` to only show frames from `my_crate`.

This third option could also be implemented mostly through changes to [`_print_fmt`].
It also gives a great deal of control.
Libraries or projects could provide suggested backtrace filters in their documentation, and programmers can refine these as necessary depending on their needs.

This functionality would be helpful in other contexts as well.
For example, iterator-heavy code tends to have stack traces that interleave user code with internal library implementation details.

One question is whether we want to communicate to the user that frames were omitted.
There are a couple ways we might do this.
For example, we could add a `...` indicating frames are missing:

```
   0: std::panicking::begin_panic_handler
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:517
   1: core::panicking::panic_fmt
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:101
   2: core::panicking::panic
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:50
   3: common::baz::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10
   ...
   4: common::bar::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:6
   ...
   5: common::foo::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:2
   ...
   6: async_tokio::main::generator$0
            at .\src\main.rs:4
```

Or, we could simply omit the frames and have non-consecutive frame numbers.

```
   0: std::panicking::begin_panic_handler
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\std\src\panicking.rs:517
   1: core::panicking::panic_fmt
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:101
   2: core::panicking::panic
            at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35\/library\core\src\panicking.rs:50
   3: common::baz::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:10
   5: common::bar::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:6
   7: common::foo::generator$0
            at C:\Users\ericholk\repo\backtrace-examples\async-common\src\lib.rs:2
   9: async_tokio::main::generator$0
            at .\src\main.rs:4
```

In an interactive context, such as a debugger, the `...` approach is probably best, since it could also provide an option to expand that section and see the frames that are missing.
In non-interactive cases, such as printing a backtrace with some `RUST_BACKTRACE` setting, it may be better to omit the frame numbers that were skipped since that leads to a slightly more compact backtrace.

## References

* [Beautiful tracebacks in Trio v0.7.0 (Python)](https://vorpus.org/blog/beautiful-tracebacks-in-trio-v070/)
* [Faster async functions and Promises (JavaScript)](https://v8.dev/blog/fast-async#improved-developer-experience)
* [Zero-cost async stack traces (JavaScript)](https://docs.google.com/document/d/13Sy_kBIJGP0XT34V1CV3nkWya4TwYx9L3Yv45LdGB6Q/edit#heading=h.e6lcalo0cl47)
* Async stack traces in folly [[1][folly-1]] [[2][folly-2]] [[3][folly-3]] [[4][folly-4]] [[5][folly-5]]

[folly-1]: https://developers.facebook.com/blog/post/2021/09/16/async-stack-traces-folly-Introduction/
[folly-2]: https://developers.facebook.com/blog/post/2021/09/23/async-stack-traces-folly-synchronous-asynchronous-stack-traces/
[folly-3]: https://developers.facebook.com/blog/post/2021/09/30/async-stack-traces-folly-forming-async-stack-individual-frames/
[folly-4]: https://developers.facebook.com/blog/post/2021/10/14/async-stack-traces-c-plus-plus-coroutines-folly-walking-async-stack/
[folly-5]: https://developers.facebook.com/blog/post/2021/10/21/async-stack-traces-folly-improving-debugging-developer-lifecycle/
