# 😱 Status quo stories: Alan tries using a socket Sink

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Alan is working on a project that uses `async-std`. He has worked a bit with `tokio` in the past and is more familiar with that, but he is interested to learn something how things work in `async-std`.

One of the goals is to switch from a WebSocket implementation using raw TCP sockets to one managed behind an HTTP server library, so both HTTP and WebSocket RPC calls can be forwarded to a transport-agnostic RPC server.

In this server implementation:

* RPC call strings can be received over a WebSocket
* The strings are decoded and sent to an RPC router that calls the methods specified in the RPC call
* Some of the methods that are called can take some time to return a result, so they are spawned separately
    * RPC has built-in properties to organize call IDs and methods, so results can be sent in any order
* Since WebSockets are bidirectional streams (duplex sockets), the response is sent back through the same client socket

He finds the HTTP server `tide` and it seems fairly similar to `warp`, which he was using with `tokio`. He also finds the WebSocket middleware library `tide-websockets` that goes with it.

However, as he's working, Alan encounters a situation where the socket needs to be written to within an async thread, and the traits just aren't working. He wants to split the stream into a sender and receiver:

```rust
use futures::{SinkExt, StreamExt};
use async_std::sync::{Arc, Mutex};
use log::{debug, info, warn};

async fn rpc_ws_handler(ws_stream: WebSocketConnection) {
    let (ws_sender, mut ws_receiver) = ws_stream.split();
    let ws_sender = Arc::new(Mutex::new(ws_sender));

    while let Some(msg) = ws_receiver.next().await {
        debug!("Received new WS RPC message: {:?}", msg);

        let ws_sender = ws_sender.clone();

        async_std::task::spawn(async move {
            let res = call_rpc(msg).await?;

            match ws_sender.lock().await.send_string(res).await {
                Ok(_) => info!("New WS data sent."),
                Err(_) => warn!("WS connection closed."),
            };
        });
    }
}
```

This is necessary because both sides of the stream are stateful iterators, and if they're kept as one, the lock on the receiver can't be released within the loop. He's seen this pattern used in other projects in the Rust community, but is frustrated to find that the `Sink` trait wasn't implemented in the WebSockets middleware library he's using.

Alan also tries creating a sort of poller worker thread using an intermediary messaging channel, but he has trouble reasoning about the code and wasn't able to get it to compile:

```rust
use async_std::channel;
use async_std::sync::{Arc, Mutex};
use log::{debug, info, warn};

async fn rpc_ws_handler(ws_stream: WebSocketConnection) {
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
        async_std::task::spawn(async move {
            let res = call_rpc(msg).await?;
            ws_sender.send(res);
        });
    }
}
```

Alan wonders if he's thinking about it wrong, but the solution isn't as obvious as his earlier `Sink` approach. Looking around, he realizes a solution to his problems already exists-- as others have been in his shoes before-- within two other nearly-identical pull requests, but they were both closed by the project maintainers. He tries opening a third one with the same code, pointing to an example where it was actually found to be useful. To his joy, his original approach works with the code in the closed pull requests in his local copy! Alan's branch is able to compile for the first time.

However, almost immediately, his request is closed [with a comment suggesting that he try to create an intermediate polling task instead](https://github.com/http-rs/tide-websockets/issues/15#issuecomment-797090892), much as he was trying before. Alan is feeling frustrated. "I already tried that approach," he thinks, "and it doesn't work!"

As a result of his frustration, Alan calls out one developer of the project on social media. He knows this developer is opposed to the `Sink` traits. Alan's message is not well-received: the maintainer sends a short response and Alan feels dismissed. Alan later finds out he was blocked. A co-maintainer responds to the thread, defending and supporting the other maintainer's actions, and suggests that Alan "get over it". Alan is given a link to a blog post. The post provides a number of criticisms of `Sink` but, after reading it, Alan isn't sure what he should do instead.

Because of this heated exchange, Alan grows concerned for his own career, what these well-known community members might think or say about his to others, and his confidence in the community surrounding this language that he really enjoys using is somewhat shaken.

Despite this, Alan takes a walk, gathers his determination, and commits to maintaining his fork with the changes from the other pull requests that were shut down. He publishes his version to crates.io, vowing to be more welcoming to "misfit" pull requests like the one he needed.

A few weeks later, Alan's work at his project at work is merged with his new forked crate. It's a big deal, his first professional open source contribution to a Rust project! Still, he  doesn't feel like he has a sense of closure with the community. Meanwhile, his friends say they want to try Rust, but they're worried about its async execution issues, and he doesn't know what else to say, other than to offer a sense of understanding. Maybe the situation will get better someday, he hopes.

## 🤔 Frequently Asked Questions


* **What are the morals of the story?**
    * There are often many sources of opinion in the community regarding futures and async, but these opinions aren't always backed up with examples of how it should be better accomplished. Sometimes we just find a thing that works and would prefer to stick with it, but others argue that some traits make implementations unnecessarily complex, and choose to leave it out. Disagreements like these in the ecosystem can be harmful to the reputation of the project and the participants.
    * If there's a source of substantial disagreement, the community becomes even further fragmented, and this may cause additional confusion in newcomers.
    * Alan is used to fragmentation from the communities he comes from, so this isn't too discouraging, but what's difficult is that there's enough functionality overlap in async libraries that it's tempting to get them to interop with each other as-needed, and this can lead to architectural challenges resulting from a difference in design philosophies.
    * It's also unclear if Futures are core to the Rust asynchronous experience, much as Promises are in JavaScript, or if the situation is actually more complex.
    * The `Sink` trait is complex but it solves a real problem, and the workarounds required to solve problems without it can be unsatisfactory.
    * Disagreement about core abstractions like `Sink` can make interoperability between runtimes more difficult; it also makes it harder for people to reproduce patterns they are used to from one runtime to another.
    * It is all too easy for technical discussions like this to become heated; it's important for all participants to try and provide each other with the "benefit of the doubt".
* **What are the sources for this story?**
    * <https://github.com/http-rs/tide-websockets>
        * <https://github.com/http-rs/tide-websockets/pull/17> - Third pull request
        * <https://github.com/http-rs/tide-websockets/issues/15#issuecomment-797090892> - Suggestion to use a broadcast channel
    * <https://github.com/ChainSafe/forest/commit/ff2691bab92823a8595d1d456ed5fa9683641d76#diff-2770a30d9f259666fb470d6f11cf1851ebb2d579a1480a8173d3855572748385> - Where some of the original polling work is replaced
        * <https://github.com/ChainSafe/forest/blob/b9fccde00e7356a5e336665a7e482d4ef439d714/node/rpc/src/rpc_ws_handler.rs#L121> - File with Sink solution
    * <https://github.com/cryptoquick/tide-websockets-sink>
    * <https://twitter.com/cryptoquick/status/1370143022801846275>
    * <https://twitter.com/cryptoquick/status/1370155726056738817>
    * <https://blog.yoshuawuyts.com/rust-streams/#why-we-do-not-talk-about-the-sink-trait>
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
