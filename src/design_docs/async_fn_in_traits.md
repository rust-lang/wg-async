# 🧬 Async fn in traits

* [Why async fn in traits are hard][wafth]

[wafth]: http://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/

## General goal

```rust,ignore
trait Foo {
    // Currently disallowed:
    async fn bar();
}
```

## Concerns

### How to name the resulting future

If you wanted to name the future that results from calling `bar` (or whatever), you can't.

Also true for functions `fn bar() -> impl Trait`.

### Requiring `Send` on futures

[Relevant thread](https://internals.rust-lang.org/t/how-often-do-you-want-non-send-futures/10360)

```rust,ignore
async fn foo() {}

// desugars to
fn foo() -> impl Future<Output = ()> { } // resulting type is Send if it can be

// alternative desugaring we chose not to adopt would require Send
fn foo() -> impl Future + Send { }
```

If I want to constrain the future I get back from a method, it is difficult to do without a name:

```rust,ignore
trait Service {
    async fn request(&self);
}

fn parallel_service<S: Service>()
where
    S::Future: Send,
{
    ...
}
```

* Should this be solved at the impl trait layer
* Or should we specialize something for async functions
* Would be nice, if there are many, associated types, to have some shorthand

## Example use case: the Service

```rust,ignore
trait Service {
    type Future: Future<Output = Response>;

    fn request(&self, ...) -> Self::Future;
}

impl Service for MyService {
    type Future = impl Future<Output = Response>;

    fn request(&self) -> Self::Future {
        async move { .. }
    }
}
```

* Dependent on impl Trait, see lang-team repo

## Example use case: capturing lifetimes of arguments

```rust,ignore
trait MyMethod {
    async fn foo(&self);
}
```

## 🤔 Frequently Asked Questions

* **What do people say about this to their friends on twitter?**
    * (Explain your key points here)
