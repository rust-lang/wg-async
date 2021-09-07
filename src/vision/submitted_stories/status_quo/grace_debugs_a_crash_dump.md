# 😱 Status quo stories: Grace debugs a crash dump

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[Grace] is an engineer working on a hosted [DistriData] service, similar to [Azure Cosmos DB] or [Amazon DynamoDB]. Sometimes one of the DistriData nodes panics. There is a monitor system that catches these panics, saves a crash dump, and restarts the service. The crash dumps can be analyzed after the fact to try to debug the issue.

After a recent version push, there has been an increase in the number of panics. This represents a threat to the service's overall reliability, so Grace has been tasked to investigate. Grace is known as one of the team's best debuggers, with years of experience diagnosing tricky issues from crash dumps. With C and C++ code, Grace can see raw hex dumps and decode the underlying data structures in her head.

Despite this, Grace is relatively new to Rust and is still developing this intuition for Rust code. To get started, Grace hopes her debugger will help her get started. What executors are running? What tasks are running? What state were they in?

She starts by looking at a backtrace:

```
[dbg] bt
  0 0x407e5a7cae11 • syscalls.inc:675
  1 _zx_port_wait(…) • syscalls.inc:675
      handle = 1569127495
      deadline = 9223372036854775807
      packet = (*)0x1aea3201dc8

  2 distridata_zircon::port::Port::wait(…) • src/port.rs:323
      self = (*)0x3f0f481a580 ➔ Port(Handle(…))
      deadline = Time(9223372036854775807)

  3 λ(…) • default/../../src/lib/distridata-async/src/executor.rs:397
      timer_heap = (*)0x2116e3c3a00 ➔ BinaryHeap<distridata_async::executor::…>[]

  4 λ(…) • default/../../src/lib/distridata-async/src/executor.rs:316
      e = (*)0x2116e3c39f0 ➔ RefCell<core::option::Option<(all…>{borrow: Cell<isize>{…}, value: UnsafeCell<core::option::Option<(all…>{…}}

  5 std::thread::local::LocalKey<…>::try_with<…>(…) • thread/local.rs:262
      self = (*)0x3816da0c9b0 ➔ LocalKey<core::cell::RefCell<core:…>{inner: &distridata_async::executor::EXECUTOR::__getit}
      f = $(closure-0)($(closure-0)((*)0x1aea32022a0))

  6 std::thread::local::LocalKey<…>::with<…>(…) + 0x27 (no line info)
      self = (*)0x3816da0c9b0 ➔ LocalKey<core::cell::RefCell<core:…>{inner: &distridata_async::executor::EXECUTOR::__getit}
      f = $(closure-0)($(closure-0)((*)0x1aea32022a0))

  7 distridata_async::executor::with_local_timer_heap<…>(…) + 0x2a (no line info)
      f = $(closure-0)((*)0x1aea32022a0 ➔ (*)0x1aea3202758)

▶ 8 distridata_async::executor::Executor::run_singlethreaded<…>(…) • default/../../src/lib/distridata-async/src/executor.rs:393
      self = (*)0x1aea3202758 ➔ Executor{inner: (*)0x3f0f481a380, next_packet: …}
      main_future = GenFuture<generator-0>(Unresumed)

  9 distridata_pkg_testing_lib_test::serve::tests::test_serve_empty() • serve.rs:345

  10 λ(…) • serve.rs:345
      (*)0x1aea3202b80 ➔ $(closure-0)

  11 core::ops::function::FnOnce::call_once<…>(…) • function.rs:232
      $(closure-0)
      <Value has no data.>
```

The backtrace shows a lot of detail about the executor, but not of this is really relevant to Grace's code. She will have to inspect the executor manually in order to find the information she needs. Frame 8 looks promising, so the finds the local variables there and sees one called `main_future`. Inspecting the code, she sees this has a `pointer` field, which might tell her something about the task that's running. She takes a look:

```
[dbg] print -t --max-array=2 main_future.pointer
(std::future::GenFuture<generator-0>*) 0x1aea32022a8 ➔ std::future::GenFuture<generator-0>(
  (distridata_pkg_testing_lib_test::serve::tests::test_serve_empty::func::generator-0) distridata_pkg_testing_lib_test::serve::tests::test_serve_empty::func::$(generator-0)::Suspend6{
    packages: alloc::vec::Vec<distridata_pkg_testing_lib_test::repo::PackageEntry>[]
    bytes: alloc::vec::Vec<u8>[123, 34, ]
    (alloc::string::String) url: "ht"...
    also_bytes: alloc::vec::Vec<u8>[123, 34, ]
    pinned: std::future::GenFuture<generator-0>(
      distridata_pkg_testing_lib_test::serve::get::$(generator-0){
        (alloc::string::String) __0: "ht"...
      }
    )
  }
)
```

This has some more information, but it is still not as helpful as Grace was hoping for.

Grace quickly realizes her tools are not going to give her as much help as she'd like. She does manage to find the executor in memory, so she starts reading the code to understand how tasks are laid out in memory, etc. Even once she finds the list of tasks, she can only see the opaque contents of the closure. It is hard even to track these back to a line number, or to what operating system resource the task is blocked on (IOCP handle, io_uring event, etc.).

She realizes this is going to take a lot longer than it would if this were a C++ service, so she gets up to grab another cup and coffee and then settles in for a long debugging session.

[Azure Cosmos DB]: https://azure.microsoft.com/en-us/services/cosmos-db/
[Amazon DynamoDB]: https://aws.amazon.com/dynamodb/

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

While much of the focus for async debugger is on the live debugging case, where a developer is running a build on their own machine, there will also be a need to debug crashes after the fact. For example, an application running on a consumer's device may upload crash dumps automatically, or a service running in a cloud environment may also collect a crash dump before restarting the server. Often the bugs that show up in these scenarios are hard to reproduce on a developer's machine, so the more information it's possible to glean from a crash dump, the better.

Even just an accurate and complete stack trace can help a lot. Many error reporting systems cluster crashes by stack trace, so having an incomplete stack trace can lead to unrelated crashes being grouped together.

### **What are the sources for this story?**

This is inspired by requests from internal teams looking to expand the use of Rust in services they develop.

This story also includes some input from Fuchsia developers, including a bug they have about getting [async backtraces in the debugger](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=49435).

### **Why did you choose Grace to tell this story?**

Grace is part of a team of experienced systems hackers who have recently migrated to Rust because of its safety guarantees while still maintaining high performance. Grace is used to debugging these kinds of issues in a certain way, and would like to transfer these skills to Rust.

### **How would this story have played out differently for the other characters?**

This could happen to [Alan] or [Barbara] as well. In Alan's case, he may be used to C# and Visual Studio's `async` debugger tools. He'd probably miss those tools and wish support for something similar could be added to his IDE.

In [Niklaus]'s case, he would probably need to ask one of his more experienced team mates to help him debug the issue. With better tooling, he'd probably be able to get further on his own.

### **What other stories are related to this one?**

* In [Alan tries to debug a hang](alan_tries_to_debug_a_hang.md), Alan misses some of the strong debugging tools he's used in the past. Grace would enjoy using those same tools if they worked on crash dumps in addition to live processes.
* In [Barbara wants async insights][async-insights], Barbara wants to use a debugger to inspect a running process. Most of the insights Barbara is looking for in that situation would also be relevant to Grace in a post-hoc debugging situation.
* In [Barbara gets burned by select](barbara_gets_burned_by_select.md), Barbara has trouble debugging an issue where not all database updates are processed. Similar debugging tools would help both Barbara and Grace.
* In [Grace deploys her service and hits obstacles](grace_deploys_her_service.md), Grace finds a tricky issue in production that only appears at high load. Because she doesn't have the right tooling to debug, she resorts to ad hoc logging, combined with some operating system tools. She could have benefited from the ability to inspect what is blocking tasks in an executor as well.
* In [Grace waits for `gdb next`](grace_waits_for_gdb_next.md), Grace finds that her usual debugging techniques do not work well with async programs.

* This is tangentially related to the story [Alan iteratively regresses performance](alan_iteratively_regresses.md), because there Alan was used to applying existing native tools to Rust, even though there is sometimes an impedence mismatch. The mismatch is likely to be even more challenging for async debugging, since this scenario is already not well supported in a lot of existing tools.


[character]: ../../characters.md
[status quo stories]: ../status_quo.md
[Alan]: ../../characters/alan.md
[Grace]: ../../characters/grace.md
[Niklaus]: ../../characters/niklaus.md
[Barbara]: ../../characters/barbara.md
[htvsq]: ../status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
[DistriData]: ../../projects/DistriData.md
[async-insights]: barbara_wants_async_insights.md