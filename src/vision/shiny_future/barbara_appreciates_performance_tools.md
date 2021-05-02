# ✨ Shiny future stories: Barbara appreciates great performance analysis tools

[How To Vision: Shiny Future]: ../how_to_vision/shiny_future.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async-foundations/master/src/vision/shiny_future/template.md
[`shiny_future`]: https://github.com/rust-lang/wg-async-foundations/tree/master/src/vision/shiny_future
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async-foundations/blob/master/src/SUMMARY.md

## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

[Barbara] has built an initial system prototype in sync Rust. She notes that it's completely I/O bound, and benchmarking shows that most of her CPU consumption is thread switch overheads. She decides to rewrite it in async Rust, using an executor that she believes will fix her bottlenecks.

She sprinkles `async/.await` in all the right places, switches her sync dependencies to async libraries, and gets the code compiling. When she runs it, she discovers that the service no longer responds when she sends a request to the endpoint. Her logging shows her that the endpoint handler has been invoked, many tasks have been spawned, but that something isn't working as she expected.

Fortunately, there are great tracing tools available for async Rust. Barbara turns on tracing, and immediately gets interesting information in her trace viewer. She can see all the tasks she has spawned, the lines of code where a `.await` returns control to the executor, and delays between a `Waker` being invoked and the corresponding `.await` resuming execution.

With this information in hand, she finds a decompression path that is unexpectedly CPU-bound, because she can see a stack trace for the task that is running and blocking a woken up future from getting invoked again. The memory use of this future tells her that the compressed blobs are larger than she thought, but inspecting shows that this is reasonable. She thus puts the decompression onto its own blocking task, which doesn't fix things, but makes it clear that there is a deadlock passing data between two bounded channels; the trace shows the `Waker` for a `rx.next().await` being invoked, but the corresponding `.await` never runs. Looking into the code, she notes that the task is waiting on a `tx.send().await` call, and that the channel it is trying to send to is full. When Barbara reads this code, she identifies a classic AB-BA deadlock; the task that would consume items from the channel this task is waiting on is itself waiting on a transmit to the queue that this task will drain.

She refactors her code to resolve this issue, and then re-checks traces. This time, the endpoint behaves as expected, but she's not seeing the wall clock time she expects; the trace shows that she's waiting on a network call to another service (also written in async Rust), and it's taking about 10x longer to reply than she would expect. She looks into the tracing libraries, and finds two useful features:

1. She can annotate code with extra information that appears on the traces.
2. Every point in the code has access to a unique ID that can be passed to external services to let her correlate traces.

Barbara adds annotations that let her know how many bytes she's sending to the external service; it's not unreasonable, so she's still confused. A bit of work with the service owner, and she can now get traces from the external service that have IDs she sends with a request in them. The tooling combines traces nicely, so that she can now trace across the network into the external service, and she realises that it's going down a slow code path because she set the wrong request parameters.

With the extra insights from the external service's trace, she's able to fix up her code to run perfectly, and she gets the desired wins from async Rust. Plus, she's got a good arsenal of tooling to use when next she sees an unidentified problem.

## 🤔 Frequently Asked Questions

### What status quo story or stories are you retelling?

* [Barbara compares some C++ code (and has a performance problem)](../status_quo/barbara_compares_some_cpp_code.md)
* [Barbara wants Async Insights](../status_quo/barbara_wants_async_insights.md)

### **What is [Alan] most excited about in this future? Is he disappointed by anything?**

Alan is excited about how easy it is to find out when his projects don't work as expected. He's happy

### **What is [Grace] most excited about in this future? Is she disappointed by anything?**

Grace is happy because the performance tools give her all the low level insights she wants into her code, and shows her what's going on "behind the scenes" in the executor. As a C++ developer, she is also excited when she sees that Rust developers who see an issue with her services can give her useful information about exactly what they see her C++ doing - which she can correlate with her existing C++ performance tools via the unique ID.

### **What is [Niklaus] most excited about in this future? Is he disappointed by anything?**

Niklaus is content. The tooling tells him what he needs to know, and allows him to add interesting information to places where he'd otherwise be stuck trying to extract it via `println!()`. He's not entirely sure how to use some of the detailed information, but he can ignore it easily.

### **What is [Barbara] most excited about in this future? Is she disappointed by anything?**

Barbara is impressed at how easy it is to spot problems and handle them; she is especially impressed when the tooling is able to combine traces from two services and show her their interactions in a useful fashion as-if they were one process. She kinda wishes that the compiler would spot more of the mistakes she made - the decompression path should be something the compiler should get right for her - but at least the tooling made the problems easy to find.

### **What [projects] benefit the most from this future?**

All the projects benefit; there's a useful amount of tracing "for free", and places where you can add your own data as needed.

### **Are there any [projects] that are hindered by this future?**

[MonsterMesh] needs to be able to remove a lot of the tracing because the CPU and memory overhead is too high in release builds.

[MonsterMesh]: ../projects/MonsterMesh.md

### **What are the incremental steps towards realizing this shiny future?**

The [tracing] crate has a starting point for a useful API; combined with [tracing-futures], we have a prototype.

Next steps are to make integrating that with executors trivial (minimal code change), and to add in extra information to [tracing-futures] so that we can output the best possible traces. In parallel to that, we'll want to work on tooling to display, combine, and filter traces so that we can always extract just what we need from any given trace.

[tracing]: https://crates.io/crates/tracing
[tracing-futures]: https://crates.io/crates/tracing-futures

### **Does realizing this future require cooperation between many projects?**

Yes. We need an agreed API for tracing that all async projects use - both to add tracing information, and to consume it in a useful form.

[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[projects]: ../projects.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
