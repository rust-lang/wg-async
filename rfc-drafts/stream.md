- Feature Name: `async_stream`
- Start Date: 2020-05-13
- RFC PR: [rust-lang/rfcs#0000](https://github.com/rust-lang/rfcs/pull/0000)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)

# Summary
[summary]: #summary

Introduce the `Stream` trait into the standard library, using the
design from `futures`. Redirect the `Stream` trait definition in the 
`futures-core` crate (which is "pub-used" by the `futures` crate) to the standard library.

# Motivation
[motivation]: #motivation

Streams are a core async abstraction. We want to enable portable libraries that 
produce/consume streams without being tied to a particular executor.

People can do this currently using the `Stream` trait defined in the 
[futures](https://crates.io/crates/futures) crate. However, the 
stability guarantee of that trait would be clearer if it were added 
to the standard library. For example, if [Tokio](https://tokio.rs/) 
wishes to declare a [5 year stability period](http://smallcultfollowing.com/babysteps/blog/2020/02/11/async-interview-6-eliza-weisman/#communicating-stability), 
having the stream trait in the standard library means there are no concerns 
about the trait changing during that time ([citation](http://smallcultfollowing.com/babysteps/blog/2019/12/23/async-interview-3-carl-lerche/#what-should-we-do-next-stabilize-stream)).

## Examples of crates that are consuming streams

### async-h1

* [async-h1](https://docs.rs/async-h1)'s server implementation takes `TcpStream` instances produced by a `TcpListener` in a loop.

### async-sse

* [async-sse](https://docs.rs/async-sse/) parses incoming buffers into a stream of messages.

## Why a shared trait?

We eventually want dedicated syntax for working with streams, which will require a shared trait. 
This includes a trait for producing streams and a trait for consuming streams.

## Why is the stream trait defined how it is?
* It is the "pollable iterator"
* [dyn compatibility](https://doc.rust-lang.org/std/keyword.dyn.html)

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

It is possible that it could live in another area as well, though this followes
the pattern of `core::future`.

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

As mentioned above, `core::stream` is analogous to `core::future`. But, do we want to find some other naming scheme that can scale up to other future additions, such as io traits or channels?

# Prior art
[prior-art]: #prior-art

Discuss prior art, both the good and the bad, in relation to this proposal.

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

## IntoStream / FromStream traits

### IntoStream

**Iterators**

Iterators have an `IntoIterator` that is used with `for` loops to convert items of other types to an iterator.

```rust
pub trait IntoIterator where
    <Self::IntoIter as Iterator>::Item == Self::Item, 
{
    type Item;

    type IntoIter: Iterator;

    fn into_iter(self) -> Self::IntoIter;
}
```

Examples taken from the Rust docs on [for loops and into_iter]](https://doc.rust-lang.org/std/iter/index.html#for-loops-and-intoiterator)

* `for x in iter` uses `impl IntoIterator for T`

```rust
let values = vec![1, 2, 3, 4, 5];

for x in values {
    println!("{}", x);
}
```

Desugars to:

```rust
let values = vec![1, 2, 3, 4, 5];
{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            let next;
            match iter.next() {
                Some(val) => next = val,
                None => break,
            };
            let x = next;
            let () = { println!("{}", x); };
        },
    };
    result
}
```
* `for x in &iter` uses `impl IntoIterator for &T`
* `for x in &mut iter` uses `impl IntoIterator for &mut T`

**Streams**

We may want a trait similar to this for `Stream`. The `IntoStream` trait would provide a way to convert something into a `Stream`.

This trait could look like this:

```rust
pub trait IntoStream where 
    <Self::IntoStream as Stream>::Item == Self::Item,
{
    type Item;

    type IntoStream: Stream;

    fn into_stream(self) -> Self::IntoStream;
}
```

### FromStream

Iterators have an `FromIterator` that is used to convert iterators into another type.

We may want a trait similar to this for `Stream`. The `FromStream` trait would provide way to convert a `Stream` into another type.

This trait could look like this:

[TO BE ADDED]

## Other Traits

Eventually, we may also want to add some (if not all) of the roster of traits we found useful for `Iterator`.

[async_std::stream](https://docs.rs/async-std/1.6.0/async_std/stream/index.html) has created several async counterparts to the traits in [std::iter](https://doc.rust-lang.org/std/iter/). These include:

* DoubleEndedStream: A stream able to yield elements from both ends.
* ExactSizeStream: A stream that knows its exact length.
* Extend: Extends a collection with the contents of a stream.
* FromStream: Conversion from a Stream.
* FusedStream: A stream that always continues to yield None when exhausted.
* IntoStream: Conversion into a Stream.
* Product: Trait to represent types that can be created by multiplying the elements of a stream.
* Stream: An asynchronous stream of values.
* Sum: Trait to represent types that can be created by summing up a stream.

As detailed in previous sections, the migrations to add these traits are out of scope for this RFC.

## Async iteration syntax

Currently, if someone wishes to iterate over a `Stream` as defined in the `futures` crate,
they are not able to use  `for` loops, they must use `while let` and `next/try_next` instead.

We may wish to extend the `for` loop so that it works over streams as well. 

```rust
#[async]
for elem in stream { ... }
```

Designing this extension is out of scope for this RFC. However, it could be prototyped using procedural macros today.

## "Lending" streams

There has been much discussion around lending streams (also referred to as attached streams).

### Definitions

[Source](https://smallcultfollowing.com/babysteps/blog/2019/12/10/async-interview-2-cramertj-part-2/#the-need-for-streaming-streams-and-iterators)


In an **lending** stream (also known as an "attached" stream), the `Item` that gets returned by `Stream` may be borrowed from `self`. It can only be used as long as the `self` reference remains live.

In a **non-lending** stream (also known as a "detached" stream), the `Item` that gets returned by `Stream` is "detached" from self. This means it can be stored and moved about independently from `self`.

This RFC does not cover the addition of lending streams (streams as implemented through this RFC are all non-lending streams).

We can add the `Stream` trait to the standard library now and delay
adding in this distinction between the two types of streams - lending and
non-lending. The advantage of this is it would allow us to copy the `Stream`
trait from `futures` largely 'as is'. 

The disadvantage of this is functions that consume streams would 
first be written to work with `Stream`, and then potentially have 
to be rewritten later to work with `LendingStream`s.

### Current Stream Trait

```rust
pub trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
```

This trait, like `Iterator`, always gives ownership of each item back to its caller. This offers flexibility - 
such as the ability to spawn off futures processing each item in parallel.

### Potential Lending Stream Trait

```rust
impl<S> LendingStream for S
where
    S: Stream,
{
    type Item<'_> = S::Item;
    
    fn poll_next<'s>(
        self: Pin<&'s mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item<'s>>> {
        Stream::poll_next(self, cx)
    }
}
```

This is a "conversion" trait such that anything which implements `Stream` can also implement 
`Lending Stream`.

This trait captures the case we re-use internal buffers. This would be less flexible for 
consumers, but potentially more efficient. Types could implement the `LendingStream` 
where they need to re-use an internal buffer and `Stream` if they do not. There is room for both.

We would also need to pursue the same design for iterators - whether through adding two traits
or one new trait with a "conversion" from the old trait.

This also brings up the question of whether we should allow conversion in the opposite way - if
every "Detached" stream can become an attached one, should _some_ detached streams be able to 
become attached ones? These use cases need more thought, which is part of the reason 
it is out of the scope of this particular RFC.

## Generator syntax
[generator syntax]: #generator-syntax

In the future, we may wish to introduce a new form of function - 
`gen fn` in iterators and `async gen` in async code that
can contain `yield` statements. Calling such a function would
yield a `impl Iterator` or `impl Stream`, for sync and async 
respectively. Given an "attached" or "borrowed" stream, the generator
yield could return references to local variables. Given a "detached"
or "owned" stream, the generator yield could return things
that you own or things that were borrowed from your caller.

```rust
gen async fn foo() -> X {
    yield value;
}
```

Designing generator functions is out of the scope of this RFC.
