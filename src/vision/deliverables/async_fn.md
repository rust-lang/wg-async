# Async fn everywhere

## Impact

* To a first-order approximation, any place that you can write some sort of Rust function or closure, you should be able to make it asynchronous.
* You should be able to tune the size of futures and easily correct futures that grow too large.
