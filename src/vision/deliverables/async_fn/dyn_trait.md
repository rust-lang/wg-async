# Dyn trait

## Impact

* Soundness holes relating to `dyn Trait` are closed.
* The semver implication of whether a trait is "dyn or not" are clear.
* More kinds of traits are dyn-safe.
* Easily able to have a "dynamically dispatched core" with helper methods.
* Users are able to the "adaptation" from a statically known type (`T: Trait`) into a `dyn Trait`.

## Design notes

### Soundness holes

FIXME-- list various issues here :)

### Semver implications

Today, the compiler automatically determines whether a trait is "dyn-safe". This means that otherwise legal additions to the trait (such as new )

### More kinds of traits are dyn-safe

Currently dyn-safe traits exclude a lot of functionality, such as generic methods. We may be able to lift some of those restrictions.

### Easily able to have a "dynamically dispatched core" with helper methods

There is a common pattern with e.g. `Iterator` where there is a dynamically dispatched "core method" (`fn next()`) and then a variety of combinators and helper methods that use `where Self: Sized` to side-step dyn-safety checks. These methods often involve generics. We should make this pattern easier and more obvious, and (ideally) make it work better -- e.g., by having those methods *also* available on `dyn Trait` receivers (which seems fundamentally possible).

### Adaptation

In the case of async Rust, given a trait `Foo` that contains `async fn` methods, we wish to be able to have the user write `dyn Foo` without having to specify the values of the associated types that contain the future types for those methods. Consider the fully desugard example:

```rust
trait Foo {
    type Method<..>: Future;
    fn method() -> Self::Method<..>
}
```

Roughly speaking we wish to be able to supply an impl like

```rust
impl Foo for dyn Foo {
    type Method<..> = Box<dyn Future<..>>;
    fn method() -> Self::Method {
        // call, via vtable, a shim that will create the `Box`
        // (or whichever smart pointer is desired)
    }
}
```

Ideally, this would be a general capability that users can use to control the adaptation of "known types" to `dyn` types for other traits.
