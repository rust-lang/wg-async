# Yield-safe lint

## Use-case

Some types should not be held across a "yield" bound. A typical example is a `MutexGuard`:

```rust,ignore
async fn example(x: &Lock<u32>) {
    let data = x.lock().unwrap();
    something().await;
    *data += 1;
}

async fn something() { }
```

In practice, a lot of these issues are avoided because `MutexGuard` is not `Send`, but single-thread runtimes hit these issues.

## Types where this would apply

* `MutexGuard` for mutexes, read-write locks
* Guards for ref-cells
* Things that might use these types internally and wish to bubble it up

## Precedent and related questions

* The `#[must_use]` lint on types, we would want their design to work very closely.
* Non-async-friendly functions like `sleep` or `task::block_on`.