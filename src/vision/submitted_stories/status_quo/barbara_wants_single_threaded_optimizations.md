# 😱 Status quo stories: Barbara wants single threaded optimizations, but not that much

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Barbara is working on operating system services, all of which benefit from concurrency, but only some of which benefit from parallelism. In cases where a service does not benefit from parallelism, a single-threaded executor is used which allows spawning `!Send` tasks.

Barbara has developed a useful feature as a module within one of her system's single-threaded services. The feature allows for the creation of multiple IPC objects to use within concurrent tasks while caching and reusing some of the heavier computation performed. This is implemented with reference counted interior mutability:

```rust
pub struct IpcHandle {
    cache_storage: Rc<RefCell<IpcCache>>,
    // ...
}

struct IpcCache { /* ... */ }
```

A colleague asks Barbara if she'd be interested in making this code available to other services with similar needs. After Barbara factors the module out into its own crate, her colleague tries integrating it into their service. This fails because the second service needs to hold `IpcHandle`s across yieldpoints and it uses a multi-threaded executor. The multi-threaded executor requires that all tasks implement `Send` so they can be migrated between threads for work stealing scheduling.

### Rejected: both single- and multi-threaded versions

Barbara considers her options to make the crate usable by the multi-threaded system service. She decides against making `IpcHandle` available in both single-threaded and multi-threaded versions. To do this generically would require a lot of boilerplate. For example, it would require manually duplicating APIs which would need to have a `Send` bound in the multi-threaded case:

```rust
impl LocalIpcHandle {
    fn spawn_on_reply<F: Future + 'static>(&mut self, to_spawn: impl Fn(IpcReply) -> F) {
        // ...
    }
}

impl SendIpcHandle {
    fn spawn_on_reply<F: Future + Send + 'static>(&mut self, to_spawn: impl Fn(IpcReply) -> F) {
        // ...
    }
}
```

### Accepted: only implement multi-threaded version

Barbara decides it's not worth the effort to duplicate so much of the crate's functionality, and decides to make the whole library thread-safe:

```rust
pub struct IpcHandle {
    cache_storage: Arc<Mutex<IpcCache>>,
    // ...
}

struct IpcCache { /* ... */ }
```

This requires her to migrate her original system service to use multi-threaded types when interacting with the library. Before the change her service uses only single-threaded reference counting and interior mutability:

```rust
#[derive(Clone)]
struct ClientBroker {
    state: Rc<RefCell<ClientState>>,
}

impl ClientBroker {
    fn start_serving_clients(self) {
        let mut ipc_handle = self.make_ipc_handle_for_new_clients();
        ipc_handle.spawn_on_reply(move |reply| shared_state.clone().serve_client(reply));
        LocalExecutor::new().run_singlethreaded(ipc_handle.listen());
    }

    fn make_ipc_handle_for_new_clients(&self) { /* ... */ }
    async fn serve_client(self, reply: IpcReply) { /* accesses interior mutability... */ }
}
```

In order to be compatible with her own crate, Barbara needs to wrap the shared state of her service behind multi-threaded reference counting and synchronization:

```rust
#[derive(Clone)]
struct ClientBroker {
    state: Arc<Mutex<ClientState>>,
}

impl ClientBroker { /* nothing changed */ }
```

This incurs some performance overhead when cloning the `Arc` and when accessing the `Mutex`. The former is cheap when uncontended on x86 but will have different performance characteristics on e.g. ARM platforms. The latter's overhead varies depending on the kind of `Mutex` used, e.g. an uncontended `parking_lot::Mutex` may only need a few atomic instructions to acquire it. Acquiring many platforms' `std::sync::Mutex` is much more expensive than a few atomics. This overhead is usually not very high, but it does pollute shared resources like cache lines and is multiplied by the number of single-threaded services which make use of such a library.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

In synchronous Rust, choosing the "`Send`ness" of a crate is basically a choice about the concurrency it can support. In asynchronous Rust, one can write highly concurrent programs that still execute using only a single thread, but it is difficult to achieve maximum performance with reusable code.

Abstracting over a library's `Send`ness requires being generic over storage/synchronization types *and* APIs which need to accept user-defined types/tasks/callbacks.

### **What are the sources for this story?**

As of writing, the [Fuchsia](https://fuchsia.dev) operating system had [over 1,500 invocations][st-invocations] of `LocalExecutor::run_singlethreaded`. There were [less than 500 invocations][mt-invocations] of `SendExecutor::run`.[^fuchsia-methods] As of writing the author could not find any widely used support libraries which were not thread-safe.

[st-invocations]: https://cs.opensource.google/search?q=file:rs%20run_singlethreaded&sq=&ss=fuchsia%2Ffuchsia
[mt-invocations]: https://cs.opensource.google/search?q=file:rs%20%5C.run%5C(&ss=fuchsia%2Ffuchsia

`actix-rt`'s [spawn function](https://docs.rs/actix-rt/1.1.1/actix_rt/fn.spawn.html) does not require `Send` for its futures, because each task is polled on the thread that spawned it. However it is very common when using `actix-rt` via `actix-web` to make use of async crates originally designed for `tokio`, whose [spawn function](https://docs.rs/tokio/1.6.1/tokio/fn.spawn.html) does require `Send`.

Popular crates like `diesel` are still designing async support, and it appears they are [likely to require `Send`](https://github.com/diesel-rs/diesel/issues/399#issuecomment-850826567).

[^fuchsia-methods]: There are multiple ways to invoke the different Rust executors for Fuchsia. The other searches for each executor yield a handful of results but not enough to change the relative sample sizes here.

### **Why did you choose *Barbara* to tell this story?**

As an experienced Rustacean, [Barbara] is more likely to be responsible for designing functionality to share across teams. She's also going to be more aware of the specific performance implications of her change, and will likely find it more frustrating to encounter these boundaries.

### **How would this story have played out differently for the other characters?**

A less experienced Rustacean may not even be tempted to define two versions, as the approach Barbara took is pretty close to the "just `.clone()` it" advice often given to beginners.

[character]: ../../characters.md
[status quo stories]: ../status_quo.md
[Alan]: ../../characters/alan.md
[Grace]: ../../characters/grace.md
[Niklaus]: ../../characters/niklaus.md
[Barbara]: ../../characters/barbara.md
[htvsq]: ../status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
