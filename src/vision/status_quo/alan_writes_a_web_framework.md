# 😱 Status quo stories: Template


[How To Vision: Status Quo]: ../how_to_vision/status_quo.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async-foundations/master/src/vision/status_quo/template.md
[`status_quo`]: https://github.com/rust-lang/wg-async-foundations/tree/master/src/vision/status_quo
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async-foundations/blob/master/src/SUMMARY.md
[open issues]: https://github.com/rust-lang/wg-async-foundations/issues?q=is%3Aopen+is%3Aissue+label%3Astatus-quo-story-ideas
[open an issue of your own]: https://github.com/rust-lang/wg-async-foundations/issues/new?assignees=&labels=good+first+issue%2C+help+wanted%2C+status-quo-story-ideas&template=-status-quo--story-issue.md&title=


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[YouBuy](../projects/YouBuy.md) is written using an async web framework that predates the stabilization of async function syntax. When [Alan] joins the company, it is using async functions for its business logic, but can't use them for request handlers because the framework doesn't support it yet. It requires the handler's return value to be `Box<dyn Future<...>>`. Because the web framework predates async function syntax, it requires you to take ownership of the request context (`State`) and return it alongside your response in the success/error cases. This means that even with async syntax, an http route handler in this web framework looks something like this (from [the Gotham Diesel example](https://github.com/gotham-rs/gotham/blob/9f10935bf28d67339c85f16418736a4b6e1bd36e/examples/diesel/src/main.rs)):

```rust
fn get_products_handler(state: State) -> Pin<Box<HandlerFuture>> {
    use crate::schema::products::dsl::*;

    async move {
        let repo = Repo::borrow_from(&state);
        let result = repo.run(move |conn| products.load::<Product>(&conn)).await;
        match result {
            Ok(prods) => {
                let body = serde_json::to_string(&prods).expect("Failed to serialize prods.");
                let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
                Ok((state, res))
            }
            Err(e) => Err((state, e.into())),
        }
    }
    .boxed()
}
```
and then it is registered like this:
```rust
    router_builder.get("/").to(get_products_handler);
```

The handler code is forced to drift to the right a lot, because of the async block, and the lack of ability to use `?` forces the use of a match block, which drifts even further to the right.

Rather than switching YouBuy to a different web framework, Alan decides to contribute to the web framework himself. After a bit of a slog and a bit of where-clause-soup, he manages to make the web framework capable of using an `async fn` as an http request handler. He does this by extending the router builder with a closure that boxes up the `impl Future` from the async fn and then passes that closure on to `.to()`.

```rust
    fn to_async<H, Fut>(self, handler: H)
    where
        Self: Sized,
        H: (FnOnce(State) -> Fut) + RefUnwindSafe + Copy + Send + Sync + 'static,
        Fut: Future<Output = HandlerResult> + Send + 'static,
    {
        self.to(move |s: State| handler(s).boxed())
    }
```

This allows him to strip out the async blocks in his handlers and use `async fn` instead.

```rust
async fn get_products_handler(state: State) -> HandlerResult {
    use crate::schema::products::dsl::*;

    let repo = Repo::borrow_from(&state);
    let result = repo.run(move |conn| products.load::<Product>(&conn)).await;
    match result {
        Ok(prods) => {
            let body = serde_json::to_string(&prods).expect("Failed to serialize prods.");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
            Ok((state, res))
        }
        Err(e) => Err((state, e.into())),
    }
}
```

It's still not fantastically ergonomic though. [[Because the web framework predates async function syntax, it requires you to take ownership of the request context (`fn handler(state: State)`) and return it alongside your response in the success/error cases (`Ok((state, response))` or `Err((state, error))`).]] This means that Alan can't use the `?` operator inside his http request handlers. Alan knows answer is to make another wrapper function so that the handler can take an `&mut` reference to `State` for the lifetime of the future, but Alan can't work out how to express this. He submits his pull-request upstream as-is, but it nags on his mind that he has been defeated.

Shortly afterwards, someone raises a bug about `?`, and a few other web framework contributors try to get it to work, but they also get stuck. When Alan tries it, the compiler diagnostics keep sending him around in circles. He can work out how to express the lifetimes for a function that returns a `Box<dyn Future + 'a>` but not an `impl Future` because of how where clauses are expressed. Alan longs to be able to say "this function takes an async function as a callback" (`fn register_handler(handler: impl async Fn(state: &mut State) -> Result<Response, Error>)`) and have Rust elide the lifetimes for him, like how they are elided for async functions.

A month later, one of the contributors finds a forum comment by Barbara explaining how to express what the Alan is after (using higher-order lifetimes and a helper trait). They implement this and merge it.

When Alan sees another open source project struggling with the same issue, he notices that Barbara has helped them out as well.

## 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * Callback-based APIs with async callbacks are a bit fiddly, because of the `impl Future` return type, but not insurmountable.
    * Callback-based APIs with async callbacks that borrow their arguments are almost impossible to write without help.
* **What are the sources for this story?**
    * This is from the author's [own experience](https://github.com/rust-lang/wg-async-foundations/issues/78#issuecomment-808193936).
* **Why did you choose Alan/YouBuy to tell this story?**
    * Callback-based apis are a super-common way to interact with web frameworks. I'm not sure how common they are in other fields.
* **How would this story have played out differently for the other characters?**
    * I suspect that even many Barbara-shaped developers would struggle with this problem.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
