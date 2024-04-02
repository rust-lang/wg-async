# 😱 Status quo stories

## 🚧 Under construction! Help needed! 🚧

We are still in the process of drafting the vision document. The stories you see on this page are examples meant to give a feeling for how a status quo story looks; you can expect them to change. See the ["How to vision"][htv] page for instructions and details.

[htv]: ../how_to_vision.md

## What is this

The "status quo" stories document the experience of using Async Rust today. Each story narrates the challenges encountered by [one of our characters][cc] as they try (and typically fail in dramatic fashion) to achieve their goals.

[cc]: ../characters.md

Writing the "status quo" stories helps us to compensate for the [curse of knowledge][cok]: the folks working on Async Rust tend to be experts in Async Rust. We've gotten used to the workarounds required to be productive, and we know the little tips and tricks that can get you out of a jam. The stories help us gauge the cumulative impact all the paper cuts can have on someone still learning their way around. This gives us the data we need to prioritize.

[cok]: https://en.wikipedia.org/wiki/Curse_of_knowledge

### Based on a true story

These stories may not be true, but they are not fiction. They are based on real-life experiences of actual people. Each story contains a "Frequently Asked Questions" section referencing sources used to create the story. In some cases, it may link to notes or summaries in the [conversations] section, though that is not required. The "Frequently Asked Questions" section also contains a summary of what the "morals" of the story are (i.e., what are the key takeaways), along with answers to questions that people have raised along the way.

[conversations]: ../../conversations.md

### The stories provide data we use to prioritize, not a prioritization itself

**Just because a user story is represented here doesn't mean we're going to be able to fix it right now.** Some of these user stories will indicate more severe problems than others. As we consider the stories, we'll select some subset to try and address; that choice is reflected in the [roadmap].

[roadmap]: ../roadmap.md

## Metanarrative

*What follows is a kind of "metanarrative" of using async Rust that summarizes the challenges that are present today. At each point, we link to the various stories; you can read the full set in the table of contents on the left. We would like to extend this to also cover some of its glories, since reading the current stories is a litany of difficulties, but obviouly we see great promise in async Rust. Note that many stories here appear more than once.*

Rust strives to be a language that brings together performance, productivity, and correctness. Rust programs are designed to surface bugs early and to make common patterns both ergonomic and efficient, leading to a sense that "if it compiles, it generally works, and works efficiently". Async Rust aims to extend that same feeling to an async setting, in which a single process interweaves numerous tasks that execute concurrently. Sometimes this works beautifully. However, other times, the reality falls short of that goal.

<details><summary>Making hard choices from a complex ecosystem from the start</summary>

The problems begin from the very first moment a user starts to try out async Rust. The async Rust support in Rust itself is very basic, consisting only of the core Future mechanism. Everything else -- including the basic async runtimes themselves -- lives in user space. This means that users must make a number of choices from the very beginning:

* what runtime to use
    * [Barbara makes their first foray into async](status_quo/barbara_makes_their_first_steps_into_async.md)
    * [Niklaus wants to share knowledge](status_quo/niklaus_wants_to_share_knowledge.md)
* what http libraries to use
    * [Barbara anguishes over http](status_quo/barbara_anguishes_over_http.md)
* basic helpers and utility crates are hard to find, and there are many choices, often with subtle differences between them
    * [Barbara needs async helpers](status_quo/barbara_needs_async_helpers.md)
* Furthermore, the async ecosystem is fractured. Choosing one library may entail choosing a specific runtime. Sometimes you may wind up with multiple runtimes running at once. But sometimes you want that!
    * [Alan started trusting the rust compiler but then async](status_quo/alan_started_trusting_the_rust_compiler_but_then_async.md)
    * [Barbara needs async helpers](status_quo/barbara_needs_async_helpers.md)
* Of course, sometimes you *want* multiple runtimes running together
    * [Alan has an external event loop and wants to use futures/streams](https://rust-lang.github.io/wg-async/vision/status_quo/alan_has_an_event_loop.html)
    * 🚧 [Need more stories about multiple runtimes working together](https://github.com/rust-lang/wg-async/issues/183)
* There is a lack of common, standardized abstractions, which means that often there are multiple attempts to establish common traits and different libraries will employ a distinct subset.
    * [`Sink` is not implemented by async-std websockets](status_quo/alan_tries_a_socket_sink.md)
    * 🚧 [No standardized lower-level traits for read, write, iterators in an async setting](https://github.com/rust-lang/wg-async/issues/177)
    * 🚧 [Lack of widely used higher-level abstractions (like those tower aims to provide)](https://github.com/rust-lang/wg-async/issues/178)
    * 🚧 [Tokio has `Stream` support in tokio-stream for stability concerns](https://github.com/rust-lang/wg-async/issues/179)
* Some of the problems are due to the design of Rust itself. The coherence rules in particular.
    * 🚧 [Write about how coherence makes it nearly impossible to establish standard traits outside of libstd.](https://github.com/rust-lang/wg-async/issues/180)

</details>

<details><summary>Once your basic setup is done, the best design patterns are subtle and not always known.</summary>

Writing async programs turns out to have all kinds of subtle tradeoffs. Rust aims to be a language that gives its users control, but that also means that users wind up having to make a lot of choices, and we don't give them much guidance.

* If you need synchronization, you might want an async lock, but you might want a synchronous lock, it's hard to know.
    * [Alan thinks he needs async locks](status_quo/alan_thinks_he_needs_async_locks.md)
* Mixing sync and async code is tricky and it's not always obvious how to do it -- something it's not even clear what is "sync" (how long does a loop have to run before you can consider it blocking?)
    * [Barbara bridges sync and async](status_quo/barbara_bridges_sync_and_async.md)
    * [Barbara compares some C++ code](status_quo/barbara_compares_some_cpp_code.md)
    * [Alan thinks he needs async locks](status_quo/alan_thinks_he_needs_async_locks.md) -- "locks are ok if they don't take too long"
* There are often many options for doing things like writing futures or other core concepts; which libraries or patterns are best?
    * [Barbara needs async helpers](status_quo/barbara_needs_async_helpers.md)
    * [Grace wants to integrate c api](status_quo/grace_wants_to_integrate_c_api.html#the-second-problem-doing-this-many-times)
    * [Barbara plays with async](status_quo/barbara_plays_with_async.md), where she tries a number of combinations before she lands on `Box::pin(async move { .. })`
* If you would to have data or task parallel operations, it's not always obvious how to do that
    * [Barbara plays with async](status_quo/barbara_plays_with_async.md)
    * [Barbara tries async streams](status_quo/barbara_tries_async_streams.md)
    * [Niklaus builds a hydrodynamic simulator](status_quo/niklaus_simulates_hydrodynamics.md)
* Sometimes it's hard to understand what will happen when the code runs
    * [Grace wants to integrate c api](status_quo/grace_wants_to_integrate_c_api.html#the-second-problem-doing-this-many-times)
    * [Barbara bridges sync and async](status_quo/barbara_bridges_sync_and_async.md)
* Sometimes async may not even be the right solution
    * [Niklaus builds a hydrodynamic simulator](status_quo/niklaus_simulates_hydrodynamics.md)
    * 🚧 [Avoiding async entirely](https://github.com/rust-lang/wg-async/issues/58)

</details>

<details><summary>Even once you've chosen a pattern, gettings things to compile can be a challenge.</summary>

* Async fn doesn't work everywhere
    * [not in traits](status_quo/alan_needs_async_in_traits.md)
    * not in closures -- [barbara plays with async](status_quo/barbara_plays_with_async.md)
    * [barbara needs async helpers](status_quo/barbara_needs_async_helpers.md)
* Recursion doesn't work
    * [barbara needs async helpers](status_quo/barbara_needs_async_helpers.md)
* Things have to be Send all the time, some things can't live across an await
    * [send isn't what it means anymore](https://tomaka.medium.com/a-look-back-at-asynchronous-rust-d54d63934a1c)
    * [alan thinks he needs async locks](status_quo/alan_thinks_he_needs_async_locks.md)
* The tricks you know from Sync rust apply but don't quite work
    * e.g., Box::pin, not Box::new -- [barbara plays with async](status_quo/barbara_plays_with_async.md)
* Sometimes you have to add `boxed`
    * [Grace tries new libraries](status_quo/grace_tries_new_libraries.md)
* Writing strings is hard
    * [Grace wants to integrate a C API](status_quo/grace_wants_to_integrate_c_api.html#the-second-problem-doing-this-many-times)
* When you stray from the happy path, the complexity cliff is very steep
    * Working with Pin is really hard, but necessary in various scenarios
        * 🚧 [Need a story about implementing async-read, async-write](https://github.com/rust-lang/wg-async/issues/181)
        * [Alan hates writing a stream](status_quo/alan_hates_writing_a_stream.md)
    * It's easy to forget to invoke a waker
        * [Alan hates writing a stream](status_quo/alan_hates_writing_a_stream.html#-frequently-asked-questions)
        * [Grace deploys her service](status_quo/grace_deploys_her_service.md)
    * Ownership and borrowing rules get really complicated when async is involved
        * [Alan writes a web framework](status_quo/alan_writes_a_web_framework.md)
    * Sometimes you want `&mut` access that ends while the future is suspended
        * [Alan lost the world](status_quo/alan_lost_the_world.md)
        * [Ghostcell](status_quo/barbara_wants_to_use_ghostcell.md)
    * Writing executors is pretty non-trivial, things have to have references to one another in a way that is not very rusty
        * [barbara builds an async executor](status_quo/barbara_builds_an_async_executor.md)

</details>

<details><summary>Once you get it to compile, things don't "just work" at runtime, or they may be unexpectedly slow.</summary>

* Libraries are tied to particular runtimes and those runtimes can panic when combined, or require special setup
    * [Alan started trusting the rust compiler but then async](status_quo/alan_started_trusting_the_rust_compiler_but_then_async.md)
    * [Alan picks a web server](status_quo/alan_picks_web_server.md)
* Cancellation can in principle occur at any point in time, which leads to subtle bugs
    * [Alan builds a cache](status_quo/alan_builds_a_cache.md)
    * [Alan finds dropping database handles is hard](status_quo/alan_finds_database_drops_hard.md)
    * [Barbara gets burned by select](https://github.com/rust-lang/wg-async/pull/169)
* Dropping is synchronous but sometimes wants to do asynchronous things and block for them to complete
    * [Alan finds dropping database handles is hard](status_quo/alan_finds_database_drops_hard.md)
* Nested awaits mean that outer awaits cannot make progress
    * [Barbara battles buffered streams](status_quo/barbara_battles_buffered_streams.md)
* Async functions let you build up large futures that execute without allocation, which is great, but can be its own cost
    * [Alan iteratively regresses](status_quo/alan_iteratively_regresses.md)
    * [Alan runs into stack allocation trouble](status_quo/alan_runs_into_stack_trouble.md)
* It's easy to have async functions that inadvertently spend too long in between awaits
    * [Barbara compares some C++ code](status_quo/barbara_compares_some_cpp_code.md)

</details>

<details><summary>When you have those problems, you can't readily debug them or get visibility into what is going on.</summary>

* The state of the executor can be very opaque: what tasks exist? why are they blocked?
    * [Alan tries to debug a hang](status_quo/alan_tries_to_debug_a_hang.md)
    * [Barbara tries unix socket](status_quo/barbara_tries_unix_socket.md)
    * [Barbara wants async insights](status_quo/barbara_wants_async_insights.md)
    * [Grace deploys her service](status_quo/grace_deploys_her_service.md)
* Stacktraces are full of gobbly gook and hard to read.
    * [Barbara trims a stacktrace](status_quo/barbara_trims_a_stacktrace.md)
* Tooling doesn't work as well with async or just plain doesn't exist.
    * [Grace waits for gdb](status_quo/grace_waits_for_gdb_next.md)
    * [Alan iteratively regresses](status_quo/alan_iteratively_regresses.md)

</details>

<details><summary>Rust has always aimed to interoperate well with other languages and to fit itself into every niche, but that's harder with async.</summary>

* Runtimes like tokio and async-std are not designed to "share ownership" of the event loop with foreign runtimes
    * [Alan has an event loop](status_quo/alan_has_an_event_loop.md)
* Embedded environments can have pretty stringent requirements; Future was designed to be minimal, but perhaps not minimal enough
    * [Barbara carefully discusses embedded future](status_quo/barbara_carefully_dismisses_embedded_future.md)
* Evolving specs for C and C++ require careful thought to integrate with async Rust's polling model
    * 🚧 [Wrapping C++ APIs in Rust Futures](https://github.com/rust-lang/wg-async/issues/67)
    * 🚧 [Write about the challenges of io-uring integration](https://github.com/rust-lang/wg-async/issues/182)
* Advanced new techniques like [Ghostcell](status_quo/barbara_wants_to_use_ghostcell.md) may not fit into the traits as designed

</details>
