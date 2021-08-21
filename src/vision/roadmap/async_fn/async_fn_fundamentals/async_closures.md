# Async closures

## Impact

* Able to create async closures that work like ordinary closures but which can await values.
* Analogous traits to `Fn`, `FnMut`, `FnOnce`, etc
* Reconcile async blocks and async closures

## Design notes

Async functions need their own traits, analogous to `Fn` and friends:

```rust
#[repr(async_inline)]
trait AsyncFnOnce<A> {
    type Output;

    // Uh-oh! You can't encode these as `async fn` using inline async functions!
    async fn call(mut self, args: A) -> Self::Output;
}

#[repr(async_inline)]
trait AsyncFnMut: AsyncFnOnce {
    type Output;

    async fn call_mut(&mut self, args: A) -> Self::Output;
}

#[repr(async_inline)]
trait AsyncFn: AsyncFnMut {
    // Uh-oh! You can't encode these as `async fn` using inline async functions!
    async fn call(&self, args: A) -> Self::Output;
}
```

Some notes:

- `AsyncFnOnce` is really the same as [`Async`](../compose_control_scheduling/async_trait.md) -- both represent, effectively, a future that can be driven exactly once.
- The concept of `AsyncFn` is more reasonable, but it requires storing the state externally to make sense: how else can there be multiple parallel executions.
- Something is a bit off here.
