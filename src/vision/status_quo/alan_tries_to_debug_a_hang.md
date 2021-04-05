# 😱 Status quo stories: Alan tries to debug a hang

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Alan's startup has officially launched and YouBuy is live for the world to use.
The whole team is very excited especially as this will be their first use of Rust in production! 
Normally, as a .NET shop, they would have written the entire application in C#, but because of the scalability and latency requirements on their inventory service, they decided to write a microservice in Rust utilizing the `async` features they've heard so much about.

The day's excitement soon turns into concern as reports begin coming into support of customers who can't checkout.
After a few cases, a pattern begins to emerge: when a customer tries to buy the last available item, the checkout process hangs forever.

Alan suspects there is an issue with the lock used in the inventory service to prevent multiple people from buying the last available item at the same time.
With this hunch, he builds the latest code and opens this local dev environment to conduct some tests.
Soon enough, Alan has a repro of the bug.

With the broken environment still running, he decides to use a debugger to see if he can confirm his theory.
In the past, Alan has used Visual Studio's debugger to [diagnose a very similar issue in a C# application he wrote](https://devblogs.microsoft.com/visualstudio/how-do-i-debug-async-code-in-visual-studio/#is-there-a-way-to-better-visualize-tasks-and-async-code-flow).
The debugger was able to show him all the async Tasks currently waiting, their call stacks and what resource they were waiting on.

Alan hasn't used a debugger with Rust before, usually a combination of the strict compiler and a bit of manual testing has been enough to fix all the bugs he's previously encountered.
He does a quick Google search to see what debugger he should use and decides to go with `gdb` because it is already installed on his system and sounds like it should work.
Alan also pulls up a blog post that has a helpful cheatsheet of `gdb` commands since he's not familiar with the debugger.

Alan restarts the inventory service under `gdb` and gets to work reproducing the issue.
He reproduces the issue a few times in the hope of making it easier to identify the cause of the problem.
Ready to pinpoint the issue, Alan presses `Ctrl+C` and then types `bt` to get a backtrace:

<details><summary>(gdb) bt</summary>

```ignore
(gdb) bt
#0  0x00007ffff7d5e58a in epoll_wait (epfd=3, events=0x555555711340, maxevents=1024, timeout=49152)
    at ../sysdeps/unix/sysv/linux/epoll_wait.c:30
#1  0x000055555564cf7d in mio::sys::unix::selector::epoll::Selector::select (self=0x7fffffffd008, events=0x7fffffffba40, 
    timeout=...) at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.7.11/src/sys/unix/selector/epoll.rs:68
#2  0x000055555564a82f in mio::poll::Poll::poll (self=0x7fffffffd008, events=0x7fffffffba40, timeout=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.7.11/src/poll.rs:314
#3  0x000055555559ad96 in tokio::io::driver::Driver::turn (self=0x7fffffffce28, max_wait=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/io/driver/mod.rs:162
#4  0x000055555559b8da in <tokio::io::driver::Driver as tokio::park::Park>::park_timeout (self=0x7fffffffce28, duration=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/io/driver/mod.rs:238
#5  0x00005555555e9909 in <tokio::signal::unix::driver::Driver as tokio::park::Park>::park_timeout (self=0x7fffffffce28, 
    duration=...) at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/signal/unix/driver.rs:156
#6  0x00005555555a9229 in <tokio::process::imp::driver::Driver as tokio::park::Park>::park_timeout (self=0x7fffffffce28, 
    duration=...) at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/process/unix/driver.rs:84
#7  0x00005555555a898d in <tokio::park::either::Either<A,B> as tokio::park::Park>::park_timeout (self=0x7fffffffce20, 
    duration=...) at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/park/either.rs:37
#8  0x00005555555ce0b8 in tokio::time::driver::Driver<P>::park_internal (self=0x7fffffffcdf8, limit=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/time/driver/mod.rs:226
#9  0x00005555555cee60 in <tokio::time::driver::Driver<P> as tokio::park::Park>::park (self=0x7fffffffcdf8)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/time/driver/mod.rs:398
#10 0x00005555555a87bb in <tokio::park::either::Either<A,B> as tokio::park::Park>::park (self=0x7fffffffcdf0)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/park/either.rs:30
#11 0x000055555559ce47 in <tokio::runtime::driver::Driver as tokio::park::Park>::park (self=0x7fffffffcdf0)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/runtime/driver.rs:198
#12 0x000055555557a2f7 in tokio::runtime::basic_scheduler::Inner<P>::block_on::{{closure}} (scheduler=0x7fffffffcdb8, 
    context=0x7fffffffcaf0)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/runtime/basic_scheduler.rs:224
#13 0x000055555557b1b4 in tokio::runtime::basic_scheduler::enter::{{closure}} ()
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/runtime/basic_scheduler.rs:279
#14 0x000055555558174a in tokio::macros::scoped_tls::ScopedKey<T>::set (
    self=0x555555701af8 <tokio::runtime::basic_scheduler::CURRENT>, t=0x7fffffffcaf0, f=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/macros/scoped_tls.rs:61
#15 0x000055555557b0b6 in tokio::runtime::basic_scheduler::enter (scheduler=0x7fffffffcdb8, f=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/runtime/basic_scheduler.rs:279
#16 0x0000555555579d3b in tokio::runtime::basic_scheduler::Inner<P>::block_on (self=0x7fffffffcdb8, future=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/runtime/basic_scheduler.rs:185
#17 0x000055555557a755 in tokio::runtime::basic_scheduler::InnerGuard<P>::block_on (self=0x7fffffffcdb8, future=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/runtime/basic_scheduler.rs:425
#18 0x000055555557aa9c in tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on (self=0x7fffffffd300, future=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/runtime/basic_scheduler.rs:145
#19 0x0000555555582094 in tokio::runtime::Runtime::block_on (self=0x7fffffffd2f8, future=...)
    at /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.4.0/src/runtime/mod.rs:450
#20 0x000055555557c22f in inventory_service::main () at /home/alan/code/inventory_service/src/main.rs:4
```ignore

</details>

Puzzled, the only line Alan even recognizes is the `main` entry point function for the service.
He knows that async tasks in Rust aren't run individually on their own threads which allows them to scale better and use fewer resources but surely there has to be a thread somewhere that's running his code?
Alan doesn't completely understand how async works in Rust but he's seen the `Future::poll` method so he assumes that there is a thread which constantly polls tasks to see if they are ready to wake up.
"Maybe I can find that thread and inspect its state?" he thinks and then consults the cheatsheet for the appropriate command to see the threads in the program.
`info threads` seems promising so he tries that:

<details><summary>(gdb) info threads</summary>

```ignore
(gdb) info threads
  Id   Target Id                                          Frame 
* 1    Thread 0x7ffff7c3b5c0 (LWP 1048) "inventory_servi" 0x00007ffff7d5e58a in epoll_wait (epfd=3, events=0x555555711340, 
    maxevents=1024, timeout=49152) at ../sysdeps/unix/sysv/linux/epoll_wait.c:30
```ignore

</details>

Alan is now even more confused: "Where are my tasks?" he thinks.
After looking through the cheatsheet and StackOverflow, he discovers there isn't a way to see which async tasks are waiting to be woken up in the debugger.
Taking a shot in the dark, Alan concludes that this thread must be thread which is polling his tasks since it is the only one in the program.
He googles "epoll_wait rust async tasks" but the results aren't very helpful and inspecting the stack frame doesn't yield him any clues as to where his tasks are so this seems to be a dead end.

After thinking a bit, Alan realizes that since the runtime must know what tasks are waiting to be woken up, perhaps he can have the service ask the async runtime for that list of tasks every 10 seconds and print them to stdout? 
While crude, this would probably also help him diagnose the hang.
Alan gets to work and opens the runtime docs to figure out how to get that list of tasks.
After spending 30 minutes reading the docs, looking at StackOverflow questions and even posting on users.rust-lang.org, he discovers this simply isn't possible and he will have to add tracing to his application to figure out what's going on.

Disgruntled, Alan begins the arduous, boring task of instrumenting the application in the hope that the logs will be able to help him.

## 🤔 Frequently Asked Questions


### **What are the morals of the story?**
* Developers, especially coming from an language that has a tightly integrated development environment, expect their debugger to help them particularly in situations where "println" debugging can't.
* If the debugger can't help them, developers will often try to reach for a programmatic solution such as debug functions in their runtime that can be invoked at critical code paths.
* Trying to debug an issue by adding logging and then triggering the issue is painful because of the long turn-around times when modifying code, compiling and then repro'ing the issue.

### **What are the sources for this story?**
* @erickt's comments in #76, similar comments I've heard from other developers.

## **Why did you choose Alan to tell this story?**
* Coming from a background in managed languages where the IDE, debugger and runtime are tightly integrated, Alan would be used to using those tools to diagnose his issue.
* Alan has also been a bit insulated from the underlying OS and expects the debugger to understand the language and runtime even if the OS doesn't have similar concepts such as async tasks.

### **How would this story have played out differently for the other characters?**
* Some of the characters with either a background in Rust or a background in systems programming might know that Rust's async doesn't always map to an underlying system feature and so they might expect that `gdb` or `lldb` is unable to help them.
* Barbara, the experienced Rust dev, might also have used a tracing/instrumentation library from the beginning and have that to fall back on rather than having to do the work to add it now.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
