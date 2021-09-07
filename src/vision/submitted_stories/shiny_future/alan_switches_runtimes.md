# ✨ Shiny future stories: Alan switches runtimes

[How To Vision: Shiny Future]: ../shiny_future.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async-foundations/master/src/vision/shiny_future/template.md
[`shiny_future`]: https://github.com/rust-lang/wg-async-foundations/tree/master/src/vision/shiny_future
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async-foundations/blob/master/src/SUMMARY.md

## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

Since his [early adventures](./alans_trust_in_the_compiler_is_rewarded.md) with Async I/O went so well, Alan has been looking for a way to learn more. He finds a job working in Rust. One of the projects he works on is [DistriData]. Looking at their code, he sees an annotation he has never seen before:

```rust
#[humboldt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = std::async_thread::spawn(async move {
        do_something()
    });
}
```

He asks Barbara, one of his coworkers, "What is this `humboldt::main` annotation? What's `humboldt`?" She answers by explaining to him that Rust's support for async I/O is actually based around an underlying runtime. "Rust gives you a pretty decent runtime by default," she says, "but it's not tuned for our workloads. We wrote our own runtime, which we call `humboldt`."

Alan asks, "What happens with the various std APIs? For example, I see we are calling `std::async_thread::spawn` -- when I used that before, it spawned tasks into the default runtime. What happens now?"

Barbara explains that the "async" APIs in std generally execute relative to the current runtime that is in use. "When you call `std::async_thread::spawn`, it will spawn a task onto the current runtime. It's the same with the routines in `std::async_io` and so forth. The `humboldt::main` annotation actually just creates a synchronous `main` function that initializes the `humboldt` runtime and launches the first future. When you just write an `async fn main` without any annotation, the compiler synthesizes the same `main` function with the default runtime."



## Learning more about Humboldt

Alan sees that some of the networking code that is being used in their application is creating network connections using humboldt APIs:

```rust
use humboldt::network;
```

He asks Barbara, "Why don't we use the `std::async_io` APIs for that?" She explains that Humboldt makes use of some custom kernel extensions that, naturally enough, aren't part of the std library. "TCP is for rubes," she says, "we are using TTCP -- Turbo TCP." Her mind wanders briefly to [Turbo Pascal] and she has a brief moment of yearning for the days when computers had a "Turbo" button that changed them from 8 MHz to 12 MHz. She snaps back into the present day. "Anyway, the `std::async_io` APIs just call into humboldt's APIs via various traits. But we can code directly against `humboldt` when we want to access the extra capabilities it offers. That *does* make it harder to change to another runtime later, though."

[Turbo Pascal]: https://en.wikipedia.org/wiki/Turbo_Pascal

## Integrating into other event loops

Later on, Alan is working on a visualizer front-end that integrates with [DistriData] to give more details about their workloads. To do it, he needs to integrate with Cocoa APIs and he wants to run certain tasks on [Grand Central Dispatch]. He approaches Barbara and asks, "If everything is running on `humboldt`, is there a way for me to run some things on another event loop? How does that work?"

[Grand Central Dispatch]: https://en.wikipedia.org/wiki/Grand_Central_Dispatch

Barbara explains, "That's easy. You just have to use the `gcd` wrapper crate -- you can find it on `crates.io`. It implements the runtime traits for `gcd` and it has a `spawn` method. Once you spawn your task onto `gcd`, everything you run within `gcd` will be running in that context."

Alan says, "And so, if I want to get things running on `humboldt` again, I spawn a task back on `humboldt`?"

"Exactly," says Barbara. "Humboldt has a global event loop, so you can do that by just doing `humboldt::spawn`. You can also just use the `humboldt::io` APIs directly. They will always use the Humboldt I/O threads, rather than using the current runtime."

Alan winds up with some code that looks like this:

```rust
async fn do_something_on_humboldt() {
    gcd::spawn(async move {
        let foo = do_something_on_gcd();

        let bar = humboldt::spawn(async move {
            do_a_little_bit_of_stuff_on_humboldt();
        });

        combine(foo.await, bar.await);
    });
}
```

## 🤔 Frequently Asked Questions

### What status quo story or stories are you retelling?

Good question! I'm not entirely sure! I have to go looking and think about it. Maybe we'll have to write some more.

### What are the key points you were trying to convey with this status quo story?

* There is some way to seamlessly change to a different default runtime to use for `async fn main`.
* There is no global runtime, just the current runtime.
* When you are using this different runtime, you can write code that is hard-coded to it and which exposes additional capabilities.
* You can integrate multiple runtimes relatively easily, and the std APIs work with whichever is the current runtime.

### How do you imagine the std APIs and so forth know the current runtime?

I was imagining that we would add fields to the `Context<'_>` struct that is supplied to each `async fn` when it runs. Users don't have direct access to this struct, but the compiler does. If the std APIs return futures, they would gain access to it that way as well. If not, we'd have to create some other mechanism.

### What happens for runtimes that don't support all the features that std supports?

That feels like a portability question. See the (yet to be written) sequel story, "Alan runs some things on WebAssembly". =)

### **What is [Alan] most excited about in this future? Is he disappointed by anything?**

Alan is excited about how easy it is to get async programs up and running, and he finds that they perform pretty well once he does so, so he's happy.

### **What is [Grace] most excited about in this future? Is she disappointed by anything?**

Grace is concerned with memory safety and being able to deploy her tricks she knows from other languages. Memory safety works fine here. In terms of tricks she knows and loves, she's happy that she can easily switch to another runtime. The default runtime is good and works well for most things, but for the [`DistriData`] project, they really need something tailored just for them. She is also happy she can use the extended APIs offered by `humboldt`.

### **What is [Niklaus] most excited about in this future? Is he disappointed by anything?**

Niklaus finds it async Rust quite accessible, for the same reasons cited as in ["Alan's Trust in the Rust Compiler is Rewarded"].

["Alan's Trust in the Rust Compiler is Rewarded"]: ./alans_trust_in_the_compiler_is_rewarded.md

### **What is [Barbara] most excited about in this future? Is she disappointed by anything?**

Depending on the technical details, Barbara may be a bit disappointed by the details of how std interfaces with the runtimes, as that may introduce some amount of overhead. This may not matter in practice, but it could also lead to library authors avoiding the std APIs in favor of writing generics or other mechanisms that are "zero overhead".

### **What [projects] benefit the most from this future?**

Projects like [DistriData] really benefit from being able to customize their runtime.

### **Are there any [projects] that are hindered by this future?**

We have to pay careful attention to embedded projects like [MonsterMesh]. Some of the most obvious ways to implement this future would lean on `dyn` types and perhaps boxing, and that would rule out some embedded projects. Embedded runtimes like [embassy] are also the most different in their overall design and they would have the hardest time fitting into the std APIs (of course, many embedded projects are already no-std, but many of them make use of some subset of the std capabilities through the facade). In general, traits and generic functions in std could lead to larger code size, as well.

[embassy]: https://github.com/akiles/embassy

### **What are the incremental steps towards realizing this shiny future?**

There are a few steps required to realize this future:

* We have to determine the core mechanism that is used for std types to interface with the current scheduler. 
    * Is it based on dynamic dispatch? Delayed linking? Some other tricks we have yet to invent?
    * Depending on the details, language changes may be required. 
* We have to hammer out the set of traits or other interfaces used to define the parts of a runtime (see below for some of the considerations).
    * We can start with easier cases and proceed to more difficult ones, however.

### **Does realizing this future require cooperation between many projects?**

Yes. We will need to collaborate to define traits that std can use to interface with each runtime, and the runtimes will need to implement those traits. This is going to be non-trivial, because we want to preserve the ability for independent runtimes to experiment, while also preserving the ability to "max and match" and re-use components. For example, it'd probably be useful to have a bunch of shared I/O infrastructure, or to have utility crates for locks, for running threadpools, and the like. On the other hand, tokio takes advantage of the fact that it owns the I/O types *and* the locks *and* the scheduler to do some [nifty tricks](https://tokio.rs/blog/2020-04-preemption) and we would want to ensure that remains an option.


[character]: ../../characters.md
[comment]: ../../how_to_vision/comment.md
[status quo stories]: ../status_quo.md
[Alan]: ../../characters/alan.md
[Grace]: ../../characters/grace.md
[Niklaus]: ../../characters/niklaus.md
[Barbara]: ../../characters/barbara.md
[projects]: ../../projects.md
[htvsq]: ../shiny_future.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
[DistriData]: ../../projects/DistriData.md
[MonsterMesh]: ../../projects/MonsterMesh.md
