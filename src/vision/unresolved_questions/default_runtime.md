# Default runtime?

The [User's Manual of the future](../shiny_future/users_manual.md) suggests that one must still pick a runtime upfront and use a decorator like `#[runtime::main]`. This is "accidental complexity" for people learning async Rust: the choice of runtime is something they are not yet equipped to make. It would be better for users if they could just write `async fn main` and not choose a runtime yet (and then, later, once they are equipped to make the choice, opt for other runtimes).

However, we also wish to avoid shipping and maintaining a runtime in the Rust stdlib. We want runtimes to live in the ecosystem and evolve over time. If we were to pick a "default runtime", that might favor one runtime at the expense of others.

Should we pick a default runtime? If so, what criteria do we use to pick one, and how do we manage the technical side of things (e.g., we need to either ship the runtime with rustup or else insert some kind of implicit cargo dependency).
