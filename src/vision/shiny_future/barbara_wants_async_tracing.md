
## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

The problem:
When you have a complex network of async tasks it can be difficult to debug issues or investigate behavior because it’s hard to reason through the path of execution just by reading the code.  Adding async tracing helps solve this by letting you trace an event through the network and see which async tasks the event executed and when and in what order.

Character:
This is something an experienced Rust developer will deal with.  They know enough to be working with async and be building complex async networks. And they now need tools to help them debug issues that arise in those systems. They probably already build things like this, but providing tracing out of the box would save them time and effort.

I believe this would be Barbara: the experienced Rust developer

Story:
Barbara has written a multistage async workflow to solve problem X

A bug is found where for specific inputs the output is wrong. Barbara wants to confirm exactly what sequence of steps events follow so that she can quickly prove that all the correct async steps are being take and in the right order.

So she writes a wrapper that includes a tracing ID so at each async step she can output a log message with the trace ID and then use the log file to see what the sequence of events is.

What Barbara would like is to have this feature be built in, perhaps with a compile time flag. That would create the trace wrapper and record trace data for her to view and analyze.

## 🤔 Frequently Asked Questions

*NB: These are generic FAQs. Feel free to customize them to your story or to add more.*

### What status quo stories are you retelling?

*Link to status quo stories if they exist. If not, that's ok, we'll help find them.*

### What are the key attributes of this shiny future?

- Provide a protocol for linking events across async expressions
- Provide an output that allows a user to construct a graph showing the path of execution of an async expression

### What is the "most shiny" about this future? 

*Thing about Rust's core "value propositions": performance, safety and correctness, productivity. Which benefit the most relative to today?*

- Providing a whole new way of debugging Rust programs and giving a way to view the actual execution of code in a human readable form
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
