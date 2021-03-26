# 😱 Status quo stories: Carrie tries using a socket Sink

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Carrie is working on a project that uses an alternative popular async framework. She finds the new async framework less familiar, and as she reads through their GitHub issues, she's saddened by the project maintainer's unwelcoming attitude towards their community. She asks her teammates if they can switch to the one she's more familiar with and feels more welcome in, but the teammates are understandably resistant to changing something that important just for one feature. So, she works hard to find workarounds to accomplish her team's goals.

One of the goals is to switch from a WebSocket implementation using raw TCP sockets to one managed behind an HTTP server library, so both HTTP and WebSocket commands can be forwarded to a transport-agnostic RPC server. She finds an HTTP server that's similar to one she's used to using with the other async framework, and a WebSocket middleware library that goes with it.

However, as she's working, Carrie encounters a situation where the socket needs to be written to within an async thread, and the traits just aren't working. She wants to do split the stream into a sender and receiver:

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
        Ok(_) => {
            info!("New WS data sent.");
        }
        Err(_) => {
            warn!("WS connection closed.");
        }
    };
}
```

This is necessary because both sides of the stream are stateful Iterables, if they're kept as one, the lock on the receiver couldn't be released within the loop. She's seen this pattern used in other projects in the Rust community, but is frustrated to find that the Sink trait wasn't implemented in the WebSockets middleware library she's using.

Carrie also tries creating a sort of poller worker thread using an intermediary messaging channel, but she has trouble reasoning about the code and wasn't able to get it to compile:

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
            Ok(msg) => {
                info!("New WS data sent. {:?}", msg);
            }
            Err(msg) => {
                warn!("WS connection closed. {:?}", msg);
            }
        };
    }
});

while let Some(msg) = ws_stream.lock().await.next().await {
    // Echo the request?
    ws_sender.send(msg);
}
```

Carrie wonders if she's thinking about it wrong, but the solution isn't as obvious as her earlier Sink approach. Looking around, she realizes a solution to her problems already exists-- as others have been in her shoes before-- within two other nearly-identical pull requests, but they were both closed by the project maintainers. She tries opening a third one with the same code, pointing to an example where it was actually found to be useful. To her joy, her original approach works with the code in the closed pull requests in her local copy! Carrie's branch is able to compile for the first time.

However, almost immediately, her request was also shut down, and the maintainer suggested she try the complex, unobvious, and unspecific solution that she had already tried and just couldn't get it to work.

As a result of her frustration, Carrie calls out one of the developers of the project on social media, who she believed had frequently shot down the idea of using Sink traits on projects others needed to use them on. This is not well-received, the maintainer reacts with a condescending and dismissive attitude, and she later finds out she was blocked. A co-maintainer responds to the thread, defending and supporting the other maintainer's actions, and she's told to "get over it". She's given a link to a blog post with a piece [lamenting the popularity of Sink in the Rust ecosystem](https://blog.yoshuawuyts.com/rust-streams/#why-we-do-not-talk-about-the-sink-trait), and how it's complex and bad and not worth it, but the piece also unhelpfully makes no effort to provide examples of how the better alternatives would be used to replace uses of Sink.

Because of this heated exchange, Carrie grows concerned for her own career, what these well-known community members might think or say about her to others, and her confidence in the community surrounding this language that she really enjoys using is somewhat shaken.

Despite this, Carrie takes a walk, gathers her determination, and commits to maintaining her fork with the changes from the other pull requests that were shut down, and publishes her version to Crates.io, vowing to be more welcoming to "misfit" pull requests like the one she needed.

A few weeks later, Carrie's work at her project at work is merged with her new forked crate. It's a big deal, her first professional open source contribution to a Rust project, but she still doesn't feel like she has a sense of closure with the community. Meanwhile, her friends say they want to try Rust, but they're worried about its async execution issues, and she doesn't know what else to say, other than to offer a sense of understanding. Maybe the situation will get better someday, she hopes.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

* **What are the morals of the story?**
    * *Talk about the major takeaways-- what do you see as the biggest problems.*
* **What are the sources for this story?**
    * *Talk about what the story is based on, ideally with links to blog posts, tweets, or other evidence.*
* **Why did you choose *NAME* to tell this story?**
    * *Talk about the character you used for the story and why.*
* **How would this story have played out differently for the other characters?**
    * *In some cases, there are problems that only occur for people from specific backgrounds, or which play out differently. This question can be used to highlight that.*

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
