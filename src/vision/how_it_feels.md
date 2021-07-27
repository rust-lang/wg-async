# How using async Rust ought to feel (and why it doesn't today)

This section is, in many ways, the most important. It aims to identify the way it should feel to use Async Rust.

## Consistent: "just add async/await"

Async Rust should be a small delta atop Sync Rust. People who are familiar with sync Rust should be able to leverage what they know to make adopting Async Rust straightforward. Porting a sync code base to async should be relatively smooth: just add async/await, adopt the async variants of the various libraries, and you're done.

## Reliable: "if it compiles, it works"

One of the great things about Rust is the feeling of "it if compiles, it works". This is what allows you to do a giant refactoring and find that the code runs on the first try. It is what lets you deploy code that uses parallelism or other fancy features without exhausting fuzz testing and worry about every possible corner case.

## Empowering: "complex stuff feels easy"

Rust's great strength is taking formerly complex, wizard-like things and making them easy to do. In the case of async, that means letting people use the latest and greatest stuff, like io-uring. It also means enabling parallelism and complex scheduling patterns very easily.

## Performant: "ran well right out of the box"

Rust code tends to perform "quite well" right out of the box. You don't have to give up the "nice things" in the language, like closures or high-level APIs, in order to get good performance and tight memory usage. In fact, those high-level APIs often perform as well or better than what you would get if you wrote the code yourself.

## Productive: "great crates for every need, just mix and match"

Being able to leverage a large ecosystem of top-notch crates is a key part of what makes Rust (and most any modern language) productive. When using async Rust, you should be able to search crates.io and find crates that cover all kinds of things you might want to do. You should be able to add those crates to your `Cargo.toml` and readily connect them to one another without surprising hiccups.

## Transparent and tunable: "it's easy to diagnose deadlocks and performance bottlenecks"

Using Rust means most things work and perform well by default, but of course it can't prevent all problems. When you do find bugs, you need to be able to easily track what happened and figure out how to fix it. When your performance is subpar, you need to be able to peek under the covers and understand what's going on so that you can tune it up. In synchronous Rust, this means integrating with but also improving on existing tooling like debuggers and profilers. In asynchronous Rust, though, there's an extra hurdle, because the terms that users are thinking in (asynchronous tasks etc) exist within the runtime, but are not the same terms that synchronous debuggers and profilers expose. There is a need for more customized tooling to help users debug problems without having to map between the async concept and the underlying implementation.

## Control: "I can do all the weird things"

Part of what's great about Rust is that it lets you get into explore all the corner cases. Want to target the kernel? Develop embedded systems using async networking without any operating system? Run on WebAssembly? No problem, we can do that.

## Interoperable: "integrating with C++, node.js, etc is easy"

Much like C, Rust aims to be a "lingua franca", something you can integrate into your existing systems on a piecemeal basis. In synchronous Rust, this means that functions can "speak" the C ABI and Rust structures can be compiled with C-compatible layouts, and that we use native system functionality like the default memory allocator or the native threading APIs. In _asynchronous_ Rust, it means that we are able to integrate into other systems, like C++ futures, Grand Central Dispatch, or JavaScript promises.
