# 😱 Status quo stories: Grace wants to integrate a C-API

[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md

[bindgen]: //docs.rs/bindgen/
[`stream::unfold`]: //docs.rs/futures/0.1.17/futures/stream/fn.unfold.html

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life
experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

## The story

[Grace] is integrating a camera into an embedded project. Grace has done similar projects before in the past, and has
even used this particular hardware before. Fortunately, the camera manufacturer provides a library in C to interface
with the driver.

Grace knows that Rust provides strong memory safety guarantees, and the library provided by the manufacturer sports an
API that is easy to misuse. In particular, ownership concerns are tricky and Grace and her team have often complained in
the past that making memory mistakes is very easy and one has to be extremely careful to manage lifetimes. Therefore,
for this project, Grace opts to start with Rust as many of the pitfalls of the manufacturer's library can be
automatically caught by embedding the lifetimes into a lightweight wrapper over code bridged into Rust with [bindgen].

Grace's team manages to write a thin Rust wrapper over the manufacturer's library with little complication. This library
fortunately offers two interfaces for grabbing frames from the camera: a blocking interface that waits for the next
frame, and a non-blocking interface that polls to check if there are any frames currently available and waiting. Grace
is tempted to write a callback-based architecture by relying on the blocking interface that waits; however, early the
next morning the customer comes back and informs her that they are scaling up the system, and that there will now be 5
cameras instead of 1.

She knows from experience that she cannot rely on having 5 threads blocking just for getting camera frames, because the
embedded system she is deploying to only has 2 cores total! Her team would be introducing a lot of overhead into the
system with the continuous context switching of every thread. Some folks were unsure of Rust's asynchronous
capabilities, and with the requirements changing there were some that argued maybe they should stick to the tried and
true in pure C. However, Grace eventually convinced them that the benefits of memory safety were still applicable, and
that a lot of bugs that have taken weeks to diagnose in the past have already been completely wiped out. The team
decided to stick with Rust, and dig deeper into implementing this project in async Rust.

Fortunately, Grace notices the similarities between the polling interface in the underlying C library and the `Poll`
type returned by Rust's `Future` trait. "Surely," she thinks, "I can asynchronously interleave polls to each camera over
a single thread, and process frames as they become available!" Such a thing would be quite difficult in C while
guaranteeing memory safety was maintained. However, Grace's team has already dodged that bullet thanks to writing a thin
wrapper in Rust that manages these tricky lifetimes!

### The first problem: polls and wake-ups

Grace sets out to start writing the pipeline to get frames from the cameras. She realizes that while the polling call
that the manufacturer provided in their library is similar in nature to a future, it doesn't quite encompass everything.
In C, one might have to set some kind of heartbeat timer for polling. Grace explains to her team that this heartbeat is
similar to how the `Waker` object works in a `Future`'s `Context` type, in that it is how often the execution
environment should re-try the future if the call to `poll` returns `Poll::Pending`.

A member of Grace's team asks her how she was able to understand all this. After all, Grace had been writing Rust about
as long as the rest of her team. The main difference was that she had many more years of systems programming under C and
C++ under her belt than they had. Grace responded that for the most part she had just read the documentation for the
`Future` trait, and that she had intuited how async-await de-sugars itself into a regular function that returns a future
of some kind. The de-sugaring process was, after all, very similar to how lambda objects in C++ were de-sugared as well.
She leaves her teammate with [an
article](//smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/) she once found online that
explained the process in a lot more detail for a problem much harder than they were trying to solve.

Something Grace and her team learn to love immediately about Rust is that writing the `Future` here does not require her
team to write their own execution environment. In fact, the future can be entirely written independently of the
execution environment. She quickly writes an async method to represent the polling process:

```rust,ignore
/// Gets the next frame from the camera, waiting `retry_after` time until polling again if it fails.
///
/// Returns Some(frame) if a frame is found, or None if the camera is disconnected or goes down before a frame is
/// available.
async fn next_frame(camera: &Camera, retry_after: Duration) -> Option<Frame> {
    while camera.is_available() {
        if let Some(frame) = camera.poll() {
            return Some(frame);
        } else {
            task::sleep_for(retry_after).await;
        }
    }

    None
}
```ignore

The underlying C API doesn't provide any hooks that can be used to wake the `Waker` object on this future up, so Grace
and her team decide that it is probably best if they just choose a sufficiently balanced `retry_after` period in which
to try again. It does feel somewhat unsatisfying, as calling `sleep_for` feels about as hacky as calling
`std::this_thread::sleep_for` in C++. However, there is no way to directly interoperate with the waker without having a
separate thread of execution wake it up, and the underlying C library doesn't have any interface offering a notification
for when that should be. In the end, this is the same kind of code that they would write in C, just without having to
implement a custom execution loop themselves, so the team decides it is not a total loss.

### The second problem: doing this many times

Doing this a single time is fine, but an end goal of the project is to be able to stream frames from the camera for
unspecified lengths of time. Grace spends some time searching, and realizes that what she actually wants is a `Stream`
of some kind. `Stream` objects are the asynchronous equivalent of iterators, and her team wants to eventually write
something akin to:

```rust,ignore
let frame_stream = stream_from_camera(camera, Duration::from_millis(5));

while let Some(frame) = frame_stream.next().await {
    // process frames
}

println!("Frame stream closed.");
```ignore

She scours existing crates, in particular looking for one way to transform the above future into a stream that can be
executed many times. The only available option to transform a future into a series of futures is [`stream::unfold`],
which seems to do exactly what Grace is looking for. Grace begins by adding a small intermediate type, and then plugging
in the remaining holes:

```rust,ignore
struct StreamState {
    camera: Camera,
    retry_after: Duration,
}

fn stream_from_camera(camera: Camera, retry_after: Duration) -> Unfold<Frame, ??, ??> {
    let initial_state = StreamState { camera, retry_after };

    stream::unfold(initial_state, |state| async move {
        let frame = next_frame(&state.camera, state.retry_after).await
        (frame, state)
    })
}
```ignore

This looks like it mostly hits the mark, but Grace is left with a couple of questions for how to get the remainder of
this building:

1. What is the type that fills in the third template parameter in the return? It should be the type of the future that
   is returned by the async closure passed into `stream::unfold`, but we don't know the type of a closure!
2. What is the type that fills in the second template parameter of the closure in the return?

Grace spends a lot of time trying to figure out how she might find those types! She asks [Barbara] what the idiomatic
way to get around this in Rust would be. Barbara explains again how closures don't have concrete types, and that the
only way to do this will be to use the `impl` keyword.

```rust,ignore
fn stream_from_camera(camera: Camera, retry_after: Duration) -> impl Stream<Item = Frame> {
    // same as before
}
```ignore

While Grace was was on the correct path and now her team is able to write the code they want to, she realizes that
sometimes writing the types out explicitly can be very hard. She reflects on what it would have taken to write the type
of an equivalent function pointer in C, and slightly laments that Rust cannot express such as clearly.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* Rust was the correct choice for the team across the board thanks to its memory safety and ownership. The
  underlying C library was just too complex for any single programmer to be able to maintain in their head all at
  once while also trying to accomplish other tasks.
* Evolving requirements meant that the team would have had to either start over in plain C, giving up a lot of the
  safety they would gain from switching to Rust, or exploring async code in a more rigorous way.
* The async code is actually much simpler than writing the entire execution loop in C themselves. However, the
  assumption that you would write the entire execution loop is baked into the underlying library which Grace's team
  cannot rewrite entirely from scratch. Integrating Rust async code with other languages which might have different
  mental models can sometimes lead to unidiomatic or unsatisfying code, even if the intent of the code in Rust is
  clear.
* Grace eventually discovered that the problem was best modeled as a stream, rather than as a single future.
  However, converting a future into a stream was not necessarily something that was obvious for someone with a C/C++
  background.
* Closures and related types can be very hard to write in Rust, and if you are used to being very explicit with your
  types, tricks such as the `impl` trick above for `Stream`s aren't immediately obvious at first glance.

### **What are the sources for this story?**
My own personal experience trying to incorporate the Intel RealSense library into Rust.

### **Why did you choose Grace to tell this story?**
* I am a C++ programmer who has written many event / callback based systems for streaming from custom camera
  hardware. I mirror Grace in that I am used to using other systems languages, and even rely on libraries in those
  languages as I've moved to Rust. I did not want to give up the memory and lifetime benefits of Rust because of
  evolving runtime requirements.
* In particular, C and C++ do not encourage async-style code, and often involve threads heavily. However, some
  contexts cannot make effective use of threads. In such cases, C and C++ programmers are often oriented towards
  writing custom execution loops and writing a lot of logic to do so. Grace discovered the benefit of not having to
  choose an executor upfront, because the async primitives let her express most of the logic without relying on a
  particular executor's behaviour.

### **How would this story have played out differently for the other characters?**
* [Alan] would have struggled with understanding the embedded context of the problem, where GC'd languages don't see
  much use.
* [Niklaus] and [Barbara] may not have approached the problem with the same assimilation biases from C and C++ as
  Grace. Some of the revelations in the story such as discovering that Grace's team didn't have to write their own
  execution loop were unexpected benefits when starting down the path of using Rust!

### **Could Grace have used another runtime to achieve the same objectives?**
Grace can use _any_ runtime, which was an unexpected benefit of her work!

### **How did Grace know to use `Unfold` as the return type in the first place?**
She saw it in the [rustdoc](https://docs.rs/futures/0.3.13/futures/stream/fn.unfold.html) for `stream::unfold`.
