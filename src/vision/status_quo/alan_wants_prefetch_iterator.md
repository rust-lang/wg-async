# 😱 Status quo stories: Alan wants an async iterator with prefetch

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Alan once wrote a data processing microservice in a GC'd language which was designed for high throughput. Now he wants to write it in Rust and have strong ownership model.

The original service consumes messages from a source stream (e.g. Kafka), process them and produces results to another stream and/or saves them to a database. Since the service acquaries some data from other sources like external services and its own PostgreSQL database, Alan batches incoming messages to acquarie as much as possible data from that sources with minimal overhead.

Since messages might arrive with some delays between them, or can end at some point for a while, their number is unknown, there's an async iterator which reads the input stream and waits some time before producing a batch if the next message isn't immediately ready.

While this kind of iterator returns control to the caller, and the caller continues execution, there can be a running future preparing the next batch to be taken synchronously with a very high probability.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**
* Async looks not so hard after reading documentation and block posts on the topic, but when it comes to implementing your own async iterator, things stops seeming so beautiful as before. There's a very steep learning curve.

### **What are the sources for this story?**
Personal experience of the author.

### **Why did you choose *NAME* to tell this story?**
As a backend developer in a GC'd language, Alan writes async code every day. He wants to gain the maximum performance and have memory safety at the same time.

### **How would this story have played out differently for the other characters?**
*In some cases, there are problems that only occur for people from specific backgrounds, or which play out differently. This question can be used to highlight that.*

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade