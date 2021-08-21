# Runtime

## Impact

* Able to write simple, non-generic async Rust code that performs common operations like opening TCP sockets, sending UDP packets, accessing files, sleeping, and spawning tasks, but which is not specific to a particular runtime.
* Able to retarget code that relies on these APIs across different runtimes with no effort.

## Design notes

When writing sync code, it is possible to simply _access_ I/O and other facilities without needing to thread generics around:

```rust
fn load_socket_addr() -> Result<SocketAddr, Box<dyn Error>> {
    Ok(std::fs::read_to_string("address.txt")?.parse()?)
}
```

This code will work no matter what operating system you run it on.

Similarly, if you don't mind hard-coding your runtime, one can use `tokio` or `async_std` in a similar fashion

```rust
// Pick one:
//
// use tokio as my_runtime;
// use async_std as my_runtime;

async fn load_socket_addr() -> Result<SocketAddr, Box<dyn Error>> {
    Ok(my_runtime::fs::read_to_string("address.txt").await?.parse()?)
}
```

Given suitable traits in the stdlib, it would be possible to write generic code that feels similar:

```rust
async fn load_socket_addr<F: AsyncFs>() -> Result<SocketAddr, Box<dyn Error>> {
    Ok(F::read_to_string("address.txt").await?.parse()?)
}
```

Alternatively, that might be done with `dyn` trait:

```rust
async fn load_socket_addr(fs: &dyn AsyncFs)) -> Result<SocketAddr, Box<dyn Error>> {
    Ok(F::read_to_string("address.txt").await?.parse()?)
}
```

Either approach is significantly more annoying, both as the author of the library and for folks who invoke your library.

### Preferred experience

The ideal would be that you can write an async function that is "as easy" to use as a non-async one, and have it be portable across runtimes:

```rust
async fn load_socket_addr() -> Result<SocketAddr, Box<dyn Error>> {
    Ok(std::async_fs::read_to_string("address.txt").await?.parse()?)
}
```

### But how to achieve it?

The basic idea is to extract out a "core API" of things that a runtime must provide and to make those functions available as part of the `Context` that `Async` values are invoked with. To avoid the need for generics and monomorphization, this would have to be based purely on `dyn` values. This interface ought to be compatible with no-std runtimes as well, which imposes some challenges.

## Frequently asked questions

### What about async overloading?

Good question! The [async overloading](../async_overloading.md) feature may be another, better route to this same goal. At minimum it implies that `std::async_fs` etc might not be the right names (although those modules could be deprecated and merged going forward).

It definitely suggests that the names and signatures of all functions, methods, and types should be kept very strictly analogous. In particular, sync APIs should be a subset of async APIs.

### What about cap-std?

It's interesting to observe that the `dyn` approach is feeling very close to [cap-std](https://blog.sunfishcode.online/introducing-cap-std/). That might be worth taking into consideration. Some targets, like wasm, may well prefer if we took a more "capability oriented" approach.

### What about spawning and scopes?

Given that spawning should occur through scopes, it may be that we don't need a `std::async_thread::spawn` API so much as standards for scopes.

### What about evolving the API?

We will want to be able to start with a small API and grow it. How is that possible, given that the *implementation* of the API lives in external runtimes?

### What methods are needed?

We need to cover the things that exist in the sync stdlib

* spawn, spawn-blocking
* timers (sleep)
* TCP streams, UDP sockets
* file I/O
* channels and other primitives
    * mutexes?
