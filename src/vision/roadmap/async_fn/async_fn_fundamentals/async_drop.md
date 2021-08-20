# Async drop

## Impact

* Able to create types (database connections etc) that perform async operations on cleanup
* Able to detect when such types are dropped synchronously
* Able to identify the await points that result from async cleanup if needed

## Requires

* [inline async fn support](./inline_async_fn.md)

## Design notes

We can create a `AsyncDrop` variant that contains an `async fn`:

```rust
impl AsyncDrop for MyType {
    async fn drop(&mut self) {
        ...
    }
}
```

Like `Drop`, the `AsyncDrop` trait must be implemented for all values of its self-type.

### Async drop glue

Within async functions, when we drop a value, we will invoke "async drop glue" instead of "drop glue". "Async drop glue" works in the same basic way as "drop glue", except that it invokes `AsyncDrop` where appropriate (and may suspend):

* The async drop glue for a type T first executes the `AsyncDrop` method
    * If `T` has no `AsyncDrop` impl, then the glue executes the synchronous `Drop` impl
        * If `T` has no `Drop` impl, then this is a no-op
* The async drop glue then recursively "async drops" all fields of T

### Requires inline fn 

Making this work requires [inline async fn]. This is because Rust presently assumes *all* types are droppable. Consider a function `foo`:

```rust
async fn foo<T>(x: T) {}
```

Here, we will drop `x` when `foo` returns, but we do not know whether `T` implements `AsyncDrop` or not, and we won't know until monomorphization. However, to know whether the resulting future for `foo(x)` is `Send`, we have to know whether the code that drops `x` will be send. Using an inline function, we know that `T: Send` implies that the async drop future for `T` is `Send`.

Another argument in favor of [inline async fn] is that dropping ought not to create a lot more memory.

[inline async fn]: ./inline_async_fn.md

### Explicit async drop

We should have a `std::mem::async_drop` analogous to `std::mem::drop`:

```rust
async fn async_drop<T>(x: T) { }
```

### Implicit await points

When you run async drop glue, there is an implicit await point. Consider this example:

```rust
async fn foo(dbc: DatabaseConnection) -> io::Result<()> {
    let data = socket().read().await?;
    dbc.write(data).await?;
}
```

Here, presuming that `DatabaseConnection` implements `AsyncDrop`, there are actually a number of async drops occurring:

```rust
async fn foo(dbc: DatabaseConnection) -> io::Result<()> {
    let data = match socket().read().await {
        Ok(v) => v,
        Err(e) => {
            std::mem::async_drop(dbc).await;
            return e;
        }
    };
    let () = match dbc.write(data).await? {
        Ok(()) => (),
        Err(e) => {
            std::mem::async_drop(dbc).await;
            return e;
        }
    };
    std::mem::async_drop(dbc).await;
}
```

As this example shows, there are important ergonomic benefits here to implicit async drop, and it also ensures that async and sync code work in analogous ways. However, implicit await points can be a hazard for some applications, where it is important to identify all await points explicitly (for example, authors of embedded applications use await points to reason about what values will be stored in the resulting future vs the stack of the poll function). To further complicate things, async-drop doesn't only execute at the end of a block or an "abrupt" expression like `?`: async-drop can also execute at the end of every statement, given temporary values.

The best solution here is unclear. We could have an "allow-by-default" lint encouraging explicit use of `async_drop`, but as the code above shows, the result may be highly unergonomic (also, imagine how it looks as the number of variables requiring async-drop grows).

Another option is to target the problem from another angle, for example by adding lints to identify when large values are stored in a future or on the stack, or to allow developers to tag local variables that they expect to be stored on the stack, and have the compiler warn them if this turns out to not be true. Users could then choose how to resolve the problem (for example, by shortening the lifetime of the value so that it is not live across an await).

### Preventing sync drop

It is easy enough to make async-drop be used, but it is currently not possible to prevent sync drop, even from within an async setting. Consider an example such as the following:

```rust
async fn foo(dbc: DatabaseConnection) -> io::Result<()> {
    drop(dbc);
}
```

The compiler could however lint against invoking (or defining!) synchronous functions that take ownership of values whose types implement `AsyncDrop`. This would catch code like the case above. We may have to tune the lint to avoid false warnings. Note that it is important to lint *both* invocation *and* definition sites because the synchronous function may be generic (like `drop`, in fact).

The question remains: what should code that implements `AsyncDrop` *do* if synchronous `Drop` is invoked? One option is panic, but that is suboptimal, as panic from within a destructor is considered bad practice. Another option is to simply abort. A final option is to have some form of portable "block-on" that would work, but this is effectively the (as yet unsolved) [async-sync-async sandwich problem](../../unresolved_questions/async_sync_async_sandwich.md).

Preventing this 'properly' would require changing fundamental Rust assumptions (e.g., by introducing the `?Drop` trait). While such a change would make Rust more expressive, it also carries complexity and composition hazards, and would require thorough exploration. It is also a step that could be taken later (although it would require some form of explicit `impl !Drop` opt-in by types to avoid semver breakage).

### Supporting both sync and async drop

Final point: it should perhaps be possible to support *both* sync and async drop. It is not clear though if there are any real use cases for this.