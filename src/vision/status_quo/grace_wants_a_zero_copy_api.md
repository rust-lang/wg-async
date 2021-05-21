# 😱 Status quo stories: Grace wants a zero-copy API

[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

## The story

Grace had written lots of operating system code in the past, and up until recently was working on a project using [DPDK](https://www.dpdk.org/) for zero-copy networking. The vast majority of the bugs
that Grace found were related to memory (mis)management, so she is excited for the prospect of trying Rust as part of her new job.

However, Grace has a hard time getting this to work without heavily resorting to `unsafe` constructs. As she evolves her undertanding of Rust, she looks hopefully at the signature of `poll_read`:

```rust
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8]
    ) -> Poll<Result<usize, Error>>
```

She notices that the buffer is always passed to the invocation, but she can't pass it down to the operating system: because of the well-known cancellation problem, the buffer is
not guaranteed to be alive throughout the entire operation. There needs to be at least one copy!

Grace hears from her coworkers that they are all using Tokio anyway. But the Tokio traits, although different from the standard traits, are not much better:

```rust
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>
    ) -> Poll<Result<()>>;
```

There's a specialized type for the buffer, but its management and lifetime are still not suitable for zero-copy I/O.

Grace then came across a famous [blog post](https://boats.gitlab.io/blog/post/io-uring/) from a seasoned developer that mentions
another trait, `AsyncBufRead`, but she immediately identifies two issues with that:

* There is not a similar trait for writes, which suffer from much the same problem
* Grace's team is already using a plethora of convenience traits built upon these base traits, including `AsyncReadExt` and `AsyncBufReadExt`,
  and they all pass a buffer, forcing a copy.

Grace now has no good choices: she can live with the performance penalty of the copies, which lets her down since she how has the feeling she
could do more with C++, or she can come up with her own specialized traits, which will make her work harder to consume by her team.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

* The cancellation problem and buffer lifetimes make it impossible to keep a user-provided buffer alive. That makes zero-copy I/O much harder
than it could be.

### **What are the sources for this story?**

* Personal experience.

### **Why did you choose Grace to tell this story?**

* Grace has experience with C/C++, which is still the de-facto language for very low level things like zero-copy. The author had a similar experience
when trying to expose zero-copy APIs.

### **How would this story have played out differently for the other characters?**

* Zero-copy I/O is an important, but fairly niche use case that requires specialized prior knowledge that usually is only found among system-level
  programmers.
* That is usually done in C/C++, and Grace is the only one that is very likely to have this experience.
* There is a chance Barbara would have ventured into similar problems. She would likely have had a similar experience than Grace.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
