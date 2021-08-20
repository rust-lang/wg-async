# Capability

## Impact

* The ability to create async tasks that can be safely given access to borrowed data, similar to crossbeam or rayon scopes
* There are potentially multiple routes with which this can be accomplished

## Design notes

Today's `Future` trait lacks one fundamental capability compared to synchronous code: there is no (known?) way to "block" your caller and be sure that the caller will not continue executing until you agree. In synchronous code, you can use a closure and a destructor to achieve this, which is the technique used for things like `rayon::scope` and crossbeam's scoped threads. In async code, because the `Future` trait has a safe poll function, it is always possible to poll it part way and then `mem::forget` (or otherwise leak) the value; this means that one cannot have parallel threads executing and using those references.

Async functions are commonly written with borrowed references as arguments:

```rust
async fn do_something(db: &Db) { ... }
```

but important utilities like `spawn` and `spawn_blocking` require `'static` tasks. Without "unfogettable" traits, the only way to circumvent this is with mechanisms like `FuturesUnordered`, which is then subject to footguns as described in [Barbara battles buffered streams](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/barbara_battles_buffered_streams.html).

There are two main approaches under consideration to address this issue:

* [Introducing a new trait for futures, Async](./capability/variant_async_trait.md)
* [Introducing a new "default" trait, Leak](./capability/variant_leak.md) that can be used to prevent values from leaking
    * If we say that scopes cannot be leaked, and the scope defines `AsyncDrop`, then we can (presumably) be sure that its destructor will run.
    