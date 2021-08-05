# Barbara begets backpressure and benchmarks async_trait

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

*Write your story here! Feel free to add subsections, citations, links, code examples, whatever you think is best.*

Barbara is implementing the network stack for an experimental new operating system in Rust. She loves Rust's combination of performance, expressiveness, and safety. She and her team set off implementing the network protocols, using traits to separate protocol layers, break up the work, and make them testable.

Unlike most operating systems, this network stack is designed to live in a _separate process_ from the driver itself. Barbara eventually realizes a problem: this system architecture will require modeling backpressure explicitly when sending outbound packets.

She starts looking into how to model backpressure without having to rewrite all of her team's code.
She realizes that async is actually the perfect model for expressing backpressure implicitly. By using async, she can keep most of her code without explicitly propagating backpressure information.

When she sets off to implement this, Barbara quickly realizes async won't work off the shelf because of the lack of support for `async fn` in traits.

Barbara is stuck. She has a large codebase that she would like to convert to using async, but core features of the language she was using are not available with async. She starts looking for workarounds.

Barbara begins by writing out requirements for her use case. She needs to

- Continue using trait abstractions for core protocol implementations
- Take advantage of the backpressure model implied by async
- Maintain performance target of at most 4 µs per packet on underpowered hardware

The last requirement is important for sustaining gigabit speeds, a key goal of the network stack and one reason why Rust was chosen.

Barbara thinks about writing down the name of each Future type, but realizes that this wouldn't work with the `async` keyword. Using Future combinators directly would be extremely verbose and painful.

Barbara finds the `async_trait` crate. Given her performance constraints, she is wary of the allocations and dynamic dispatch introduced by the crate.

She decides to write a benchmark to simulate the performance impact of `async_trait` compared to a future where `async fn` is fully supported in traits. Looking at the `async_trait` documentation, she sees that it desugars code like

```rust
#[async_trait]
impl Trait for Foo {
    async fn run(&self) {
        // ...
    }
}
```

to

```rust
impl Trait for Foo {
    fn run<'a>(
        &'a self,
    ) -> Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>>
    where
        Self: Sync + 'a,
    {
        async fn run(_self: &Foo) {
            // original body
        }
        Box::pin(run(self))
    }
}
```

The benchmark Barbara uses constructs a tree of Futures 5 levels deep, using both `async` blocks and a manual desugaring similar to above. She runs the benchmark on hardware that is representative for her use case and finds that while executing a single native async future takes 639 ns, the manual desugaring using `boxed` takes 1.82 µs.

Barbara sees that in a real codebase, this performance would not be good enough for writing a network stack capable of sustaining gigabit-level throughput on underpowered hardware. Barbara is disappointed, but knows that support for `async fn` in traits is in the works.

Barbara looks at her organization's current priorities and decides that 100's of mbps will be an acceptable level of performance for the near term. She decides to adopt `async_trait` with the expectation that the performance penalty will go away in the long term.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**
*Talk about the major takeaways-- what do you see as the biggest problems.*

* Language features that don't work well together can be a major roadblock in the course of development. Developers expect all of a language's features to be at their disposal, not using one to cut them off from using another.
* Allocation and dynamic dispatch aren't acceptable runtime performance costs for all use cases.

### **What are the sources for this story?**
*Talk about what the story is based on, ideally with links to blog posts, tweets, or other evidence.*

This story is based on actual experience implementing the 3rd-generation network stack for the Fuchsia operating system.

The benchmarks are [implemented here](https://fxrev.dev/528302).

### **Why do you need to model backpressure?**

The Linux network stack doesn't do this; instead it drops packets as hardware buffers fill up.

Because our network stack lives in a separate process from the driver, paying attention to hardware queue depth directly is not an option. There is a communication channel of bounded depth between the network stack and the driver. Dropping packets when this channel fills up would result in an unacceptable level of packet loss. Instead, the network stack must "push" this backpressure up to the applications using the network. This means each layer of the system has to be aware of backpressure.

### **How would you solve this in other systems languages?**

In C++ we would probably model this using callbacks which are passed all the way down the stack (through each leayer of the system).

### **What's nice about async when modelling backpressure?**

Futures present a uniform mechanism for communicating backpressure through polling. When requests stack up but their handler futures are not being polled, this indicates backpressure. Using this model means we get backpressure "for free" by simply adding `async` and `.await` to our code, at least in theory.

Async is a viral concern in a codebase, but so is backpressure. You can't have a backpressure aware system when one layer of that system isn't made aware of backpressure in some way. So in this case it's actually _helpful_ that there's not an easy way to call an async fn from a sync fn; if there were, we might accidentally "break the chain" of backpressure awareness.

### **What was the benchmarking methodology?**

A macro was used to generate 512 slightly different versions of the same code, to defeat the branch predictor. Each version varied slightly to prevent LLVM from merging duplicate code.

The leaf futures in the benchmark always returned `Poll::Ready`. The call depth was always 5 async functions deep.

### **Did you learn anything else from the benchmarks?**

In one of the benchmarks we compared the `async fn` version to the equivalent synchronous code. This helps us see the impact of the state machine transformation on performance.

The results: synchronous code took 311.39 ns while the `async fn` code took 433.40 ns.

### **Why did you choose Barbara to tell this story?**
*Talk about the character you used for the story and why.*

The implementation work in this story was done by [@joshlf], an experienced Rust developer who was new to async.

### **How would this story have played out differently for the other characters?**
*In some cases, there are problems that only occur for people from specific backgrounds, or which play out differently. This question can be used to highlight that.*

Alan might not have done the benchmarking up front, leading to a surprise later on when the performance wasn't up to par with Rust's promise. Grace might have decided to implement async state machines manually, giving up on the expressiveness of async.

[@joshlf]: https://github.com/joshlf
[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
