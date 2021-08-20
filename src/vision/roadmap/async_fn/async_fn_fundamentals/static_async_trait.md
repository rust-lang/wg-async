# Static async fn in traits

## Impact

* Able to write `async fn` in traits and impls and use them in statically dispatched contexts
* Able to easily declare that `T: Trait + Send` where "every async fn in `Trait` returns a `Send` future"

## Design notes

Support async fn syntax in traits.

The core idea is that it desugars into [impl trait in traits](./impl_trait_in_traits.md):

```rust
trait SomeTrait {
    async fn foo(&mut self);
}

// becomes:

trait SomeTrait {
    fn foo<(&mut self) -> impl Future<Output = ()> + '_;
}
```

Naturally it should also work in an impl:

```rust
impl SomeTrait for someType {
    async fn foo(&mut self);
}
```

For async functions in traits to be useful, it is important that traits containing `async fn` be dyn-safe, which introduces a number of challenges that we have to overcome.

## Frequently asked questions

### Can users easily bound those GATs with `Send`, maybe even in the trait definition?

- People are likely to want to say "I want every future produced by this trait to be Send", and right now that is quite tedious.
- We need a way to do this.
- This applies equally to other "`-> impl Trait` in trait" scenarios.

### What about "dyn" traits?

- See the sections on "inline" and "dyn" async fn in traits below!