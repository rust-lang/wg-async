# ✏️ Design tenets for async

| Status | Owner |
| --- | --- |
| ⚠️ Draft ⚠️ | nikomatsakis |

**Draft status.** These tenets are a first draft. nikomatsakis plans to incorporate feedback and revise them before they are finalized.

The design tenets describe the key principles that drive our work on async. Hopefully, we are able to achieve and honor all of them all of the time. Sometimes, though, they come into conflict, and we have to pick -- in that case, we prefer the tenet earlier in the list.

1. **Minimal overhead.** Rust Async I/O performance should compare favourably with any other language. In the extreme case, it should be possible to use async/await without any required allocation, although this is unlikely to be a common case in production systems.
2. **Easy to get started, but able to do anything you want.** We should make it simple to write Async I/O code and get things that work reasonably well, but it should be possible for people to obtain fine-grained control as needed.
3. **Async is like sync, but with blocking points clearly identified.** At the highest level, writing a simple program using asynchronous I/O in Rust should be analogous to writing one that uses synchronous I/O, except that one adds `async` in front of function declarations and adds `.await` after each call. We should aim for analogous design between synchronous and asynchronous equivalents. Similarly, streams should be like asynchronous iterators. One should be able to use the same sort of combinators with streams and to iterate over them in analogous ways.
4. **No one true runtime.** We need to be able to hook into existing runtimes in different environments, from embedded environments to runtimes like node.js. Specialized systems need specialized runtimes. 
5. **Library ecosystem is key.** We want to have a strong ecosystem of async crates, utilities, and frameworks. This will require mechanisms to write libraries/utilities/frameworks that are generic and interoperable across runtimes.

## Stress tests

"Stress tests" are important use cases that tend to "stretch" the design. When we are contemplating changes, it's important to look over the stress tests and make sure that they all still work:

* **Single-threaded executors:** Some systems tie each task to a single thread; such tasks should be able to access data that is not `Send` or `Sync`, and the executor for those tasks should be able to be fully optimized to avoid atomic accesses, etc.
* **Multi-threaded executors:** Many systems migrate tasks between threads transparently, and that should be supported as well, though tasks will be required to be `Send`.
* **"Bring your own runtime":** The Rust language itself should not require that you start threads, use epoll, or do any other particular thing.
* **Zero allocation, single task:** Embedded systems might want to be able to have a single task that is polled to completion and which does no allocation whatsoever.
* **Multiple runtimes in one process:** Sometimes people have to combine systems, each of which come with their own event loop. We should avoid assuming there is one global event loop in the system. 
* **Non-Rust based runtimes:** Sometimes people want to integrate into event loops from other, non-Rust-based systems.
* **WebAssembly in the browser:** We want to integrate with WebAssembly.
