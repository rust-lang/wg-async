# 😱 Status quo stories: Alan iteratively regresses performance

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

A core part of DistriData, called DDSplit, is in charge of splitting input data records into fragments that are stored on distinct servers, and then reassembling those fragments back into records in response to user queries.

DDSplit was originally implemented using Java code (plus some C, interfaced via JNI). Alan thinks that Rust could provide the same quality of service while requiring less memory. He decides to try reimplementing DDSplit in Rust, atop tokio.

Alan wants to copy some of the abstractions he sees in the Java code that are defined via Java interfaces. Alan sees Rust traits as the closest thing to Java interfaces. However, when he experimentally defines a trait with an `async fn`, he gets the following message from the compiler:

```
error[E0706]: functions in traits cannot be declared `async`
 --> src/main.rs:2:5
  |
2 |     async fn method() { }
  |     -----^^^^^^^^^^^^^^^^
  |     |
  |     `async` because of this
  |
  = note: `async` trait functions are not currently supported
  = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
```

This diagnostic leads Alan to add the [async-trait crate][] as a dependency to his project. Alan then uses the `#[async_trait]` attribute provided by that crate to be able to define `async fn` methods within traits.

When Alan finishes the prototype code, he finds the prototype performance has 20% slower throughput compared to the Java version.

[async-trait crate]: https://crates.io/crates/async-trait
[async-trait transform]: https://crates.io/crates/async-trait#explanation

Alan is disappointed; his experience has been that Rust code performs great, (at least once you managed to get the code to be accepted by the compiler). Alan was not expecting to suffer a 20% performance hit over the Java code.

The DDSplit service is being developed on a Linux machine, so Alan is able use the `perf` tool to gather sampling-based profiling data the async/await port of DDSplit. 

Looking at a [flamegraph][] for the call stacks, Alan identified two sources of execution time overhead that he did not expect: calls into the memory allocator (`malloc`) with about 1% of the execution time, and calls to move values in memory (`memcpy`), with about 8% of execution time.

[flamegraph]: https://crates.io/crates/flamegraph

Alan reaches out to Barbara, as the local Rust expert, for help on how identify where the performance pitfalls are coming from.

Alan asks Barbara whether the problem could be caused by the tokio executor. Barbara says it is hard to know that without more instrumentation. She explains it *could* be that the program is overloading tokio's task scheduler (for example), but it also could be that the application code itself has expensive operations, such as lots of small I/O operations rather than using a buffer.

Alan and Barbara look at the `perf` data. They find the output of `perf report` difficult to navigate and interpret. The data has stack trace fragments available, which gives them a few hints to follow up on. But when they try to make `perf report` annotate the original source, `perf` only shows disassembled machine code, not the original Rust source code. Alan and Barbara both agree that trying to dissect the problem from the machine code is not an attractive strategy.

Alan asks Barbara what she thinks about the `malloc` calls in the profile. Barbara recommends that Alan try to eliminate the allocation calls, and if they cannot be eliminated, then that Alan try tuning the parameters for the global memory allocator, or even switching which global memory allocator he is using. Alan looks at Barbara in despair: his time tweaking GC settings on the Java Virtual Machine taught him that allocator tuning is often a black art.

Barbara suggests that they investigate where the calls to `memcpy` are arising, since they look like a larger source of overhead based on the profile data. From the call stacks in `perf report`, Alan and Barbara decide to skim over the source code files for the corresponding functions.

Upon seeing `#[async_trait]` in Alan's source code, Barbara recommends that if performance is a concern, then Alan should avoid `#[async_trait]`. She explains that `#[async_trait]` [transforms][async-trait transform] a trait's async methods into methods that return `Pin<Box<dyn Future>>`, and the overhead that injects that will be hard to diagnose and impossible to remove. When Alan asks what other options he could adopt, Barbara thinks for a moment, and says he could make an enum that carries all the different implementations of the code. Alan says he'll consider it, but in the meantime he wants to see how far they can improve the code while keeping `#[async_trait]`.

They continue looking at the code itself, essentially guessing at potential sources of where problematic `memcpy`'s may be arising. They identify two potential sources of moves of large datatypes in the code: pushes and pops on vectors of type `Vec<DistriQuery>`, and functions with return types of the form `Result<SuccessCode, DistriErr>`.

Barbara asks how large the `DistriQuery`, `SuccessCode`, and `DistriErr` types are. Alan immediately notes that `DistriQuery` may be large, and they discuss options for avoiding the memory traffic incurred by pushing and popping `DistriQuery`.

For the other two types, Alan responds that the `SuccessCode` is small, and that the error variants are never constructed in his benchmark code. Barbara explains that the size of `Result<T, E>` has to be large enough to hold either variant, and that `memcpy`'ing a result is going to move all of those bytes. Alan investigates and sees that `DistriErr` has variants that embed byte arrays that go up to 50kb in size. Barbara recommends that Alan look into boxing the variants, or the whole `DistriErr` type itself, in order to reduce the cost of moving it around.

Alan uses Barbara's feedback to box some of the data, and this cuts the `memcpy` traffic in the `perf report` to one quarter of what it had been reporting previously.

However, there remains a significant performance delta between the Java version and the Rust version. Alan is not sure his Rust-rewrite attempt is going to get anywhere beyond the prototype stage.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**

1. Rust promises great performance, but when performance is not meeting one's targets, it is hard to know what to do next. Rust mostly leans on leveraging existing tools for native code development, but those tools are (a.) foreign to many of our developers, (b.) do not always measure up to what our developers have access to elsewhere, (c.) do not integrate as well with Rust as they might with C or C++.

2. Lack of certain language features leads developers to use constructs like `#[async_trait]` which add performance overhead that is (a.) hard to understand and (b.) may be significant.

3. Rust makes some things very explicit, e.g. the distinction between `Box<T>` versus `T` is quite prominent. But Rust's expressive type system also makes it easy to compose types without realizing how large they have gotten.

4. Programmers do not always have a good mental model for where expensive moves are coming from.

5. An important specific instance of (1c.) for the async vision: Native code tools do not have any insight into Rust's async model, as that is even more distant from the execution model of C and C++.

6. We can actually generalize (5.) further: When async performance does not match expectations, developers do not have much insight into whether the performance pitfalls arise from issues deep in the async executor that they have selected, or if the problems come directly from overheads built into the code they themselves have written.

### **What are the sources for this story?**

Discussions with engineers at Amazon Web Services.

### **Why did you choose Alan to tell this story?**

I chose Alan because he is used to Java, where these issues play out differently.

Java has very mature tooling, including for performance investigations. Alan is frustrated by his attempts to use (or even identify) equivalent tools for Rust.

With respect to memory traffic: In Java, every object is handled via a reference, and those references are cheap to copy. (One pays for that convenience in other ways, of course.)


### **How would this story have played out differently for the other characters?**

From her C and C++ background, [Grace][] probably would avoid letting her types get so large. But then again, C and C++ do not have enums with a payload, so even Grace may have fallen in the same trap that Alan did (of assuming that the cost of moving an enum value is proportional to its current variant, rather than to its type's overall size).

[Barbara][] probably would have added direct instrumentation via the `tracing` crate, potentially even to tokio itself, rather than spend much time wrestling with `perf`.

[Niklaus][] is unlikely to be as concerned about the 20% throughput hit; he probably would have been happy to get code that seems functionally equivalent to the original Java version.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
