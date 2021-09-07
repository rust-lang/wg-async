# To await or not to await?

Should we require you to use `.await`? After the epic syntax debates we had, wouldn't it be ironic if we got rid of it altogether, as [carllerche has proposed](https://carllerche.com/2021/06/17/six-ways-to-make-async-rust-easier/)?

Basic idea:

- When you invoke an async function, it could await by default.
- You would write `async foo()` to create an "async expression" -- i.e., to get a `impl Async`.
  - You might instead write `async || foo()`, i.e., create an async closure.

Appealing characteristics:

- **More analogous to sync code.** In sync code, if you want to defer immediately executing something, you make a closure. Same in async code, but it's an async closure.
- **Consistency around async-drop.** If we adopt an [async drop](../roadmap/async_fn/async_fn_fundamentals/async_drop.md) proposal, that implies that there will be "awaits" that occur as you exit a block (or perhaps from the control-flow of a `break` or `?`). These will not be signaled with a `.await`. So you can no longer rely on _every_ await point being visible with a keyword.
- **No confusion around remembering to await.** Right now the compiler has to go to some lengths to offer you messages suggesting you insert `.await`. It'd be nice if you just didn't have to remember.
- **Room for optimization.** When you first invoke an async function, it can immediately start executing; it only needs to create a future in the event that it suspends. This may also make closures somewhat smaller.
  - This could be partially achieved by adding an optional method on the trait that compiles a version of the fn meant to be used when it is _immediately awaited_.

But there are some downsides:

- **Churn.** Introducing a new future trait is largely invisible to users except in that it manifests as version mismatches. Removing the await keyword is a much more visible change.
- **Await points are less visible.** There may be opportunity to introduce concurrency and so forth that is harder to spot when reading the code, particularly outside of an IDE. (In Kotlin, which adopts this model, suspend points are visible in the "gutter" of the editor, but this is not visible when reviewing patches on github.)
  - Await points today also indicate where a live `Send` or `Sync` value will affect if the future is send or sync (but with async-drop, this would no longer be true).
- **Async becomes an effect.** In today's Rust, an "async function" desugars into a traditional function that returns a future. This function is called like any other, and hence it can implement the `Fn` traits and so forth. In this "await-less" Rust, an async function is called differently from other functions, because it induces an await. This means that we need to consider `async` as a kind of "effect" (like `unsafe`) in a way that is not today.
  - Similarly, how do we handle the case of `fn foo() -> impl Future`? Does that auto-await, or does it require an explicit `await` keyword?
  - What happens when you invoke an `async fn` in a sync environment?

## Frequently asked questions

### How could you do this anyway? Wouldn't it be a massive breaking change?

It would have to take place over an edition.
