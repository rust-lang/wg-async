# Async overloading

## Impact

* By default, function definitions can be compiled into either sync or async mode
* Able to overload a function with two variants, one for sync and one for async

## Design notes

This is a highly speculative deliverable. However, it would be great if one were able to write code that is neither sync nor sync, but potentially *either*. Further, one should be able to provide *specialized* variants that perform the same task but in slightly different ways; this would be particularly useful for primitives like TCP streams.

### Monomorphize

The way to think of this is that every function has an implicit generic parameter indicating its *scheduler mode*. When one writes `fn foo()`, that is like creating a generic impl:

```rust
impl<SM> Fn<(), SM> for Foo 
where 
    SM: SchedulerMode,
{
    ...
}
```

When one writes `async fn` or `sync fn`, those are like providing specific impls:

```rust
impl Fn<(), AsyncSchedulerMode> for Foo {
    ...
}

impl Fn<(), SchedulerMode> for Foo {
    ...
}
```

Further, by default, when you call a function, you invoke it in the same scheduler mode as the caller.

### Implications for elsewhere

* If we had this feature, then having distinct modules like `use std::io` and `use std::async_io` would not be necessary.
* Further, we would want to design our traits and so forth to have a "common subset" of functions that differ only in the presence or absence of the keyword `async`.

### Related work

* [SE-0296: Allow overloads that differ only in async](https://github.com/apple/swift-evolution/pull/1392)
* [Async Overloading (Yoshua Wuyts, 2021)](https://blog.yoshuawuyts.com/async-overloading/)
