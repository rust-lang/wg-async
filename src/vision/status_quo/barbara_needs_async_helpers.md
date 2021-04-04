# 😱 Status quo stories: Barbara needs Async Helpers

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[Barbara], an experienced Rust user, is prototyping an async Rust service for work. To get things working quickly, she decides to prototype in tokio, since it is unclear which runtime her work will use.

She starts adding warp and tokio to her dependencies list. She notices that warp suggests using tokio with the `full` feature. She's a bit concerned about how this might affect the compile times and also that *all* of tokio is needed for her little project, but she pushes forward.

As she builds out functionality, she's pleased to see tokio provides a bunch of helpers like `join!` and async versions of the standard library types like channels and mutexes.

After completing one endpoint, she moves to a new one which requires streaming http responses to the client. Barbara quickly finds out that tokio does not provide a stream type, and so she adds `tokio-stream` to her dependencies.

Moving on she tries to make some functions generic over the web framework underneath, so she tries to abstract off the functionality to a trait. But Rust doesn't support async functions in traits yet, so she adds `async_trait` to her dependencies.

Some of her functions are recursive, so to make them async she starts boxing her futures. Then she learns about `async-recursion` and then adds it to the dependencies.

Her stream implementation needed `pin_project` so she brings that also as a dependency.

"Finally!", Barbara says, breathing a sigh of relief. She is done with her prototype, and shows it off at work, but to her dismay, the team decides that tokio is not an appropriate runtime for their use case. Barbara's heart skips a beat. "Oh no, what to do now," she thinks.

So now Barbara starts the journey of replacing tokio with a myriad of off the shelf and custom helpers. She can't use warp so now she has to find an alternative. She also has to find a new channel implementations and there are a few:
* In `futures`
* `async-std` has one, but it seems to be tied to another runtime so she can't use that.
* `smol` has one that is independent.

This process of "figure out which alternative is an option" is repeated many times. She also tries to use the `select!` macro from `futures` but it requires more pinning and workarounds (and then her stack overflows).

But Barbara fights through all of it. In the end, she gets it to work, but she realizes that she has a ton of random dependencies and associated compilation time. She wonders if all that dependencies will have a negative effect on the binary size. She also had to rewrite some bits of functionality on her own.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* Functionality is found either in "framework"-like crates (e.g., tokio) *and* spread around many different ecosystem crates.
* It's sometimes difficult to discover where this functionality lives.
* Additionally, the trouble of non runtime-agnostic libraries becomes very apparent.
* Helpers and utilities might have analogues across the ecosystem, but they are different in subtle ways.
* Some patterns are clean if you know the right utility crate and very painful otherwise.

### **What are the sources for this story?**
[Issue 105](https://github.com/rust-lang/wg-async-foundations/issues/105)

### **What are helper functions/macros?**
They are functions/macros that helps with certain basic pieces of functionality and features. Like to await on multiple futures concurrently (`join!` in tokio), or else race the futures and take the result of the one that finishes first.

### **Why did you choose [Barbara] to tell this story?**
This particular issue impacts all users of Rust even (and sometimes especially) experienced ones.

### **How would this story have played out differently for the other characters?**
Other characters may not know all their options and hence might have fewer problems as a result.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
