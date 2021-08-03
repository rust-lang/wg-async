# 😱 Status quo stories: Alan wants an async iterator with prefetch

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Alan once wrote a data processing microservice in a GC'd language which was designed for high throughput. Now he wants to write it in Rust and have strong ownership model.

The original service consumes messages from a source stream (e.g. Kafka), process them and produces results to another stream and/or saves them to a database. Since the service acquires some data from other sources like external services and its own PostgreSQL database, Alan batches incoming messages to acquire as much as possible data from that sources with minimal overhead.

Since messages might arrive with some delays between them, or can end at some point for a while, their number is unknown, there's an async iterator which reads the input stream and waits some time before producing a batch if the next message isn't immediately ready.

Alan explored `FutureExt` from `async-std` and found no evidence that it's possible to wait for multiple futures returning different results (it's not possible for `ValueTask`s in .NET, but it worked well with `Task`s which can be awaited multiple times). Later he was suggested to use an `enum` and the `race` method to achive his goal:

```rust
enum Choices<A, B, C> {
    A(A),
    B(B),
    C(C),
}

// convert each future into the type `Choices<...>`:
let future_a = async move { A(future_a.await) };
let future_b = async move { B(future_b.await) };
let future_c = async move { C(future_c.await) };

// await the race:
match future_a.race(future_b).race(future_c).await {
    A(a) => ...,
    B(b) => ....,
    C(c) => ...,
}
```

While that helped Alan, it was completely unobvious to him. He expected to see a macro accepting futures and producing a new future to be awaited:

```rust
match race!(future_a, future_b, future_c).await {
    // ...
}
```

Having `join!` would be nice too for Alan, so he can avoid binding variables to futures which later shall be awaited:

```rust
// How it's now
let future_a = do_async_a();
let future_b = do_async_b();
let future_c = do_async_c();

let result_a = future_a.await;
let result_b = future_b.await;
let result_c = future_c.await;

// How it could be
let (result_a, result_b, result_c) = join!(future_a, future_b, future_c).await;
```

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**
* Even though Alan had experience writing async code in other languages, he had a hard time figuring out how to do relatively simple things in Rust, like joining or racing on futures of different types.

### **What are the sources for this story?**
Personal experience of the author.

### **Why did you choose Alan to tell this story?**
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
