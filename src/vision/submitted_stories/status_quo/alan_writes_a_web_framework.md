# 😱 Status quo stories: Alan writes a web framework


[How To Vision: Status Quo]: ../status_quo.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async/master/src/vision/status_quo/template.md
[`status_quo`]: https://github.com/rust-lang/wg-async/tree/master/src/vision/status_quo
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async/blob/master/src/SUMMARY.md
[open issues]: https://github.com/rust-lang/wg-async/issues?q=is%3Aopen+is%3Aissue+label%3Astatus-quo-story-ideas
[open an issue of your own]: https://github.com/rust-lang/wg-async/issues/new?assignees=&labels=good+first+issue%2C+help+wanted%2C+status-quo-story-ideas&template=-status-quo--story-issue.md&title=


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[YouBuy](../../projects/YouBuy.md) is written using an async web framework that predates the stabilization of async function syntax. When [Alan] joins the company, it is using async functions for its business logic, but can't use them for request handlers because the framework doesn't support it yet. It requires the handler's return value to be `Box<dyn Future<...>>`. Because the web framework predates async function syntax, it requires you to take ownership of the request context (`State`) and return it alongside your response in the success/error cases. This means that even with async syntax, an http route handler in this web framework looks something like this (from [the Gotham Diesel example](https://github.com/gotham-rs/gotham/blob/9f10935bf28d67339c85f16418736a4b6e1bd36e/examples/diesel/src/main.rs)):

```rust
// For reference, the framework defines these type aliases.
pub type HandlerResult = Result<(State, Response<Body>), (State, HandlerError)>;
pub type HandlerFuture = dyn Future<Output = HandlerResult> + Send;

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

The handler code is forced to drift to the right a lot, because of the async block, and the lack of ability to use `?` forces the use of a match block, which drifts even further to the right. This goes against [what he has learned from his days writing go](https://github.com/uber-go/guide/blob/master/style.md#reduce-nesting).

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
The handler registration then becomes:
```rust
    router_builder.get("/").to_async(get_products_handler);
```

This allows him to strip out the async blocks in his handlers and use `async fn` instead.

```rust
// Type the library again, in case you've forgotten:
pub type HandlerResult = Result<(State, Response<Body>), (State, HandlerError)>;

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

It's still not fantastically ergonomic though. Because the handler takes ownership of State and returns it in tuples in the result, Alan can't use the `?` operator inside his http request handlers. If he tries to use `?` in a handler, like this:

```rust
async fn get_products_handler(state: State) -> HandlerResult {
    use crate::schema::products::dsl::*;

    let repo = Repo::borrow_from(&state);
    let prods = repo
        .run(move |conn| products.load::<Product>(&conn))
        .await?;
    let body = serde_json::to_string(&prods).expect("Failed to serialize prods.");
    let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
    Ok((state, res))
}
```
then he receives:
```ignore
error[E0277]: `?` couldn't convert the error to `(gotham::state::State, HandlerError)`
  --> examples/diesel/src/main.rs:84:15
   |
84 |         .await?;
   |               ^ the trait `From<diesel::result::Error>` is not implemented for `(gotham::state::State, HandlerError)`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = note: required by `std::convert::From::from`
```

Alan knows that the answer is to make another wrapper function, so that the handler can take an `&mut` reference to `State` for the lifetime of the future, like this:

```rust
async fn get_products_handler(state: &mut State) -> Result<Response<Body>, HandlerError> {
    use crate::schema::products::dsl::*;

    let repo = Repo::borrow_from(&state);
    let prods = repo
        .run(move |conn| products.load::<Product>(&conn))
        .await?;
    let body = serde_json::to_string(&prods).expect("Failed to serialize prods.");
    let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
    Ok(res)
}
```
and then register it with:
```rust
    route.get("/").to_async_borrowing(get_products_handler);
```

but Alan can't work out how to express the type signature for the `.to_async_borrowing()` helper function. He submits his `.to_async()` pull-request upstream as-is, but it nags on his mind that he has been defeated.

Shortly afterwards, someone raises a bug about `?`, and a few other web framework contributors try to get it to work, but they also get stuck. When Alan tries it, the compiler diagnostics keep sending him around in circles <!-- TODO: examples of this. Maybe move this into the paragraph above? -->. He can work out how to express the lifetimes for a function that returns a `Box<dyn Future + 'a>` but not an `impl Future` because of how where clauses are expressed. Alan longs to be able to say "this function takes an async function as a callback" (`fn register_handler(handler: impl async Fn(state: &mut State) -> Result<Response, Error>)`) and have Rust elide the lifetimes for him, like how they are elided for async functions.

A month later, one of the contributors finds a forum comment by [Barbara] explaining how to express what Alan is after (using higher-order lifetimes and a helper trait). They implement this and merge it. The final `.to_async_borrowing()` implementation ends up looking like this (also from [Gotham](https://github.com/gotham-rs/gotham/blob/89c491fb4322bbc6fbcc8405c3a33e0634f7cbba/gotham/src/router/builder/single.rs)):

```rust
pub trait AsyncHandlerFn<'a> {
    type Res: IntoResponse + 'static;
    type Fut: std::future::Future<Output = Result<Self::Res, HandlerError>> + Send + 'a;
    fn call(self, arg: &'a mut State) -> Self::Fut;
}

impl<'a, Fut, R, F> AsyncHandlerFn<'a> for F
where
    F: FnOnce(&'a mut State) -> Fut,
    R: IntoResponse + 'static,
    Fut: std::future::Future<Output = Result<R, HandlerError>> + Send + 'a,
{
    type Res = R;
    type Fut = Fut;
    fn call(self, state: &'a mut State) -> Fut {
        self(state)
    }
}

pub trait HandlerMarker {
    fn call_and_wrap(self, state: State) -> Pin<Box<HandlerFuture>>;
}

impl<F, R> HandlerMarker for F
where
    R: IntoResponse + 'static,
    for<'a> F: AsyncHandlerFn<'a, Res = R> + Send + 'static,
{
    fn call_and_wrap(self, mut state: State) -> Pin<Box<HandlerFuture>> {
        async move {
            let fut = self.call(&mut state);
            let result = fut.await;
            match result {
                Ok(data) => {
                    let response = data.into_response(&state);
                    Ok((state, response))
                }
                Err(err) => Err((state, err)),
            }
        }
        .boxed()
    }
}

...
    fn to_async_borrowing<F>(self, handler: F)
    where
        Self: Sized,
        F: HandlerMarker + Copy + Send + Sync + RefUnwindSafe + 'static,
    {
        self.to(move |state: State| handler.call_and_wrap(state))
    }
```

Alan is still not sure whether it can be simplified.

Later on, other developers on the project attempt to extend this approach to work with closures, but they encounter limitations in rustc that seem to make it not work ([rust-lang/rust#70263](https://github.com/rust-lang/rust/issues/70263)).

When Alan sees another open source project struggling with the same issue, he notices that Barbara has helped them out as well. Alan wonders how many people in the community would be able to write `.to_async_borrowing()` without help.

## 🤔 Frequently Asked Questions

### What are the morals of the story?

* Callback-based APIs with async callbacks are a bit fiddly, because of the `impl Future` return type forcing you to write where-clause-soup, but not insurmountable.
* Callback-based APIs with async callbacks that borrow their arguments are almost impossible to write without help.

### What are the sources for this story?

* This is from the author's [own experience](https://github.com/rust-lang/wg-async/issues/78#issuecomment-808193936).

### Why did you choose Alan/YouBuy to tell this story?

* Callback-based apis are a super-common way to interact with web frameworks. I'm not sure how common they are in other fields.

### How would this story have played out differently for the other characters?

* I suspect that even many Barbara-shaped developers would struggle with this problem.

[character]: ../../characters.md
[status quo stories]: ../status_quo.md
[Alan]: ../../characters/alan.md
[Grace]: ../../characters/grace.md
[Niklaus]: ../../characters/niklaus.md
[Barbara]: ../../characters/barbara.md
[htvsq]: ../status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
