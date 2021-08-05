# 😱 Status quo stories: Template

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Barbara is working on the [YouBuy] server. In one particular part of the story, she has a process that has to load records from a database on the disk. As she receives data from the database, the data is sent into a channel for later processing. She writes an `async fn`  that looks something like this:

```rust
async fn read_send(db: &mut Database, channel: &mut Sender<...>) {
  loop {
    let data = read_next(db).await;
    let items = parse(&data);
    for item in items {
      channel.send(item).await;
    }
  }
}
```

This database load has to take place while also fielding requests from the user. The routine that invokes `read_send` uses [`select!`](https://docs.rs/futures/0.3.14/futures/macro.select.html) for this purpose. It looks something like this:

```rust
let mut db = ...;
let mut channel = ...;
loop {
    futures::select! {
        _ = read_send(&mut file, &mut channel) => {},
        some_data = socket.read_packet() => {
            // ...
        }
    }
}
```

This setup seems to work well a lot of the time, but Barbara notices that the data getting processed is sometimes incomplete. It seems to be randomly missing some of the rows from the middle of the database, or individual items from a row.

### Debugging

She's not sure what could be going wrong! She starts debugging with print-outs and logging. Eventually she realizes the problem. Whenever a packet arrives on the socket, the `select!` macro will drop the other futures. This can sometime cause the `read_send` function to be canceled in between reading the data from the disk and sending the items over the channel. Ugh!

Barbara has a hard time figuring out the best way to fix this problem.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

* Cancellation doesn't always cancel the entire task; particularly with `select!`, it sometimes cancels just a small piece of a given task.
    * This is in tension with Rust's original design, which was meant to tear down an entire thread or task at once, precisely because of the challenge of writing exception-safe code.
* Cancellation in Async Rust therefore can require fine-grained recovery.

### **What are the sources for this story?**

This was based on [tomaka's blog post](https://tomaka.medium.com/a-look-back-at-asynchronous-rust-d54d63934a1c), which also includes a number of possible solutions, all of them quite grungy.

### **Why did you choose Barbara to tell this story?**

The problem described here could strike anyone, including veteran Rust users. It's a subtle interaction that is independent of source language. Also, the original person who reported it, tomaka, is a veteran Rust user.

### **How would this story have played out differently for the other characters?**

They would likely have a hard time diagnosing the problem. It really depends on how well they have come to understand the semantics of cancellation. This is fairly independent from programming language background; knowing non-async Rust doesn't help in particular, as this concept is specific to async code.

### What is different between this story and other cancellation stories?

There is already a story, ["Alan builds a cache"] that covers some of the challenges around cancellation. It is quite plausible that those stories could be combined, but the focus of this story is different. The key moral of this story is that certain combinators, notably `select!`, can cause small pieces of a single task to be torn down and canceled. This cancellation can occur for any reason -- it is not always associated with (for example) clients timing out or closing sockets. It might be (as in this story) the result of clients *sending* data!

This is one key point that makes cancellation in async Rust rather different than panics in sync Rust. Panics in sync Rust generally occur for bugs, to start, and they are typically not meant to be recovered from except at a coarse-grained level. In contrast, as this story shows, cancellation can require fine-grained recovery and for non-bug events.


["Alan builds a cache"]: alan_builds_a_cache.md
[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
[YouBuy]: ../projects/YouBuy.md