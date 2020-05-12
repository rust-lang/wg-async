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

XXX describe async streams

discuss also some of the design goals

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

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

This section goes into details about various aspects of the design and
why they ended up the way they did.

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
stream implementations.

## What about combinators?

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

This path does carry some risk. Adding combinator methods can cause
existing code to stop compiling due to the ambiguities in method
resolution. We have had problems in the past with attempting to migate
iterator helper methods from `itertools` for this same reason.

While such breakage is technically permitted by our semver guidelines,
it would obviously be best to avoid it, or at least to go to great
lengths to mitigate its effects. One option would be to extend the
language to allow method resolution to "favor" the extension trait in
existing code, perhaps as part of an edition migration.

## "Attached" streams

## Compatibility with future generator syntax

# Drawbacks
[drawbacks]: #drawbacks

Why should we *not* do this?

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?

# Prior art
[prior-art]: #prior-art

Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

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

Think about what the natural extension and evolution of your proposal would
be and how it would affect the language and project as a whole in a holistic
way. Try to use this section as a tool to more fully consider all possible
interactions with the project and language in your proposal.
Also consider how the this all fits into the roadmap for the project
and of the relevant sub-team.

This is also a good place to "dump ideas", if they are out of scope for the
RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities,
you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future RFC; such notes should be
in the section on motivation or rationale in this or subsequent RFCs.
The section merely provides additional information.
