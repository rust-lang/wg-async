# 😱 Status quo stories: Alan hates writing a `Stream`

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[Alan][] is used to writing web server applications using async sockets, but wants to try Rust to get that signature vroom vroom.

After a couple weeks learning Rust basics, Alan quickly understands `async` and `await`, and therefore has several routes built for his application that await a few things and then construct an HTTP response and send a buffered body. To build the buffered response bodies, Alan was reading a file, and then appending a signature, and putting that all into a single buffer of bytes.

Eventually, Alan realizes that some responses have enormous bodies, and would like stream them instead of buffering them fully in memory. He's *used* the `Stream` trait before. Using it was very natural, and followed a similar pattern to regular `async`/`await`:

```rust
while let Some(chunk) = body.next().await? {
    file.write_all(&chunk).await?;
}
```

However, _implementing_ `Stream` turns out to be rather different. While he quickly learned the simple way to turn a `File` into a `Stream` with `StreamReader`, the chaining part was much harder.

### Imperatively Wrong

Alan first hoped he could simply write "chain"-like stream imperatively, reusing his new knowledge of `async` and `await`, and assuming it'd be similar to JavaScript:

```rust
async* fn sign(file: ReaderStream) -> Result<Vec<u8>, Error> {
    let mut sig = Signature::new();

    while let Some(chunk) = file.next().await? {
        sig.push(chunk.len());
        yield Ok(chunk)
    }

    yield Ok(sig.digest().await)
}
```

Unfortunately, that doesn't work. The compiler first complains about the `async* fn` syntax:

```
error: expected item, found keyword `async`
  --> src/lib.rs:21:1
   |
21 | async* fn sign(file: ReaderStream) -> Result<Vec<u8>, Error> {
   | ^^^^^ expected item
```

Less hopeful, Alan tries just deleting the asterisk:

```
error[E0658]: yield syntax is experimental
  --> src/lib.rs:27:9
   |
27 |         yield Ok(chunk)
   |         ^^^^^^^^^^^^^^^
   |
   = note: see issue #43122 <https://github.com/rust-lang/rust/issues/43122> for more information
```

After reading about how yield is experimental, and giving up reading the 100+ comments in the linked issue, Alan figures he's just got to implement `Stream` manually.

### Implementing `Stream`

Implementing a `Stream` means writing async code in a way that doesn't _feel_ like all the other `async` code that the user has already written. The user is introduced to writing a `poll` function, which forces these three things into their face:

- `Pin`
- State machines
- `Wakers`

Unsure of what the final code will look like, he starts with:

```rust
struct SigningFile;

impl Stream for SigningFile {
    type Item = Result<Vec<u8>, Error>;
    
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context)
        -> Poll<Self::Item>
    {
 
    }
}
```

#### Pin :scream:

First, he notices `Pin`. Alan wonders, "Why does `self` have bounds? I've only ever seen `self`, `&self`, and `&mut self` before". Curious, he reads the `std::pin` page, and a bunch of jargon about pinning data in memory. He also reads that this is useful to guarantee that an object cannot move, and he wonders why he cares about that. The only example on the page explains how to write a ["self-referential struct"][self-ref], but notices it needs `unsafe` code, and that triggers an internal alarm in Alan: "I thought Rust was safe..."

Eventually (how?), Alan realizes that the types he's depending on are `Unpin`, and so he doesn't need to worry about the unsafe stuff. It's just a more-annoying pointer type.

[self-ref]: https://doc.rust-lang.org/std/pin/index.html#example-self-referential-struct

#### State Machine

With `Pin` hopefully ignored, Alan next notices that in the imperative style he wanted originally, he didn't need to explicitly keep track of state. The state was simply the imperative order of the function. But in a `poll` function, the state isn't saved by the compiler. Alan finds blog posts about the dark ages of Futures 0.1, when even `Future` was written with a "state machine".

He thinks about his stream's states, and settles on the following structure:

```rust
struct SigningFile {
    state: State,
    file: ReaderStream,
    sig: Signature,
}

enum State {
    File,
    Sign,
}
```

Now he tries to write the `poll_next` method, checking readiness of individual steps and proceeding to the next state, while grumbling away the weird `Pin` noise:

```rust
todo!("is this worth showing in the story, or just noise?")
```

It turns out it was more complicated than Alan thought (the author made this same mistake). The `digest` method of `Signature` is `async`, _and_ it consumes the signature, so the state machine needs to be adjusted. The signature needs to be able to be moved out, and it needs to be able to store a future from an `async fn`. Trying to figure out how to represent that in the type system was difficult (expand?), but Alan eventually learns to just store a `Pin<Box<dyn Future>>`, wondering if the `Pin` there is important.

```rust
struct SigningFile {
    state: State,
}

enum State {
    File(ReaderStream, Signature),
    Signing(Pin<Box<dyn Future<Item = Vec<u8>>>>),
    Done,
}
```

(show how gross this codes gets?)

Oh well, at least it _works_, right?

#### Wakers

So far, Alan hasn't paid too much attention to `Context` and `Poll`. It's been fine to simply pass them along untouched. There's a confusing bug in his state machine. Let's look more closely:

```rust
// zooming in!
match ready!(file.poll_next(cx)) {
    Some(val) => {
        me.sig.as_mut().unwrap().push(val.len()));
        return Poll::Ready(Some(val));
    },
    None => {
        me.state = State::Sign;
        // oops!
        return Poll::Pending;
    }
}
```

In one of the branches, the state is changed, and `Poll::Pending` is returned. Alan assumes that the task will be polled again with the new state. But, since the file was done (and has returned `Poll::Ready`), there was actually no waker registered to wake the task again. So his stream just hangs forever.

The compiler doesn't help at all, and he re-reads his code multiple times, but because of this easy-to-misunderstand logic error, Alan eventually has to ask for help in a chat room. After a half hour of explaining all sorts of details, a kind person points out he either needs to register a waker, or perhaps use a loop.

(Show the solution? in far too many cases (likely this one), the code gets turned into a weird loop, or the user must learn about wakers).

### Gives Up

A little later, Alan needs to add some response body transforming to some routes, to add some app-specific framing. Upon realizing he needs to implement another `Stream` in a generic fashion, he instead closes the editor and complains on Twitter.



## 🤔 Frequently Asked Questions


* **What are the morals of the story?**
    * Writing an async `Stream` is drastically different than writing an `async fn`.
    * `Pin` is explained in an abstract way, but do most users who run into `Pin` for the first time need all that? It looks so scary.
    * Missing a waker registration is a runtime error, and very hard to debug. If it's even possible, a compiler warning or hint would go a long way.
* **What are the sources for this story?**
    * Part of this story is based on the original motivation for `async`/`await` in Rust, since similar problems exist writing `impl Future`.
* **Why did you choose [Alan][] to tell this story?**
    * Choosing Alan was somewhat arbitrary, but this does get to reuse the expectation that Alan may already have around `await` coming from JavaScript.
* **How would this story have played out differently for the other characters?**
    * This likely would have been a similar story for any character.
    * It's possible [Grace][] would be more used to writing state machines, coming from C.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
