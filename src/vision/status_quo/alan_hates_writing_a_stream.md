# 😱 Status quo stories: Alan hates writing a `Stream`

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[Alan][] is used to writing web server applications using async sockets, but wants to try Rust to get that signature vroom vroom.

After a couple weeks learning Rust basics, Alan quickly understands `async` and `await`, and therefore has several routes built for his application that await a few things and then construct an HTTP response and send a buffered body. To build the buffered response bodies, Alan was reading a file, and then appending a signature, and putting that all into a single buffer of bytes.

Eventually, Alan realizes that some responses have enormous bodies, and would like to stream them instead of buffering them fully in memory. He's *used* the `Stream` trait before. Using it was very natural, and followed a similar pattern to regular `async`/`await`:

```rust,ignore
while let Some(chunk) = body.next().await? {
    file.write_all(&chunk).await?;
}
```ignore

However, _implementing_ `Stream` turns out to be rather different. With a quick search, he learned the simple way to turn a `File` into a `Stream` with `ReaderStream`, but the signing part was much harder.

### Imperatively Wrong

Alan first hoped he could simply write signing stream imperatively, reusing his new knowledge of `async` and `await`, and assuming it'd be similar to JavaScript:

```rust,ignore
async* fn sign(file: ReaderStream) -> Result<Vec<u8>, Error> {
    let mut sig = Signature::new();

    while let Some(chunk) = file.next().await? {
        sig.push(&chunk);
        yield Ok(chunk)
    }

    yield Ok(sig.digest().await)
}
```ignore

Unfortunately, that doesn't work. The compiler first complains about the `async* fn` syntax:

```notrust
error: expected item, found keyword `async`
  --> src/lib.rs:21:1
   |
21 | async* fn sign(file: ReaderStream) -> Result<Vec<u8>, Error> {
   | ^^^^^ expected item
```ignore

Less hopeful, Alan tries just deleting the asterisk:

```notrust
error[E0658]: yield syntax is experimental
  --> src/lib.rs:27:9
   |
27 |         yield Ok(chunk)
   |         ^^^^^^^^^^^^^^^
   |
   = note: see issue #43122 <https://github.com/rust-lang/rust/issues/43122> for more information
```ignore

After reading about how yield is experimental, and giving up reading the 100+ comments in the [linked issue](https://github.com/rust-lang/rust/issues/43122), Alan figures he's just got to implement `Stream` manually.

### Implementing `Stream`

Implementing a `Stream` means writing async code in a way that doesn't _feel_ like the `async fn` that Alan has written so far. He needs to write a `poll` function and it has a lot of unfamiliar concepts:

- `Pin`
- State machines
- `Wakers`

Unsure of what the final code will look like, he starts with:

```rust,ignore
struct SigningFile;

impl Stream for SigningFile {
    type Item = Result<Vec<u8>, Error>;
    
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context)
        -> Poll<Self::Item>
    {
 
    }
}
```ignore

### Pin :scream:

First, he notices `Pin`. Alan wonders, "Why does `self` have bounds? I've only ever seen `self`, `&self`, and `&mut self` before". Curious, he reads the [`std::pin`](https://doc.rust-lang.org/std/pin/struct.Pin.html) page, and a bunch of jargon about pinning data in memory. He also reads that this is useful to guarantee that an object cannot move, and he wonders why he cares about that. The only example on the page explains how to write a ["self-referential struct"][self-ref], but notices it needs `unsafe` code, and that triggers an internal alarm in Alan: "I thought Rust was safe..."

After asking [Barbara], Alan realizes that the types he's depending on are `Unpin`, and so he doesn't need to worry about the unsafe stuff. It's just a more-annoying pointer type.

[self-ref]: https://doc.rust-lang.org/std/pin/index.html#example-self-referential-struct

### State Machine

With `Pin` hopefully ignored, Alan next notices that in the imperative style he wanted originally, he didn't need to explicitly keep track of state. The state was simply the imperative order of the function. But in a `poll` function, the state isn't saved by the compiler. Alan finds blog posts about the dark ages of Futures 0.1, when it was more common for manual `Future`s to be written with a "state machine".

He thinks about his stream's states, and settles on the following structure:

```rust,ignore
struct SigningFile {
    state: State,
    file: ReaderStream,
    sig: Signature,
}

enum State {
    File,
    Sign,
}
```ignore



It turns out it was more complicated than Alan thought (the author made this same mistake). The `digest` method of `Signature` is `async`, _and_ it consumes the signature, so the state machine needs to be adjusted. The signature needs to be able to be moved out, and it needs to be able to store a future from an `async fn`. Trying to figure out how to represent that in the type system was difficult. He considered adding a generic `T: Future` to the `State` enum, but then wasn't sure what to set that generic to. Then, he tries just writing `Signing(impl Future)` as a state variant, but that triggers a compiler error that `impl Trait` isn't allowed outside of function return types. Patient [Barbara] helped again, so that Alan learns to just store a `Pin<Box<dyn Future>>`, wondering if the `Pin` there is important.

```rust,ignore
struct SigningFile {
    state: State,
}

enum State {
    File(ReaderStream, Signature),
    Signing(Pin<Box<dyn Future<Output = Vec<u8>>>>),
    Done,
}
```ignore

Now he tries to write the `poll_next` method, checking readiness of individual steps (thankfully, Alan remembers `ready!` from the futures 0.1 blog posts he read) and proceeding to the next state, while grumbling away the weird `Pin` noise:

```rust,ignore
match self.state {
    State::File(ref mut file, ref mut sig) => {
        match ready!(Pin::new(file).poll_next(cx)) {
            Some(result) => {
                let chunk = result?;
                sig.push(&chunk);
                Poll::Ready(Some(Ok(chunk)))
            },
            None => {
                let sig = match std::mem::replace(&mut self.state, State::Done) {
                    State::File(_, sig) => sig,
                    _ => unreachable!(),
                };
                self.state = State::Signing(Box::pin(sig.digest()));
                Poll::Pending
            }
        }
    },
    State::Signing(ref mut sig) => {
        let last_chunk = ready!(sig.as_mut().poll(cx));
        self.state = State::Done;
        Poll::Ready(Some(Ok(last_chunk)))
    }
    State::Done => Poll::Ready(None),
}
```ignore

Oh well, at least it _works_, right?

### Wakers

So far, Alan hasn't paid too much attention to `Context` and `Poll`. It's been fine to simply pass them along untouched. There's a confusing bug in his state machine. Let's look more closely:

```rust,ignore
// zooming in!
match ready!(Pin::new(file).poll_next(cx)) {
    Some(result) => {
        let chunk = result?;
        sig.push(&chunk);
        return Poll::Ready(Some(Ok(val));
    },
    None => {
        self.set_state_to_signing();
        // oops!
        return Poll::Pending;
    }
}
```ignore

In one of the branches, the state is changed, and `Poll::Pending` is returned. Alan assumes that the task will be polled again with the new state. But, since the file was done (and has returned `Poll::Ready`), there was actually no waker registered to wake the task again. So his stream just hangs forever.

The compiler doesn't help at all, and he re-reads his code multiple times, but because of this easy-to-misunderstand logic error, Alan eventually has to ask for help in a chat room. After a half hour of explaining all sorts of details, a kind person points out he either needs to register a waker, or perhaps use a loop.

All too often, since we don't want to duplicate code in multiple branches, the solution for Alan is to add an odd `loop` around the whole thing, so that the next match branch uses the `Context`:

```rust,ignore
loop {
    match self.state {
        State::File(ref mut file, ref mut sig) => {
            match ready!(Pin::new(file).poll_next(cx)) {
                Some(result) => {
                    let chunk = result?;
                    sig.push(&chunk);
                    return Poll::Ready(Some(Ok(chunk)))
                },
                None => {
                    let sig = match std::mem::replace(&mut self.state, State::Done) {
                        State::File(_, sig) => sig,
                        _ => unreachable!(),
                    };
                    self.state = State::Signing(Box::pin(sig.digest()));
                    // loop again, to catch the `State::Signing` branch
                }
            }
        },
        State::Signing(ref mut sig) => {
            let last_chunk = ready!(sig.as_mut().poll(cx));
            self.state = State::Done;
            return Poll::Ready(Some(Ok(last_chunk)))
        }
        State::Done => return Poll::Ready(None),
    }
}
```ignore

### Gives Up

A little later, Alan needs to add some response body transforming to some routes, to add some app-specific framing. Upon realizing he needs to implement another `Stream` in a generic fashion, he instead closes the editor and complains on Twitter.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* Writing an async `Stream` is drastically different than writing an `async fn`.
* The documentation for `Pin` doesn't provide much practical guidance in how to use it, instead focusing on more abstract considerations.
* Missing a waker registration is a runtime error, and very hard to debug. If it's even possible, a compiler warning or hint would go a long way.

### **What are the sources for this story?**
Part of this story is based on the original motivation for `async`/`await` in Rust, since similar problems exist writing `impl Future`.

### **Why did you choose [Alan][] to tell this story?**
Choosing Alan was somewhat arbitrary, but this does get to reuse the experience that Alan may already have around `await` coming from JavaScript.

### **How would this story have played out differently for the other characters?**
* This likely would have been a similar story for any character.
* It's possible [Grace][] would be more used to writing state machines, coming from C.

[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
