# ✨ Shiny future stories: Grace debugs a crash dump again

## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

It's been a few years since the new [DistriData] database has shipped. For the most part things have gone smoothly. The whole team is confident in trusting the compiler, and they have far fewer bugs in production than they had in the old system. The downside is that now when a bug does make it to production, it tends to be really subtle and take a lot of time to get right.

Today when Grace opens her e-mail, she discovers she's been assigned to investigate a dump from a crash that has been occurring in production lately. The crash happens rarely, so it's important to glean as much information as possible. They need to get this fixed soon!

Even though there's a lot of pressure around this situation, Grace is grateful that she won't have to fight her tools to make progress. A lot has changed in Async Rust over the years. The async community got together and defined the Async Debugging Protocol, which provides a standard way for tools to inspect the state of an asynchronous Rust program. Many of the most popular runtimes like Tokio and async-std follow this protocol, and a number of tools have been written to use the protocol as well. Even though Grace's team has opted to build a custom runtime to address their own unique needs, it was not too much work to implement the Async Debugging Protocol and it was well worth it due to the increase in developer productivity. This has truly revolutionized async debugging in much the same way the [Language Server Protocol] did for IDEs.

Upon opening the crash dump, her favorite debugger immediately gives an overview of the state of the program at the point it crashed. It shows what executors are running, how many OS-level threads each executor is using, what tasks are there, and what the state of each task is. For each thread, Grace can see a stack trace and the debugger provides a logical stack trace for each task as well. Many of the resources that the blocked tasks are waiting on are visible too, particularly those provided by the runtime like timers, mutexes, and I/O.

This high level, generic view provides a good start, but the team's custom executor provides additional functionality that the Async Debugging Protocol does not support. Still, using the features already provided as a starting point, Grace was able to write some additional debugging macros to recover the additional state. These macros are used by the whole team and are now a standard part of their debugging toolkit.

Grace has seen a few instances of this crash now and she notices a constellation of tasks that look a little funny. This gives her an idea for what might be going wrong. She uses that to add a new test case than ends up crashing the service in a way that looks very similar. It seems like she's found the bug! Even better, it looks like it should be a simple fix and the team will be able to put this issue behind them once and for all.

## 🤔 Frequently Asked Questions

### What status quo stories are you retelling?

Grace debugs a crash dump.

### What are the key attributes of this shiny future?

* Most of the abilities to inspect executor and task state while debugging a live process also work on crash dumps.
* Debugging async programs is both runtime- and tooling- agnostic.
    * People should be able to get a good experience using whatever tools they are comfortable with, whether that's gdb, lldb, VS Code, IntelliJ, or a specialized Rust async debugger.
    * Debugging tools should be able to work with different runtimes. Not all projects in an organization will use the same one, and some may be custom.
* It's possible to see the following things while debugging:
    * What tasks are running, along with logical stack traces.
    * Some idea of what the task is waiting on if it is blocked.
    * If there are multiple executors, we can inspect each one.
    * Raw stack traces for the OS-level threads that the executors use to schedule tasks.
* Additional tooling may be necessary for custom or exotic executors. The hypothetical Async Debugging Protocol is one size fits all, but one size won't fit all. We don't want to constrain what an executor can do just so we can debug it.
* An async runtime should not be required to support these common debugging features. For example, perhaps it requires more space to support and therefore is not appropriate for an extremely constrained embedded environment.

I envisioned provided this with some kind of "Async Debugging Protocol" that is analogous to the Language Server Protocol. It's not really clear what this would be exactly, and there may be a better approach to solving these problems. For live debugging, it may be as simple as a few traits the executor can implement that provide introspection capabilities. For crash dumps, maybe there's a convention around including a couple of debugging symbols. It might require some kind of rich metadata format that tells the debugger how to inspect and interpret the core data structures for the executor.

### What is the "most shiny" about this future? 


The biggest aspect of this shiny future is the increased developer productivity, particularly in debugging. Many of the status quo stories called out the difficulty of debugging async code. In this shiny future, there are really good tools for live debugging, and many of these work offline in the crash dump case as well.

As a follow-on, the enhanced developer productivity will support writing more correct and safer programs, and probably allow developers to diagnose performance problems as well. These are a direct consequence of better debugging, but rather an indirect consequence of giving the developer better tools.

### What are some of the potential pitfalls about this future?

Depending on how the "Async Debugging Protocol" works, there may be some overhead in following it. Hopefully this would be minimal, and not require any additional code during normal execution scenarios. But, it might make the debugging symbols or other metadata larger. Following the protocol may constrain some of the choices an async runtime can make.

At the very least, choosing to follow the protocol will require additional work on the part of the runtime implementor.

### Did anything surprise you when writing this story? Did the story go any place unexpected?

Doing this in a way that is runtime and tooling agnostic will be challenging, so the details of how that could be done are not included in this story.

In some ways, doing this for a live process seems easier, since you can write code that inspects or reports on its own state. This seems to be the approach that [tokio-console] is taking.

There seems to be a lot of overlap between live debugging scenarios and post-mortem scenarios. With a little care, it might be able to support both using many of the same underlying capabilities.

[tokio-console]: https://github.com/tokio-rs/console

### What are some variations of this story that you considered, or that you think might be fun to write? Have any variations of this story already been written?

It would be worth removing the runtime agnostic aspect of this story and looking at how things would look if we just focused on Tokio or async-std. Perhaps each runtime would include a set of debugger macros to help find the runtime's state.

### What are some of the things we'll have to figure out to realize this future? What projects besides Rust itself are involved, if any? (Optional)

A lot of the work here probably will not be done by the core Rust team, other than perhaps to coordinate and guide it. Most of the work will require coordination among projects like Tokio and async-std, as well as the debugging tool authors.

There does not seem to be an obvious way to implement everything in this story. It would probably be good to focus on a particular runtime at least to get a proof of concept and better sketch out the requirements.

[character]: ../characters.md
[comment]: ./comment.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[projects]: ../projects.md
[htvsq]: ../how_to_vision/shiny_future.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
[DistriData]: ../projects/DistriData.md
[Language Server Protocol]: https://microsoft.github.io/language-server-protocol/