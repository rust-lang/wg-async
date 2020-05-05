# Stream trait

* [Current definition](https://docs.rs/futures/0.3.1/futures/stream/trait.Stream.html)

## Trait definition

```rust,ignore
pub trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>>;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
```

## Concerns

### Poll-based design

* You have to think about Pin if you implement this trait.
* Combinators can be more difficult.
* One solution: [generator syntax](./generator_trait.md).

### Attached streams are commonly desired

Sometimes streams need to reuse internal storage ([Discussion]).

[Discussion]: http://smallcultfollowing.com/babysteps/blog/2019/12/10/async-interview-2-cramertj-part-2/#the-need-for-streaming-streams-and-iterators

### Combinators

* Currently the combinations are stored in the [`StreamExt`] module.
* In some cases, this is because of the lack of async closures support.
    * Also serves as a "semver barrier".
    * Also no-std compatibility.
* One question: what combinators (if any) to include when stabilizing?
    * e.g., [`poll_next_unpin`] can make working with pin easier, albeit at a loss of generality
        * folks who are new to pinning could use this method, and it can help us to guide the diagnostics by suggesting that they `Box::pin`

[`StreamExt`]: https://docs.rs/futures/0.3.1/futures/stream/trait.StreamExt.html
[`poll_next_unpin`]: https://docs.rs/futures/0.3.1/futures/stream/trait.StreamExt.html#method.poll_next_unpin