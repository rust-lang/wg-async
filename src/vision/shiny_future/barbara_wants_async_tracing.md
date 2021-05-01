Story for Async WG: Async Tracing

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

FAQ:
1. What status quo stories does this pull?
2. Key attributes of this story
- Provide a protocol for linking events across async expressions
- Provide an output that allows a user to construct a graph showing the path of execution of an async expression
3. What is most shiny about this future?
- Providing a whole new way of debugging Rust programs and giving a way to view the actual execution of code in a human readable form
4. What are some potential pitfalls?
- Figuring out how to propagate a trace ID in a way that’s compatible with any use of async could be difficult
- It could have some performance impact
- We could output too much data for a person to be able to use it
5. Anything surprising when writing this story?
No.
6. Fun Variations
7. What are some things that we’ll have to figure out?
- Weave a trace ID through the execution of async workflows.
- Collecting entry and exit events for async expressions and linking them together in a graph
- How to store the traces
- How to identify each async expression so that a user knows what step in the trace refers to
- How to show this information to the user