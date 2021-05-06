
## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

The problem:
When you have a complex network of async tasks it can be difficult to debug issues or investigate behavior because it’s hard to reason through the path of execution just by reading the code.  Adding async tracing helps solve this by letting you trace an event through the network and see which async tasks the event executed and when and in what order.

Character is Barbara:
This is something an experienced Rust developer will deal with.  They know enough to be working with async and be building complex async networks. And they now need tools to help them debug issues that arise in those systems. They probably already build things like this, but providing tracing out of the box would save them time and effort.

This is Barbara: the experienced Rust developer.

Story:
Barbara has written a multistage async workflow to solve X

Barbara is seeing slow performance in one of her services.  This service happens to make extensive use of asynchronous workflows and builds a rather complex graph of possible execution paths. Unfortunately, this means that it's hard to intuit what could be causing the slowdown.  Perhaps a slow API call or some performance issue in the runtime.  What Barbara wants is to be able to see the full path of execution the slow events are taking through her service and be able to see the timing through events.  She can add log statements to each task, but how will she be able to link all the log statements to the events which they are associated with?

To solve this problem, Barbara needs to be able to trace events through her service and link log entries to specific events. So, Barbara sets about creating a simple tracing wrapper around the events.  This tracing wrapper is designed to provide a way to associate every event in the system with events that they cause and to associate events with the logs she writes. To this end, she tracks a `message id` UUID that gets attached to the start of every asynchronous workflow and which is propagated consistently to any child events that get triggered. She then updates her code to capture the `message id` and insert into her logs.

Now Barbara is able to run some tests and find a message which demonstrated the performance issue she was investigating and isolate the logs for that message. With this she is able to get the time spent in each subsection of code and compare this to the trace of a message with expected behavior. Now it's obvious that there's one step in the workflow which periodically runs slower than normal and it is the only part of execution path with this issue. Looking at additional traces, Barbara realizes that the issue effects several events all around the same time and is able to deduce that the issue is most likely due to a shared resource performing poorly.

Barbara is grateful to have found the issue but wishes this tooling and been provided out of the box.  Understanding the behavior of a complex async service is difficult: both investigating performance and business logic require a lot of tedious work to reason through the paths an event may travel. Making tracing an essential tool that provides that information immediately. Her dream would be to have a compile time flag, e.g. `cargo build --tracing`, which would instrument her async code with the tracing wrapper and allow her to record tracing data when she runs her application. Even better if there are a set of supplied tools for viewing and analyzing the output data.

## 🤔 Frequently Asked Questions

*NB: These are generic FAQs. Feel free to customize them to your story or to add more.*

### What status quo stories are you retelling?

*Link to status quo stories if they exist. If not, that's ok, we'll help find them.*

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
- It could have some performance impact
- We could output too much data for a person to be able to use it

### Did anything surprise you when writing this story? Did the story go any place unexpected?

*The act of writing shiny future stories can uncover things we didn't expect to find. Did you have any new and exciting ideas as you were writing? Realize some complications that you didn't foresee?*

No.

### What are some variations of this story that you considered, or that you think might be fun to write? Have any variations of this story already been written?

*Often when writing stories, we think about various possibilities. Sketch out some of the turning points here -- maybe someone will want to turn them into a full story! Alternatively, if this is a variation on an existing story, link back to it here.*

### What are some of the things we'll have to figure out to realize this future? What projects besides Rust itself are involved, if any? (Optional)

*Often the 'shiny future' stories involve technical problems that we don't really know how to solve yet. If you see such problems, list them here!*

- Weave a trace ID through the execution of async workflows.
- Collecting entry and exit events for async expressions and linking them together in a graph
- How to store the traces
- How to identify each async expression so that a user knows what step in the trace refers to
- How to show this information to the user


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
