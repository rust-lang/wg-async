# Inline async fn

## Impact

* Able to create async functions whose storage is stored in the receiver, rather than being returned to the caller
* Resulting future is Send if receiver is Send
* Resulting trait is dyn safe without any limitations or compromises

## Design notes

Short version: make it possible to have async fn where the state is stored in the `Self` type ([detailed writeup](https://hackmd.io/bKfiVPRpTvyX8JK_Ng2EWA)). This is equivalent to writing a poll function. Like a poll function, it makes the trait dyn safe; it also has the advantage that `Self: Send` implies that the returned future is also `Send`.

## Frequently asked questions

### What aspects of the design are unresolved?

Primarily bikeshed. How should we designate that an async function is 'inline', and can we come up with a less overloaded name?

There is also the concern that the overall complexity of having varieties of "async functions" is too much.

### Do we really need "inline" async functions?

They are needed to manage `AsyncDrop` at _minimum_, but they make sense for any trait which is the "primary purpose" of the types that implement it. Basically, anywhere that we used a "poll" function today. After all, poll functions today push all the "intermediate state" into the `self` type in exactly the same fashion as an inline async function.

### Why not _just_ have "inline" async functions?

Async functions aren't a good match for traits that have a lot of methods, since that would put a lot of state into the `self` type. Also, inline async functions cannot express method-level generics (beyond lifetime parameters).

### Why do need both inline async functions and GATs?

Inline async functions are only suitable for non-generic async functions and for async fn with `&mut self`. If an async function is generic, for example:

```rust
trait Foo {
    async fn foo<T>(&mut self);
}
```

then we cannot store its state in `self` because we would need distinct copies for each value of `T`.
