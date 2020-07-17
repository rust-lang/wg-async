# RFC: Must not await lint

# Summary

Introduce a `#[must_not_await]` lint in the compiler that will warn the user when they are incorrectly holding a struct across an await boundary.

# Motivation

Enable users to fearlessly write concurrent async code without the need to understand the internals of runtimes and how their code will be affected. The goal is to provide a best effort warning that will let the user know of a possible side effect that is not visible by reading the code right away. Some examples of these side effects are holding a `MutexGuard` across an await bound in a single threaded runtime. In this case the resulting generated future will resolve to `!Send` but could still hold the lock when the future yields back to the executor. This opens up for the possibility of causing a deadlock since the future holding onto the lock did not relinquish it back before it yielded control. This can become even more problematic for futures that run on single-threaded runtimes (`!Send`) where holding a local after a yield will result in a deadlock.

The big reason for including a lint like this is because under the hood the compiler will automatically transform async fn into a state machine which can store locals. This process is invisible to users and will produce code that is different than what is in the actual rust file. Due to this it is important to inform users that their code may not do what they expect.

# Guide-level explanation

Provide a lint that can be attached to structs to let the compiler know that this struct can not be held across an await boundary.

```rust
    #[must_not_await]
    struct MyStruct {}
```

This struct if held across an await boundary would cause a warning:

```rust
    async fn foo() {
      let my_struct = MyStruct {};
      my_async_op.await;
      println!("{:?}", my_struct);
    }
```

The compiler might output something along the lines of:

TODO: Write a better error message.
```
warning: Holding `MyStruct` across the await bound on line 3 might cause side effects.
```

Example use cases for this lint:

- `MutexGuard` holding this across a yield boundary in a single threaded executor could cause deadlocks. In a multi-threaded runtime the resulting future would become `!Send` which will stop the user from spawning this future and causing issues. But in a single threaded runtime which accepts `!Send` futures deadlocks could happen.

- The same applies to other such synchronization primitives such as locks from `parking-lot`.

- `tracing::Span` has the ability to enter the span via the `tracing::span::Entered` guard. While entering a span is totally normal, during an async fn the span only needs to be entered once before the `.await` call, which might potentially yield the execution.

- Any RAII guard might possibly create unintended behavior if held across an await boundary.

This lint will enable the compiler to warn the user that the generated MIR could produce unforeseen side effects. Some examples of this are:

- [`std::sync::MutexGuard`](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html)
- [`tracing::span::Entered`](https://docs.rs/tracing/0.1.15/tracing/span/struct.Entered.html)

This will be a best effort lint to signal to the user about unintended side-effects of using certain types across an await boundary.

# Reference-level explanation

Going throuogh the prior are we see two systems currently which provide simailar/semantically similar behavior:

## Clippy `#[await_holding_lock]` lint
This lint goes through all types in `generator_interior_types` looking for `MutexGuard`, `RwLockReadGuard` and `RwLockWriteGuard`. While this is a first great step, we think that this can be further extended to handle not only the hardcoded lock guards, but any type which is should not be held across an await point. By marking a type as `#[must_not_await]` we can warn when any arbitrary type is being held across an await boundary. An additional benefit to this approach is that this behaviour can be extended to any type which holds a `#[must_not_await]` type inside of it.

## `#[must_use]` attribute
The `#[must_use]` attribute ensures that if a type or the result of a function is not used, a warning is displayed. This ensures that the user is notified about the importance of said value. Currently the attribute does not automatically get applied to any type which contains a type declared as `#[must_use]`, but the implementation for both `#[must_not_await]` and `#[must_use]` should be similar in their behavior.

### Auto trait vs attribute
`#[must_use]` is implemented as an attribute, and from prior art and [other literature][linear-types], we can gather that the decision was made due to the complexity of implementing true linear types in Rust. [`std::panic::UnwindSafe`][UnwindSafe] on the other hand is implemented as a marker trait with structural composition.

[linear-types]: https://gankra.github.io/blah/linear-rust/
[UnwindSafe]: https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html


**TODO**

 - [ ] Draw ideas from `#[must_use]` implementation
 - [ ] Go through generator implementation to understand how values are captured in state machine.

 - Reference link on how mir transfroms async fn https://tmandry.gitlab.io/blog/posts/optimizing-await-2/

# Drawbacks
- There is a possibility it can produce a false positive warning and it could get noisy. We likely want to allow overriding via some sort of module level `allow` attribute.

# Rationale and alternatives


# Prior art

* [Clippy lint for holding locks across await points](https://github.com/rust-lang/rust-clippy/pull/5439)
* [Must use for functions](https://github.com/iopq/rfcs/blob/f4b68532206f0a3e0664877841b407ab1302c79a/text/1940-must-use-functions.md)

# Future possibilities

- Propagate the lint in nested structs/enums. Similar to the use case for the `must_use` attribute. These likely should be solved together.


