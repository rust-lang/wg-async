# 😱 Status quo stories: Alan Creates a Hanging Alarm

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[Alan] is a developer on the Bottlerocket project.
[Bottlerocket] is a Linux-based open-source operating system that is purpose-built by Amazon Web Services for running containers.
Alan created a rust program, [pubsys], to ensure that Bottlerocket update repositories are healthy.
A _repository verification alarm_ uses pubsys to check the validity of Bottlerocket update repositories and notifies the team if any issues are found.

### Multiple Tokio Runtimes

Bottlerocket uses its own [tough] library to read and write TUF repositories.
This library was created before async became widespread and [reqwest] changed its main interface to `async`.
When reqwest switched to async, Alan used the `reqwest::blocking` feature instead of re-writing tough to be an async interface.
(Maybe Alan [should](https://github.com/awslabs/tough/issues/213) make tough an async interface, but he hasn't yet.)
In order to provide a non-async interface, `reqwest::blocking` creates a tokio runtime so that it can await futures.

In pubsys Alan created some parallel downloading logic while using the above libraries:
Without realizing the danger, he created a tokio runtime in pubsys and used futures/await to do this parallelization, like this:

```rust
for target in targets {
    // use pubsys, which uses reqwest::blocking to get a response body reader
    let mut reader = pubsys_repo.read_target(&target).unwrap();

    // spawn a task in our own tokio runtime that conflicts with reqwest::blocking's runtime
    tasks.push(tokio::spawn(async move {
        io::copy(&mut reader, &mut io::sink()).context(error::TargetDownload {
            target: target.to_string(),
        })
    }));
}
```

Surprisingly, in retrospect, this worked... until it didn't.

Recently Alan discovered that his repository verification alarm was hanging.
Alan discovered this by turning on trace level debugging and noticing that tokio was in an endless loop.
Alan remembered previous development efforts when multiple tokio runtimes caused a panic, but he had never seen a hang for this reason.
Still, he suspected multiple runtimes might be in play and audited to code.
The root cause _was_, in fact, having multiple tokio runtimes, though Alan don't know what change exposed the issue.
(Maybe it was a `cargo update`?)

The fix was to eliminate the need for a tokio runtime in the pubsys code path by doing the parallel downloads in a different way
(first with [threads] for a quick fix, then with a [thread pool]).

[Alan] is surprised and sad since he thought the compiler would help him write safe code.
Instead the compiler was ignorant of his misuse of the de-facto standard Rust async runtime.

### Addendum: Multiple Tokio Major Versions

Alan is also sad that the cargo package manager doesn't understand the de-facto standard runtime's versioning requirements.

Alan had trouble updating to tokio v1 because:
- Having two major versions of the tokio runtime can/will cause problems.
- Cargo does not understand this and allows multiple major versions of tokio.

Ultimately Alan's strategy for this in Bottlerocket is to ensure that only one version of tokio exists in the Cargo.lock.
This requirement delayed his ability to upgrade to tokio v1 and caused him to use a beta version of actix-web since all depenencies need to agree on tokio v1.

### Not Easy to Block-On

When Alan is writing a procedural program, and it is perfectly fine to block, then encountering an async function is problematic.

```rust
fn my_blocking_program() {
    blocking_function_1();
    blocking_function_2();

    // uh oh, now what?
    async_function_1().await
}
```

Uh oh.
Now Alan needs to decide what third-party runtime to use.
Should he create that runtime around main, or should I create it and clean it up around this one function call?
Put differently, should he bubble up async throughout the program even though the program is blocking and procedural (non-async) by nature?

If he uses tokio, and gets it wrong (foot-guns described above), his program may hang or panic at runtime.

In this scenario, Alan would consider this a nicer experience:

```rust
fn my_blocking_program() {
    blocking_function_1();
    blocking_function_2();

    std::thread::block_on({
        async_function_1()
    })
}
```

<!-- links -->

[Bottlerocket]: https://github.com/bottlerocket-os/bottlerocket
[pubsys]: https://github.com/bottlerocket-os/bottlerocket/tree/develop/tools/pubsys
[tough]: https://github.com/awslabs/tough/
[reqwest]: https://github.com/seanmonstar/reqwest
[threads]: https://github.com/bottlerocket-os/bottlerocket/pull/1521/files#diff-7546c95d0732614af12f62ff8c072f8c1061f82945c714daf1dd2962c42921ffL47
[thread pool]: https://github.com/bottlerocket-os/bottlerocket/pull/1564/files

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**

When you use a Rust async runtime, which is unavoidable these days, you *really* need to know what you're doing.

Although the first two of the following points are about tokio, they are really about Rust async since tokio serves as the de-facto `std::runtime` for Rust.

- It is confusing and dangerous that multiple tokio runtimes can panic or hang at program runtime.
- It is challenging that using multiple major versions of tokio (which is allowed by cargo) can fail at runtime.
- It is unfortunate that we need a 3rd party runtime in order to `block_on` a future, even if we are not trying to write async code.

### **What are the sources for this story?**

See the links embedded in the story itself (mostly at the top).

### **Why did you choose *Bottlerocket* to tell this story?**

Bottlerocket is a real-life project that experienced these real-life challenges!
[Alan] is representative of several programmers on the project that have experience with batteries-included languages like Go and Java.

### **How would this story have played out differently for the other characters?**

- [Barbara] would not have made this mistake given her experience.
- [Grace] could have made the same mistake since this issue is very specific to the Rust ecosystem.
- [Niklaus] could have easily made this mistake and might also have had a hard time understanding anything about the runtime or what went wrong.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
