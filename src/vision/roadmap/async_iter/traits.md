# Async iteration

## Impact

* Able to write code that takes "something iterable"
* Able to use combinators similar to synchronous `Iterator`
* Able to construct complex, parallel schedules that [can refer to borrow data](../borrowed_data_and_cancellation.md)

## Requires

* [inline async functions](../async_fn/inline_async_fn.md), but it is possible to prototype and experiment without that
* [borrowed data and cancellation](../borrowed_data_and_cancellation.md), but it is possible to prototype and experiment without that

## Design notes

The async iterator trait can leverage [inline async functions](../async_fn_everywhere/inline_async_fn.md):

```rust
#[repr(inline_async)]
trait AsyncIterator {
    type Item;

    async fn next(&mut self) -> Self::Item;
}
```

Note the name change from `Stream` to `AsyncIterator`.

One implication of this change is that pinning is no longer necessary when driving an async iterator. For example, one could now write an async iterator that recursively walks through a set of URLs like so (presuming `std::async_iter::from_fn` and [async closures](https://rust-lang.github.io/async-fundamentals-initiative/design-discussions/async_closures.html)):

```rust
fn explore(start_url: Url) -> impl AsyncIterator {
    let mut urls = vec![start_url];
    std::async_iter::from_fn(async move || {
        if let Some(url) = urls.pop() {
            let mut successor_urls = fetch_successor_urls(url).await;
            urls.extend(successor_urls);
            Some(url)
        } else {
            None
        }
    })
}
```

### Parallel async iteration

We should have combinators like `buffered` that enable *parallel* async iteration, similar to the parallel iterators offered by [rayon]. The core operation here is `for_each` (which processes each item in the iterator):

```rust
trait ParAsyncIter {
    type Item;

    async fn for_each(&mut self, op: impl AsyncFn(Self::Item));
}
```

The `buffered` combinator would be implemented by creating an internal scope and spawning tasks into it as needed.