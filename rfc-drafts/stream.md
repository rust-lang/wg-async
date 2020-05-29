- Feature Name: `async_stream`
- Start Date: 2020-05-13
- RFC PR: [rust-lang/rfcs#0000](https://github.com/rust-lang/rfcs/pull/0000)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)

# Summary
[summary]: #summary

Introduce the `Stream` trait into the standard library, using the
design from `futures`. Redirect the futures-stream definition to the
standard library.

# Motivation
[motivation]: #motivation

Streams are a core async abstraction. We want to enable portable libraries that produce/consume streams without being tied to a particular executor.

People can do this currently using the [futures](https://crates.io/crates/futures) crate, but stability guarantees are clearer when traits are added to the standard library than when they exist in a separate crate. For example, if [Tokio](https://tokio.rs/) wishes to declare a [5 year stability period](http://smallcultfollowing.com/babysteps/blog/2020/02/11/async-interview-6-eliza-weisman/#communicating-stability), having the stream trait in std means there are no concerns about trait changing during that time ([citation](http://smallcultfollowing.com/babysteps/blog/2019/12/23/async-interview-3-carl-lerche/#what-should-we-do-next-stabilize-stream)).

## Examples of crates that are consuming streams

### async-h1

* [async-h1](https://docs.rs/async-h1)'s server implementation takes `TcpStream` instances produced by a `TcpListener` in a loop.

### async-sse

* [async-sse](https://docs.rs/async-sse/) parses incoming buffers into a stream of messages.

## Why a shared trait?

We eventually want dedicated syntax for working with streams, which will require a shared trait. This includes a trait for producing streams and a trait for consuming streams.

## Why is the stream trait defined how it is?
* It is the "pollable iterator"
* dyn compatibility

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

A "stream" is the async version of an iterator. The `Stream` trait
matches the definition of an [iterator], except that the `next` method
is defined to "poll" for the next item. In other words, where the
`next` method on an iterator simply computes (and returns) the next
item in the sequence, the `poll_next` method on stream asks if the
next item is ready. If so, it will be returned, but otherwise
`poll_next` will return [`Poll::pending`]. Just as with a [`Future`],
returning [`Poll::pending`] implies that the stream has arranged for
the current task to be re-awoken when the data is ready.

[iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
[`Poll::pending`]: https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Pending

```rust
// Defined in std::stream module
pub trait Stream {
    type Item;
    
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
    
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
```

The arguments to `poll_next` match that of the [`Future::poll`] method:

* The self must be a pinned reference, ensuring both unique access to
  the stream and that the stream value itself will not move. Pinning
  allows the stream to save pointers into itself when it suspends,
  which will be required to support generator syntax at some point.
* The [context] `cx` defines details of the current task. In particular,
  it gives access to the [`Waker`] for the task, which will allow the
  task to be re-awoken once data is ready.

[`Future::poll`]: https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll
[pinned]: https://doc.rust-lang.org/std/pin/struct.Pin.html
[context]: https://doc.rust-lang.org/std/task/struct.Context.html
[Waker]: https://doc.rust-lang.org/std/task/struct.Waker.html

## Initial impls

There are a number of simple "bridge" impls that are also provided:

```rust
impl<S> Stream for Box<S>
where
    S: Stream + Unpin + ?Sized,
{
    type Item = <S as Stream>::Item
}

impl<S> Stream for &mut S
where
    S: Stream + Unpin + ?Sized,
{
    type Item = <S as Stream>::Item;
}

impl<S, T> Stream for Pin<P>
where
    P: DerefMut<Target=T> + Unpin,
    T::Target: Stream,
{
    type Item = <T as Stream>::Item;
}

impl<S> Stream for AssertUnwindSafe<S>
where
    S: Stream, 
{
    type Item = <S as Stream>::Item;
}
```

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

This section goes into details about various aspects of the design and
why they ended up the way they did.

## Where does `Stream` live in the std lib?

`Stream` will live in the `core::stream` module and be re-exported as `std::stream`.

## Why use a `poll` method?

An alternative design for the stream trait would be to have a trait
that defines an async `next` method:

```rust
trait Stream {
    type Item;
    
    async fn next(&mut self) -> Option<Self::Item>;
}
```

Unfortunately, async methods in traits are not currently supported,
and there [are a number of challenges to be
resolved](https://rust-lang.github.io/wg-async-foundations/design_notes/async_fn_in_traits.html)
before they can be added. 

Moreover, it is not clear yet how to make traits that contain async
functions be `dyn` safe, and it is imporant to be able to pass around `dyn
Stream` values without the need to monomorphize the functions that work
with them.

Unfortunately, the use of poll does mean that it is harder to write
stream implementations. The long-term fix for this, discussed in the [Future possibilities][future-possibilities] section, is dedicated [generator syntax].

# Drawbacks
[drawbacks]: #drawbacks

Why should we *not* do this?

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

## Where should stream live?

* core::stream is analogous to core::future
* but do we want to find some other naming scheme that can scale up to other future additions, such as io traits or channels?

# Prior art
[prior-art]: #prior-art

Discuss prior art, both the good and the bad, in relation to this proposal.

The best example of prior art in Rust is the [futures](https://crates.io/crates/futures) crate.

* Ruby - https://github.com/socketry/async-io
* Javascript https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator
* more Javascript https://javascript.info/async-iterators-generators
* Dart https://dart.dev/tutorials/language/streams



- For language, library, cargo, tools, and compiler proposals: Does this feature exist in other programming languages and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other languages, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other languages.

Note that while precedent set by other languages is some motivation, it does not on its own motivate an RFC.
Please also take into consideration that rust sometimes intentionally diverges from common language features.

# Unresolved questions
[unresolved-questions]: #unresolved-questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?

# Future possibilities
[future-possibilities]: #future-possibilities

## Convenience methods

The `Iterator` trait defines a number of useful combinators, like
`map`.  The `Stream` trait being proposed here does not include any
such conveniences.  Instead, they are available via extension traits,
such as the [`StreamExt`] trait offered by the [`futures`] crate.

[`StreamExt`]: https://docs.rs/futures/0.3.5/futures/stream/trait.StreamExt.html
[`futures`]: https://crates.io/crates/futures

The reason that we have chosen to exclude combinators is that a number
of them would require access to async closures. As of this writing,
async closures are unstable and there are a number of [outstanding
design issues] to be resolved before they are added. Therefore, we've
decided to enable progress on the stream trait by stabilizing a core,
and to come back to the problem of extending it with combinators.

[outstanding design issues]: https://rust-lang.github.io/wg-async-foundations/design_notes/async_closures.html

Another reason to defer adding combinators is because of the possibility
that some combinators may work best 

This path does carry some risk. Adding combinator methods can cause
existing code to stop compiling due to the ambiguities in method
resolution. We have had problems in the past with attempting to migrate
iterator helper methods from `itertools` for this same reason.

While such breakage is technically permitted by our semver guidelines,
it would obviously be best to avoid it, or at least to go to great
lengths to mitigate its effects. One option would be to extend the
language to allow method resolution to "favor" the extension trait in
existing code, perhaps as part of an edition migration.

Designing such a migration feature is out of scope for this RFC.

## IntoStream / FromStream traits, mirroring iterators

* currently blocked on async fn in traits
* The exact bounds are unclear.
* the same as combinators
* These would be needed to provide similar iteration semantics as Iterator:
    * `for x in iter` uses `impl IntoIterator for T`
    * `for x in &iter` uses `impl IntoIterator for &T`
    * `for x in &mut iter` uses `impl IntoIterator for &mut T`

## Async iteration syntax

We may wish to introduce some dedicated syntax, analogous to `for` 

## Generator syntax
[generator syntax]: #generator-syntax

```rust
gen async fn foo() -> X {
    yield value;
}
```

## "Attached" streams

Just as with iterators, there is a 


