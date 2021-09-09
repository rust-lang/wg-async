# Async fn everywhere

## Impact

* To a first-order approximation, any place that you can write some sort of Rust function or closure, you should be able to make it asynchronous:
    * [in traits and closures, including the Drop trait](https://rust-lang.github.io/async-fundamentals-initiative/)
    * in [main and tests](./async_fn/async_main_and_tests.md)
* You should be able to [easily create futures that heap allocate their storage](./async_fn/boxable.md), both for performance tuning and for scenarios like recursive functions
