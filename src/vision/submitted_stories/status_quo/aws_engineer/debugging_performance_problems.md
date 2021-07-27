# Status quo of an AWS engineer: Debugging overall performance loss

Alan's service is working better and better, but performance is still lagging from where he hoped it would be. It seems to be about 20% slower than the Java version! After [calling in Barbara to help him diagnose the problem](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_iteratively_regresses.html), Alan identifies one culprit: Some of the types in Alan's system are really large! The system seems to spend a surprising amount of time just copying bytes. Barbara helped Alan diagnose this by showing him some hidden rustc flags, tinkering with his perf setup, and a few other tricks.

There is still a performance gap, though, and Alan's not sure where it could be coming from. There are a few candidates:

* Perhaps they are not using tokio's scheduler optimally.
* Perhaps the memory allocation costs introduced by the `#[async_trait]` are starting to add up.

Alan tinkers with jemalloc and finds that it does improve performance, so that's interesting, but he'd like to have a better understanding of *why*.