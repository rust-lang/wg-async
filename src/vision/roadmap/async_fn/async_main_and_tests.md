# Async main and tests

## Impact

* Able to write `#[test]` that easily use async functions.
* In the case of portable libraries, end users are able to re-run test suites with distinct runtimes.

## Milestones

> Able to write `async fn main` and `#[test] async fn` just like you would in synchronous code.

This initiative is **on hold** while we investigate mechanisms for [portability across runtimes](./roadmap/portable.md).

