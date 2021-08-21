# Variant: Async trait

As proposed in https://github.com/Matthias247/rfcs/pull/1, one way to solve this is to introduce a new future trait with an unsafe poll method:

```rust
trait Async {
    type Output;

    /// # Unsafe conditions
    ///
    /// * Once polled, cannot be moved
    /// * Once polled, destructor must execute before memory is deallocated
    /// * Once polled, must be polled to completion
    ///
    /// FIXME: Have to specify how to manage panic.
    unsafe fn poll(
        &mut self,
        context: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output>;
}
```

This would then require "bridging impls" to convert the (now likely deprecated, or at least repurposed) Future trait:

```rust
impl<F: Future> Async for F { .. } // impl A
```

which in turn creates an interesting question, since if we wish to have a single combinator that is usable with either trait, specialization would be required:

```rust
impl<F: Future> Future for Combinator<F> { .. } // impl B
impl<F: Async> Async for Combinator<F> { .. }  // impl C

// Coherence error: Given some type `F1: Future`, 
// two ways to show that `Combinator<F1>: Async`.
```

## Bridging

Introduce "bridge impls" like the following:

```rust
impl<F> Async for F where F: Future {

}
```

Newer runtimes will be built atop the `Async` trait, but older code will still work with them, since everything that implements `Future` implements `Async`.

#### Combinators

One tricky case has to do with bridging combinators. If you have a combinator like `Join`:

```rust
struct Join<A, B> { ... }

impl<A, B> Future for Join<A, B>
where
    A: Future,
    B: Future,
{ }
```

This combinator cannot then be used with `Async` values. You cannot (today) add a second impl like the following for coherence reasons:

```rust
impl<A, B> Async for Join<A, B>
where
    A: Async,
    B: Async,
{ }
```

The problem is that this second impl creates multiple routes to implement `Async` for `Join<A, B>` where `A` and `B` are futures. These routes are of course equivalent, but the compiler doesn't know that.

### Solution A: Don't solve it

We might simply introduce new combinators for the `Async` trait. Particularly given the move to [scoped threads](./scoped.md) it is likely that the set of combinators would want to change anyhow.

### Solution B: Specialization

Specialization can be used to resolve this, and it would be a great feature for Rust overall. However, specialization has a number of challenges to overcome. Some related articles:

- [Maximally minimal specialization](https://smallcultfollowing.com/babysteps/blog/2018/02/09/maximally-minimal-specialization-always-applicable-impls/)
- [Supporting blanket impls in specialization](https://smallcultfollowing.com/babysteps/blog/2016/10/24/supporting-blanket-impls-in-specialization/)
