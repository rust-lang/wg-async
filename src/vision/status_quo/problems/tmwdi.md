# Too many ways to do it

In general, in Async Rust, we have a lot of ways to do things. The standard library and language ship with only the bare minimum: the ability to create a `Future` and to write an `async fn`. Everything else is currently built in the ecosystem:

* runtimes;
* utilities;
* http libraries;
* web and server frameworks;
* traits for interop, like I/O and the like;
* and so forth.

The `futures` crate is part of rust-lang, and contains a number of things headed towards standardization, but it doesn't have stability promises nor the "prestige" of the standard library. Some runtimes, like tokio, have opted not to use traits that live in the `futures` crate, such as `Stream`, because of their uncertain stability guarantees.

On the one hand, having choices is a good thing, because it allows aysnc Rust to be used in more places, and it allows for more experimentation and exploration of new ideas. But right now users are confronted with all of this choice right up front, before they really know much about async Rust.

Furthermore, there are no standard ways to write interoperable libraries, and as a result, users often find that combining different crates in the ecosystem results in bringing in multiple runtimes, which can be either inefficient or sometimes even leads to panics at runtime. These panics undermine their faith in Rust.

Another side-effect of the TMWDI problem is that it is hard to write good documentaton on async Rust. There is little shared vocabulary or functionality, so book authors must select a particular runtime and code to that, and users who are trying to use other runtimes may find that their runtime works in different ways (even when offering equivalent functionality). This problem is particularly acute for the official Rust docs, which generally try not to endorse items from the ecosystem. Similar problems face the compiler, as attempts to offer diagnostics sometimes have to recommend crates from the ecosystem, which raises the risk of "playing favorites".

