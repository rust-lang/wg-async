# 😱 Status quo stories: Alan tries using a socket Sink

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Alan is working on a project that uses an alternative popular async framework. He finds the new async framework less familiar, and as he reads through their GitHub issues, he's saddened by the project maintainer's unwelcoming attitude towards their community. He asks his teammates if they can switch to the one he's more familiar with and feels more welcome in, but the teammates are understandably resistant to changing something that important just for one feature. So, he works hard to find workarounds to accomplish his team's goals.

One of the goals is to switch from a WebSocket implementation using raw TCP sockets to one managed behind an HTTP server library, so both HTTP and WebSocket commands can be forwarded to a transport-agnostic RPC server. He finds an HTTP server that's similar to one he's used to using with the other async framework, and a WebSocket middleware library that goes with it.

However, as he's working, Alan encounters a situation where the socket needs to be written to within an async thread, and the traits just aren't working. He wants to split the stream into a sender and receiver:

```rust
use futures::{SinkExt, StreamExt};
use async_std::sync::{Arc, Mutex};
use log::{debug, info, warn};

/// In the connection handler:
let (ws_sender, mut ws_receiver) = ws_stream.split();
let ws_sender = Arc::new(Mutex::new(ws_sender));

while let Some(msg) = ws_receiver.next().await {
    debug!("Received new WS RPC message: {:?}", msg);

    // Echo the request:
    match ws_sender.lock().await.send_string(msg).await {
        Ok(_) => info!("New WS data sent."),
        Err(_) => warn!("WS connection closed."),
    };
}
```

This is necessary because both sides of the stream are stateful iterators, and if they're kept as one, the lock on the receiver can't be released within the loop. He's seen this pattern used in other projects in the Rust community, but is frustrated to find that the `Sink` trait wasn't implemented in the WebSockets middleware library he's using.

Alan also tries creating a sort of poller worker thread using an intermediary messaging channel, but he has trouble reasoning about the code and wasn't able to get it to compile:

```rust
use async_std::channel;
use async_std::sync::{Arc, Mutex};
use log::{debug, info, warn};

/// In the connection handler:
let (ws_sender, mut ws_receiver) = channel::unbounded::<String>();
let ws_receiver = Arc::new(ws_receiver);

let ws_stream = Arc::new(Mutex::new(ws_stream));
let poller_ws_stream = ws_stream.clone();

async_std::task::spawn(async move {
    while let Some(msg) = ws_receiver.next().await {
        match poller_ws_stream.lock().await.send_string(msg).await {
            Ok(msg) => info!("New WS data sent. {:?}", msg),
            Err(msg) => warn!("WS connection closed. {:?}", msg),
        };
    }
});

while let Some(msg) = ws_stream.lock().await.next().await {
    // Echo the request?
    ws_sender.send(msg);
}
```

Alan wonders if he's thinking about it wrong, but the solution isn't as obvious as his earlier Sink approach. Looking around, he realizes a solution to his problems already exists-- as others have been in his shoes before-- within two other nearly-identical pull requests, but they were both closed by the project maintainers. He tries opening a third one with the same code, pointing to an example where it was actually found to be useful. To his joy, his original approach works with the code in the closed pull requests in his local copy! Alan's branch is able to compile for the first time.

However, almost immediately, his request was also shut down, and the maintainer suggested he try the complex, unobvious, and unspecific solution that he had already tried and just couldn't get it to work.

As a result of his frustration, Alan calls out one of the developers of the project on social media, who he believed had frequently shot down the idea of using Sink traits on projects others needed to use them on. This is not well-received, the maintainer reacts with a condescending and dismissive attitude, and he later finds out he was blocked. A co-maintainer responds to the thread, defending and supporting the other maintainer's actions, and he's told to "get over it". He's given a link to a blog post with a piece [lamenting the popularity of Sink in the Rust ecosystem](https://blog.yoshuawuyts.com/rust-streams/#why-we-do-not-talk-about-the-sink-trait), and how it's complex and bad and not worth it, but the piece also unhelpfully makes no effort to provide examples of how the better alternatives would be used to replace uses of Sink.

Because of this heated exchange, Alan grows concerned for his own career, what these well-known community members might think or say about his to others, and his confidence in the community surrounding this language that he really enjoys using is somewhat shaken.

Despite this, Alan takes a walk, gathers his determination, and commits to maintaining his fork with the changes from the other pull requests that were shut down, and publihes his version to Crates.io, vowing to be more welcoming to "misfit" pull requests like the one he needed.

A few weeks later, Alan's work at his project at work is merged with his new forked crate. It's a big deal, his first professional open source contribution to a Rust project, but he still doesn't feel like he has a sense of closure with the community. Meanwhile, his friends say they want to try Rust, but they're worried about its async execution issues, and he doesn't know what else to say, other than to offer a sense of understanding. Maybe the situation will get better someday, he hopes.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

* **What are the morals of the story?**
    * There are often many sources of opinion in the community regarding futures and async, but these opinions aren't always backed up with examples of how it should be better accomplished. Sometimes we just find a thing that works and would prefer to stick with it, but others argue that some traits make implementations unnecessarily complex, and choose to leave it out. Disagreements like these in the ecosystem can be harmful to the reputation of the project and the participants.
    * If there's a source of substantial disagreement, the community becomes even further fragmented, and this may cause additional confusion in newcomers.
* **What are the sources for this story?**
    * <https://github.com/http-rs/tide-websockets>
        * <https://github.com/http-rs/tide-websockets/pull/17>
    * <https://github.com/cryptoquick/tide-websockets-sink>
    * <https://twitter.com/cryptoquick/status/1370143022801846275>
    * <https://twitter.com/cryptoquick/status/1370155726056738817>
* **Why did you choose [Alan](../characters/alan.md) to tell this story?**
    * Alan is more representative of the original author's background in JS, TypeScript, and NodeJS.
* **How would this story have played out differently for the other characters?**
    * (I'm not sure.)

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade