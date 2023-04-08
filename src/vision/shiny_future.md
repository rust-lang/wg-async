# ✨ Shiny future

This page represents a complete vision for where we want async to go. This vision is what we believe to be the best way to achieve the [experiences](./how_it_feels.md) that we want async to provide.

## Work in progress

Note that while a lot of the steps needed are fairly clear, several of them also have [significant unknowns or points of controversy](./unresolved_questions.md). We have attempted to highlight those and expect to be working through those points as we go.

## Certainty levels

- 🌈 -- Implemented and stable
- 🌞 -- Everything is looking good
- 🌤️ -- Still some stuff to figure out, but unlikely to see major changes in the design
- 🌥️ -- Got one or two solid leads, but still have to figure out if it will work
- 🌧️ -- No clear path yet, this may not even be a good idea

## Key aspects of the future

* 🌤️ If you know sync Rust, getting started in Async Rust is straightforward ([more](roadmap/async_fn.md)])
    * 🌤️ Mostly, you change `fn` to `async fn`, add some calls to await, and change over to other parts of the stdlib, though supporting `dyn Trait` requires making some choices, particularly in a no-std environment
    * 🌤️ It still has that "if it compiles, it generally works, and it runs pretty darn fast" feeling
    * 🌤️ Destructors and cleanup also work the same way as in sync Rust, thanks to `Drop` to `AsyncDrop`
    * 🌤️ No need to write poll functions or to interact with pin except in quite specialized scenarios
* 🌤️ High-quality documentation and tutorials helps you to get started and learn the ropes
    * 🌤️ The docs also identify common patterns for structuring your async programs and their advantages and disadvantages
* 🌥️ Tooling and debugger integration gives insight into the behavior of your program
    * 🌥️ Easy to get a snapshot of overall acitivity (e.g. to find out what tasks or exist or why a task is blocked)
    * 🌥️ Easy to see aggregate performance trends over time (e.g., number of active connections, waiting connections, etc)
    * 🌥️ Easy to profile things in terms of your async tasks (e.g., to get a flamegraph of a specific connection)
* 🌥️ Variety of high-quality runtimes available in cargo, and it's easy to change between them:
    * 🌧️ When you use things from the standard library, they work across runtimes automatically
    * 🌥️ There are standardized, foundational traits for common operations like I/O, spawning tasks, timers
* 🌥️ Hierarchical scopes allow you to easily spawn parallel and concurrent tasks
    * 🌥️ These can reference borrowed data, enabling easy parallel processing of async iterators (think "async rayon")
* 🌥️ Cancellation works well and without surprises
    * 🌥️ When cancellation is requested, it propagates to subtasks within a scope
    * 🌧️ I/O operations and the like begin to fail, so that cancellation is automatic and flows through familiar error paths
    * 🌥️ If desired, you can "opt-in" to synchronous cancellation, in which case any await becomes a cancellation point. This allows your `async fn` to be used with `select` without spawning a task.

## Learn more

Check out...

* [The user's manual of the future](./shiny_future/users_manual.md)

## Where did all the stories go?

The full set of "submitted" shiny future stories [have been moved here](./submitted_stories/shiny_future.md).
