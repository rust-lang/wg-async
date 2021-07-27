# 😱 Status quo stories: Barbara wishes for an easy runtime switch

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from
real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async
Rust programmers face today.

## The story

[Barbara] has been working on an async codebase for the past 5 years. It is extremely mature and
quite large (in the millions of lines of code). They've been relying on tokio as their async runtime
and the codebase makes heavy use of its rich API. It has served them well over the years and they're
very happy with it.

Barbara knows about async-std but has never used it. She has wondered for a while how her
application would work and perform if she had used async-std instead. She decides to test it out by
porting her projects from tokio to async-std.

To their disappointment, they discover many areas, where their choice of runtime permeates the code
base:

* tokio provides variants of helpers macros and types, like `tokio::select!` and `tokio::Mutex`.
  These helpers can be used without the rest of tokio, and there are also alternatives from the
  `futures` crate and elsewhere (albeit with subtle differences).
* tokio uses a custom version of `AsyncRead` and `AsyncWrite` traits which differ from the ones used
  by other parts of the ecosystem.
* The tokio API is needed to create core runtime operations like timers (`tokio::time::sleep`) and
  to launch tasks; there doesn't seem to be a standard way to abstract over those kinds of things in
  a runtime-independent way.
* Some of their dependencies (e.g `hyper` and `reqwest`) are tied to tokio. In some cases, there are
  configuration options or ways to use those dependencies that don't depend on tokio, but there is
  no standard mechanism for that.

These things aren't specific to tokio. There just doesn't seem to be a lot of consensus in the
ecosystem on how to write "runtime-independent" code and in some cases there aren't any great
options available (e.g., spawning tasks).

They investigate the possibility of providing some sort of compatibility layer between tokio
and their new runtime of choice but this turns out to not seem like the right way to go as this
compatibility layer would require too much overhead.

Realizing that the task of porting the entire code base to async-std, will take a lot of effort and
time, Barbara decides to give up. She is very disappointed.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* Using a certain executor often means using a certain run-time ecosystem. This often locks the user
  into that ecosystem.
* Tying yourself to a certain executor means that you are tied to the priorities of that executor.
  You may be happy with the run-time ecosystem, but have special needs that the default executor
  does not provide. If the executor doesn't have an extensibility model, you're stuck. **Note:**
  It is perfectly reasonable for a general purpose executor to not be able or willing to cater for
  specialized needs.
* All of this is made worse by that fact that [run-time agnostic libraries] are difficult and
  sometimes even impossible to write.

### **What are the sources for this story?**

This story is more of a thought experiment than a recounting of a true story. We just asked
logically what would happen if a team working on code base where it was assumed they could use a
specific runtime decides to use a different runtime.

### **Why did you choose Barbara to tell this story?**
The story assumes a Rust programmer that has worked for several years on a large and complex Rust
codebase, so Barbara is the natural choice here.

### **How would this story have played out differently for the other characters?**
It wouldn't. If this story happens them, they're on the same level of Rust expertise as Barbara is.

[Barbara]: ../characters/barbara.md
[run-time agnostic libraries]: https://github.com/rust-lang/wg-async-foundations/issues/45
