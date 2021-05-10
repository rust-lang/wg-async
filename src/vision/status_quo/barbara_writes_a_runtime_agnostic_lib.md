# 😱 Status quo stories: Barbara writes a runtime-agnostic library


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from
real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async
Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR
making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories
[cannot be wrong], only inaccurate). Alternatively, you may wish to
[add your own status quo story][htvsq]!

## The story

Barbara and Alan work at AmoolgeSoft, where many teams are switching from Java to Rust. These teams
have many different use cases and various adoption stories. Some teams are happy users of tokio,
others happy users of async-std, and others still are using custom runtimes for highly specialized
use cases.

Barbara is tasked with writing a library for a custom protocol, SLOW (only in use at AmoogleSoft)
and enlists the help of Alan in doing so. Alan is already aware that [not all libraries in Rust work
with all runtimes][nalirwwar]. Alan and Barbara start by writing a parser which works on
`std::io::Read` and get their tests working with `String`s. After this they contemplate the question
of how to accept a TCP connection.

### Incompatible `AsyncRead` traits

Alan asks Barbara what is the async equivalent is of `std::io::Read`, and Barbara sighs and says
that there isn't one. Barbara brings up tokio's and the [futures crate]'s versions of `AsyncRead`.
Barbara decides not to talk about `AsyncBufRead` for now.

Barbara and Alan decide to use the future's `AsyncRead` for no other reason other than it is
runtime-agnostic. Barbara tells Alan not to worry as they can translate between the two. With
[some](ahwas) [effort](bnah) they convert their parser to using `AsyncRead`.

Alan, excited about the progress they've made, starts working on hooking this up to actual TCP
streams. Alan looks at async-std and tokio and notices their interfaces for TCP are quite different.
Alan waits for Barbara to save the day.

Barbara helps abstract over TCP listener and TCP stream (**TODO:** code example). One big hurdle is
that tokio uses `AsyncRead` from their own crate and not the one from `futures` crate.

### Task spawning

After getting the TCP handling part working, they now want to spawn tasks for handling each incoming
TCP connection. Again, to their disappointment, they find that there's no runtime-agnostic way to do
that.

Unsure on how to do this, they do some searching and find the [`agnostik`] crate. They reject it
because this only supports N number of runtimes and their custom runtime is not one of them.
However it gives them the idea to provide a trait for specifying how to spawn tasks on the runtime.
Barbara points out that this has disadvantage of [working against orphan rules] meaning that either
they have to implement the trait for all known runtimes (defeating the purpose of the exercise) or
force the user to use new types.

They punt on this question by implementing the trait for each of the known runtimes. They're
disappointed that this means their library actually isn't runtime agnostic.

### The need for timers

To make things further complicated, they also are in need for a timer API. They could abstract
runtime-specific timer APIs in their existing trait they use for spawning, but they find a
runtime-agnostic library. It works but is pretty heavy in that it spawns an OS thread (from a pool)
every time they want to sleep. They become sadder.

### Channels

They need channels as well but after long searches and discussions on help channels, they learn of
a few runtime-agnostic implementations: `async-channel`, `futures-channel`, and trimmed down (
through feature flags) `async-std`/`tokio`. They pick one and it seems to work well. They become
less sadder.

### First release

They get things working but it was a difficult journey to get to the first release. Some of their
users find the APIs harder to use than their runtime-specific libs.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **Why did you choose Barbara to tell this story?**
[Barbara] has years of rust experience that she brings to bear in her async learning experiences.

### **What are the morals of the story?**

* People have to roll their own implementations which can lead to often subtle differences between
  runtimes (For example TCPListeners in `async-std` and `tokio`).
* Orphan rules and no standard traits guarantee that a truly agnostic library is not possible.
* Takes way more time than writing synchronous protocols.
* It's a hard goal to achieve.
* Leads to poorer APIs sometimes (both in ease of use and **performance**).
* More API design considerations need to go into making an generic async library than a generic sync library.

### **What are the sources for this story?**
Personal experiences of the author from adding async API in [`zbus`] crate, except for `AsyncRead`,
which is based on common knowledge in async Rust community.

### **How would this story have played out differently for the other characters?**
Alan, Grace, and Niklaus would be overwhelmed and will likely want to give up.

### What are other related stories?

**TODO:**

### What are the downside of using runtime agnostic crates?

Some things can be implemented very efficiently in a runtime-agnostic way but even then you can't
integrate deeply into the runtime. For example, see tokio’s pre-emption strategy, which relies on
deep integration with the runtime.

### What other runtime utilities are generally needed?
* [async-locks][async-locks-story]

[status quo stories]: ./status_quo.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[ahwas]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_hates_writing_a_stream.html
[bnah]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo/barbara_needs_async_helpers.html
[working against orphan rules]: https://github.com/rust-lang/wg-async-foundations/issues/180
[futures crate]: https://crates.io/crates/futures
[nalirwwar]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_picks_web_server.html#first-problem-incompatible-runtimes
[`agnostik`]: https://crates.io/crates/agnostik
[`zbus`]: https://crates.io/crates/zbus/2.0.0-beta.3
[async-locks-story]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_thinks_he_needs_async_locks.html
