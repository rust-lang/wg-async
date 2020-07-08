# RFC: yield safe lint

# Summary

Introduce a `#[yield_unsafe]` lint in the compiler that will warn the user when they unsafely hold an unsafe yield struct across a yield boundary. 

# Motivation

Enable users to fearlessly write concurrent async code without the need to understand the internals of runtimes and how their code will be affected. The goal is to provide a best effort warning that will let the user know of a possible side effect that is not visible by reading the code right away. Some examples of these side effects are holding a `MutexGuard` across a yield bound in a single threaded runtime. In this case the resulting generated future will resolve to `!Send` but could still hold the lock when the future yield back to the executor. This opens up for the possibility of causing a deadlock since the future holding onto the lock did not relinquish it back before it yielded control.

# Guide-level explanation

Provide a lint that can be attached to structs to let the compiler know that this struct is unsafe to be held across a yield boundary.

```rust
    #[yield_unsafe]
    struct MyUnsafeYieldStruct {}
```

This struct if held across a yield boundary would cause a warning:

```rust
    async fn foo() {
      let my_struct = MyUnsafeYieldStruct {};
      my_async_op.await;
      println!("{:?}", my_struct);
    }
```

The compiler might output something along the lines of:


    warning: Holding `MyUnsafeYieldStruct` across the await bound on line 3 might cause side effects.

Examples use cases for this lint:

    - `MutexGuard` holding this across a yield boundary in a single threaded executor could cause deadlocks. In a multi-threaded runtime the resulting future would become `!Send` which will stop the user from spawning this future and causing issues. But in a single threaded runtime which accepts `!Send` futures deadlocks could happen.
        - The same applies to other such synchronization primitives such as locks from ParkingLot.
    - `tracing::Span` has the ability to enter the span via the `tracing::span::Entered` guard. While entering a span is totally normal, during an async fn the span only needs to be entered during 

This lint will enable the compiler to warn the user that the generated MIR could produce unforeseen side effects.
An unsafe yield struct is some struct that may cause side effects when it is stored internally to the generator. Some examples of this are 

This will be a best effort lint to signal to the user about unsafety of using certain types across yield points.

(somewhere go into that async fn/generators generate new “code” that is different than what the users sees, this causes the problem)

Signal incorrect usage of `!Send` types across yield points.

Examples:

- `[MutexGuard](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html)`
- https://docs.rs/tracing/0.1.15/tracing/span/struct.Entered.html

# Reference-level explanation

**TODO**

- [ ] Draw ideas from `#[must_use]` implementation
 - [ ] Go through generator implementation to understand how values are captured in state machine.

# Drawbacks
- There is a possibility it can produce a false positive warning and it could get noisy. We likely want to allow overriding via some sort of module level `allow` attribute.
    - This is probab


# Rationale and alternatives


# Prior art


# Unresolved questions

- What should we call a struct that should not be held across yield boundaries? `unsafe` is quite an overloaded term but is somewhat correct in this sense. That said, rust does not consider deadlocks as unsafe so technically the term is incorrect from rust’s definition.
- Better ways to disable the lint rather than using a `#[allow(yield_unsafe)]` at the top of the module? This would disable the lint for the entire module possibly masking other issues.
- What happens in this situation
    struct MyType<T> {
      item: usize,
      lock: MutexGuard<T> // known to be unsafe to use across yield points
    }
    - Is it the user’s responsibility to add `#[allow(yield_unsafe)]` to the struct?
    - Is it possible to “auto implement” the rule to the struct containing any fields marked `[allow(yield_unsafe)]`?
    From a usability perspective, the second option is much more desirable, as the user automatically gets this warning for “free”.

# Future possibilities

 

