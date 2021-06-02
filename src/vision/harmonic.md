# 🎼 The Harmonic Synthesis

## Key goals and points

By the time we reach our shiny future...

* **Working in async Rust should feel "analogous" to sync Rust to the extent possible:**
    * You should be able to write `async fn` anywhere you can write `fn` and things should generally work.
    * You should be able to rely on destructors to cleanup 
    * There should be a combination of quality learning resources, compiler diagnostics, helpful lints, and runtime feedback that ensures that, when you do have a problem, you can quickly understand what went wrong and how to fix it (this should be true both when getting started and at intermediate levels).
* **The complexity that underpins (pun intended) async Rust should be largely hidden from view:**
    * You should be able to do everything you want to do by writing `async fn`; implementing combinators and poll functions manually should not be necessary.
* **We should enable seamless interoperability both within the Rust ecosystem and across languages:**
    * It should be easy to write libraries that work across different runtimes.
    * Your async programs should be portable across different runtimes by default, enabling easy experimentation.
    * You should be able to interoperate with native runtimes available on your platform and the async mechanisms from other languages.
* **You should not be *surprised* by the semantics of your program; we want async Rust to have that same "if it compiles, it works as expected" feeling that sync Rust gives you:**
    * It should be easy to create parallel/concurrent tasks and create complex scheduling structures ("at most 5 of these requests at a time, combined with 1 of those") that work reliably and run efficiently. These tasks should be able to access borrowed data using a scope-like mechanism.
    * Cancellation should be both reliable (cancelling everything it makes sense to cancel) and unsurprising (not interrupting work in the middle).
    * Combining libraries and utilities from crates.io should not result in surprising panics nor in surprising event loops running in the background.    
* **We should help you to maintain and optimize your system over its lifecycle:**
    * It should be easy to figure out where your program is spending its time and to identify common async performance footguns and hazards.
    * It should be easy to writing testing harnesses to test your logic, simulate extreme networking events, and so forth.
* **The system should scale to all the niches and key use cases:**
    * Enabling zero-copy and the efficient use of io-uring
    * Enabling code that doesn't require thread-safety (e.g., worker per core) to avoid overhead from atomics
    * Writing efficient servers and networking hardware
    * Writing simpler servers and web frameworks
    * Writing networking clients 
    * Writing GUIs and games
    * Working on embedded platforms that do not have an operating system or standard library
    * Working on web assembly (exactly what this means is a bit of an open question)

## Some specific technical challenges

Achieving the previous points will not be easy! We think we have an outline of how to do it, but there are some key technical challenges to overcome.
Most or all of these have seen a lot of prior discussion:

* Traits that support async fn and offer various ways to make that dyn safe
* Async closures
* Generators to make writing iterators and async iterators easy
* Standard traits for portability tht are built on `async fn` primities (not the poll-based traits offered today):
    * async read
    * async write
    * async iteration
    * timers
    * opening TCP, UDP ports, potentially other standard I/O primities
    * spawning, spawn-blocking
* Easy, standard patterns for doing common I/O operations without specifying a runtime
    * presumably interacting with the traits above, but hopefully not requiring all portable code to be deeply generic
* Standard combinators for async iterators, futures, implemented largely using async fn
* Structured conncurrency: a way to launch tasks and to propagate cancellation downwards
    * This should provide alternatives to `FuturesUnordered` and stream that are not as error-prone
* A mechianism for being generic over sync vs async code
* Defining some form of drop impl that will run asynchronously
    * There remains debate about whether this is desirable. We argue that it is, because otherwise async code will always feel distinct from `Drop` code.

## Skill tree

What follows is a "skill tree" that describes the key points of the harmonic synthesis. Each of the "blocks" in the tree represents a kind of coherent experience that we can achieve. There are a few sentences describing the impact of this experience on users, and then a list of technical items that must be implemented to achieve that goal. Note that we can work on many of the technical items in parallel -- including items that appear relatively late in the tree.

Eventually we aim to have each of the nodes in this tree have an associated "shiny future" story spelling it out in more detail.

```skill-tree
[graphviz]
rankdir = "TD"

[doc]
columns = ["status"]

[doc.defaults]
status = "tbd"
assigned = "no"

[doc.emoji.status]
"tbd" = "▯▯▯▯▯"
"exploration" = "▮▯▯▯▯"
"design" = "▮▮▯▯▯"
"implementation" = "▮▮▮▯▯"
"stabilization" = "▮▮▮▮▯"
"done" = "▮▮▮▮▮"

[doc.emoji.assigned]
"no" = "✋"
"yes" = "✍️"
"blocked" = "🛑"


[[group]]
name = "async-traits"
label = "Async traits are possible"
description = [
  "Possible to write async abstractions using traits",
  "Working with dyn-safety requires careful work",
]
items = [
  { label = "Type alias impl Trait", status = "implementation" },
  { label = "Generic associated types", status = "implementation" },
]

[[group]]
name = "async-fn-everywhere"
label = "Async fn everywhere"
description = [
  "Write async fn anywhere you can write fn",
  "Write async closures anywhere you can write sync closures",
]
requires = [
  "async-traits",
]
items = [
  { label = "Support for `dyn Trait` where `Trait` has async fn", status = "design" },
  { label = "Async fn sugar in traits", status = "design" },
  { label = "Async closure support", status = "design", assigned = "blocked" },
  { label = "Boxable, recursive async fn", status = "design" },
]

[[group]]
name = "async-iter"
label = "Async iteration is awesome"
description = [
  "Use async iterators as easily as sync iterators",
  "Write async and sync iterators with equal ease",
]
requires = [
  "async-fn-everywhere",
]
items = [
  { label = "AsyncIterator trait", status = "implementation" },
  { label = "Common combinators on AsyncIterator", status = "implementation", assigned = "blocked" },
  { label = "Generators (both sync and async)", status = "design", assigned = "yes" },
  { label = "Easy conversion of sync iter to async iter", status = "design", assigned = "blocked" },
]

[[group]]
name = "async-read-and-write"
label = "Async read and write are a pleasure to use"
description = [
  "Easy to pass around interoperable readers and writers",
  "Easy to impl AsyncRead and AsyncWrite traits",
  "Easy to write adapters that wrap async read and async write",
]
requires = [
  "async-fn-everywhere",
]
items = [
  { label = "AsyncRead trait in std", status = "implementation" },
  { label = "AsyncWrite trait in std", status = "implementation" },
  { label = "TBD: some way to write poll fns easily", status = "exploration" },
]

[[group]]
name = "portability-is-possible"
label = "Portability across runtimes is possible"
description = [
  "Grab a library from crates.io and<br/>it works with your chosen runtime,<br/>as long as the author does a good job",
  "Possible to author libraries<br/>that can be used with many runtimes,<br/>but requires careful use of traits",
  "Create a new runtime and have existing<br/>(portable) libraries work with no modifications",
]
requires = [
  "async-iter",
  "async-read-and-write",
]
items = [
  { label = "Trait for spawning tasks", status = "exploration" },
  { label = "Trait for spawning blocking tasks", status = "exploration" },
  { label = "Trait for timers", status = "exploration" },
  { label = "Common utilities like select, join, mutexes", status = "design" },
]

[[group]]
name = "getting-started"
label = "Getting started in async Rust is a smooth experience"
description = [
  "Easy to find, quality resources for learning async Rust",
  "Use Tcp Streams and other constructs without baking in a specific runtime or implementation",
  "Use Tcp Streams without threading generics all throughout your code",
  "Compiler and error messages help you avoid common mistakes",
  "Design patterns are well known",
]
requires = [
  "portability-is-possible",
]
items = [
  { label = "Improve async book structure", status = "implementation" },
  { label = "Lint for blocking functions in an async context", status = "design" },
  { label = "Lint holding things over an await that should not be held over an await", status = "implementation" },
  { label = "Work on identifying common design patterns", status = "implementation" },
]

[[group]]
name = "resolving-problems"
label = "Resolving problems in running applications is easy"
description = [
  "Using a debugger with Rust basically works",
  "Find out what tasks your program currently has",
  "Show why tasks are blocked",
  "Detect common pitfalls and async hazards",
]
requires = [
  "getting-started",
]
items = [
  { label = "Better debuginfo", status = "exploration" },
  { label = "Runtime Debugger interface", status = "exploration" },
  { label = "Runtime bug detectors", status = "exploration" },
  { label = "Improve async book structure", status = "implementation" },
  { label = "Work on identifying common design patterns", status = "implementation" },
]

[[group]]
name = "performance-tooling"
label = "Me make Rust FAST"
description = [
  "Profiles of specific tasks (heap, memory, etc)",
  "Long-running sequential loops are easy to find and remedy",
  "Overall profiles",
]
requires = [
  "resolving-problems"
]
items = [
  { label = "Lint for functions that will take too long to execute", status = "design" },
  { label = "Runtime warnings in debug mode", status = "exploration" },
  { label = "Profiler-guided lints", status = "design" },
  { label = "Combinators and hooks to make it easy to yield in long-running loops", status = "exploration"  },
  { label = "Highly optimized `spawn_blocking`", status = "exploration"  },
  { label = "Turbowish profiler support", status = "design" },
]
[[group]]
name = "async-raii"
label = "Easily manage async cleanup"
description = [
  "Add an async drop and be confident that it will be invoked without effort",
  "Reliably detect cases where sync drop might be used instead",
]
requires = [
  "async-traits",
]
items = [
  { label = "Ability to write an async disposal method", status = "design", assigned = "blocked" },
  { label = "Lint for sync dropping when there's async drop", status = "design" },
]

# [[group]]
# name = "first-class-learning-experience"
# label = "First-class learning experience"
# description = [
#   "When async doesn't work as I expect <br/> (whether at compilation time, runtime, debugging)...",
#   "something identifies the problem",
#   "something explains the problem",
#   "something proposes solutions",
#   "after reading the explanation and the solutions, <br/> I understand what I did wrong",
# ]
# requires = [
#   "resolving-problems",
#   "performance-tooling",
# ]
# items = [
#   { label = "Cross-referencing between docs, lints, errors, and so forth", status = "exploration"  },
# ]

[[group]]
name = "portability-across-send"
label = "Portability across Send"
description = [
  "write code that can be Send or not-Send at zero-cost (e.g., use Rc vs Arc)",
]
requires = [
  "portability-is-possible",
]
items = [
  { label = "associated traits", status = "tbd" },
  { label = "module-level generics", status = "tbd" },
]

[[group]]
name = "ffcf"
label = "If it compiles, it works"
description = [
    "Bugs are generally logic bugs, not a result of surprising async mechanisms",
    "Easy to create parallelism operating on borrowed data",
    "When tasks are no longer needed, they can be reliably canceled",
    "It is easy to visualize your program's task structure",
]
requires = [
  "async-raii",
  "getting-started",
]
items = [
  { label = "Way to avoid tasks being dropped unexpectedly while they continue to execute", status = "exploration" },
  { label = "Mechanism for launching tasks within those scopes that can reference borrowed data", status = "design", assigned = "blocked" },
  { label = "Hierarchical structure for tasks with cancelation propagation", status = "exploration" },
  { label = "Lint when potentially canceling 'futures not known to be cancel safe'", status = "exploration" },
  { label = "Integration into the visualization infrastructure and debug tools", status = "design", assigned = "blocked" },
]

[[group]]
name = "zero-copy"
label = "Zero copy works beautifully"
description = [
  "permit zero copy",
]
requires = [
  "ffcf",
]
items = [
  { label = "TBD" },
]

[[group]]
name = "testing"
label = "Testing your async code is easy"
description = [
  "Testing async code does not require a lot of pre-planning",
  "You can easily test connection errors, delays, and other corner cases",
]
requires = [
  "getting-started",
]
items = [
  { label = "Ability to fire timers programmatically (mock time)" },
  { label = "`run_until_stalled`" },
  { label = "Mock sockets and file objects?" },
  { label = "Inject errors or long delays" },
  { label = "Inject fake objects for real ones without modifying your code" },
]
```