
## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

The problem:
When you have a complex network of async tasks it can be difficult to debug issues or investigate behavior because it’s hard to reason through the path of execution just by reading the code.  Adding async tracing helps solve this by letting you trace an event through the network and see which async tasks the event executed and when and in what order.

Character is Barbara:
Barbara’s team works on a set of services that power the API that powers her company’s website and all the
features their customer’s use. They’ve built the backend for these services in Rust and make heavy use of
`async` to manage IO bound operations and help make concurrency easier to leverage. However, the services
have grown quite a bit and there are a large number of features and data requirements and different internal
systems which they must interact with. The result is a very complex network of `async` expressions that do the
job well and perform great, but, are too complex to easily reason about anymore and can be extraordinarily 
intimidating when trying to fix transient small issues. Issues such as infrequent slow requests or a very small number
of requests executing certain actions out of order are very hard to resolve when the network of `async` expressions
is complex.

Recently, Barbara and her team have been notified about some customers experiencing slow responses on
some features.  The lag events are rare but Barbara and her team are determined to fix them.  With some work
Barbara is able to recreate the lag reliably in the QA environment; but now she must figure out where in the
complex code base this lag could be coming from and why it’s happening.  Fortunately, Rust’s `async` framework
now provides a built in Tracing tool.  By building her service with the `tracing` flag on, her code is automatically
instrumented and will start logging trace data to a file for later analysis.

Barbara runs the instrumented code in QA and recreates the laggy event several times.  Then she takes the 
generated trace file and looks through the data.  She immediately sees where each of the slow requests
actually lagged.  Each request experienced a slow down in different async expressions, but each expression
had one thing in common they each queried the same database table. She also noticed that there was a relation
in when the latency occurred: all the laggy requests tended to occur in clusters. From this she was able to identify
that the root cause was some updates made to the database which led to some queries, if they arrived together,
to run relatively slowly. With tracing, Barbara was saved the effort of having to meticulous work through the code
and try to deduce what the cause was and she didn’t have to add in a large amount of logging or other
instrumentation.  All the instrumentation and analysis was provided out of the box and required no development
work for Barbara to isolate the cause. 

Barbara can’t believe how much time she saved having this debugging tool provided out of the box.

## 🤔 Frequently Asked Questions

*NB: These are generic FAQs. Feel free to customize them to your story or to add more.*

### What status quo stories are you retelling?

*Link to status quo stories if they exist. If not, that's ok, we'll help find them.*
[Alan Builds A Cache](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_builds_a_cache.html)
[Alan Iteratively Regresses Performance](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_iteratively_regresses.html)
[Alan Traies To Debug A Hang](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_tries_to_debug_a_hang.html)

### What are the key attributes of this shiny future?

- Provide a protocol for linking events across async expressions.
- Provide an output that allows a user to understand the path of execution of a program through a network of async expressions.

### What is the "most shiny" about this future? 

*Thing about Rust's core "value propositions": performance, safety and correctness, productivity. Which benefit the most relative to today?*

- This will benefit the productivity of a developer. Providing a whole new way of debugging Rust programs and giving a way to view the actual execution of code in a human readable form can make it significantly faster to debug programs.  This also saves time for a developer from having to write a tracer themselves.
- This can also help with correctness. When working with asynchronous code it can be difficult; having a built-in means to trace a flow of execution makes it much easier to verify that specific inputs are following the correct paths in the correct order.

### What are some of the potential pitfalls about this future?

*Thing about Rust's core "value propositions": performance, safety and correctness, productivity. Are any of them negatively impacted? Are there specific application areas that are impacted negatively? You might find the sample [projects] helpful in this regard, or perhaps looking at the goals of each [character].*

- Figuring out how to propagate a trace ID in a way that’s compatible with any use of async could be difficult
- Recording trace data will have some impact on performance.
- We could output too much data for a person to be able to use it.

### Did anything surprise you when writing this story? Did the story go any place unexpected?

*The act of writing shiny future stories can uncover things we didn't expect to find. Did you have any new and exciting ideas as you were writing? Realize some complications that you didn't foresee?*

No.

### What are some variations of this story that you considered, or that you think might be fun to write? Have any variations of this story already been written?

Another variation of this story is tracking down functional bugs: where the program is not always executing the expected code paths.  An example of this is from the status quo story [Alan Builds A Cache](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_builds_a_cache.html).  In this type of story, a developer uses tracing to see execution flow of an event as it is fully processed by the application. This can the be used to make sure that every expected or required action is completed and done in the correct order; and if actions were missed, be able to determine why.

### What are some of the things we'll have to figure out to realize this future? What projects besides Rust itself are involved, if any? (Optional)

*Often the 'shiny future' stories involve technical problems that we don't really know how to solve yet. If you see such problems, list them here!*

- There will need to be some form of protocol for how to trace data as they move through a graph of async expressions. Perhaps by weaving a trace ID through the execution of async workflows. We will also have to provide a way "inject" or "wrap" this protocol around the users data in a way that can be automatically done as a compile time option (or is always done behind the scenes).
- A protocol or standard for recording this information and decorating logs or metrics with this data would need to be provided.
- Collecting entry and exit events for async expressions and linking them together in a graph
- How to store the traces
- How to identify each async expression so that a user knows what step in the trace refers to.
- How to show this information to the user.


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
