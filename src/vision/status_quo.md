# User stories: Where we are now

Meet Bob. Bob has spent a few years hacking in JavaScript and knows promises pretty well. He's new to Rust, but he wants to learn! The following sections tell stories from Bob's journey. You can read them linearly, but in order to cover the full range of experiences of people within async, they sometimes cover multiple parallel paths. For example, Bob tries out [tokio], but he also uses [bastion], [smol], and [async-std].

[tokio]: https://tokio.rs/
[bastion]: https://github.com/bastion-rs/bastion
[smol]: https://github.com/smol-rs/smol
[async-std]: https://github.com/async-rs/async-std

## Bob reads the async book, gets a bit stressed out

To learn Rust, Bob has decided to build a web application. He's heard that Rust has this nifty async-await feature and he wants to try it. He goes to the ["learning" section on rust-lang.org], where he finds a bunch of links[^nobook]. He opens up [Rust By Example] and skims [The Rust Book]. There are a lot of new concepts to wrap his head around, like modules and the borrow checker, but he can't find anything about Async I/O. It's kind of overwhelming[^cite-overwhelming]. Still, he perserveres.

Eventually, skimming on stack overflow and elsewhere, he comes across a link to the ["Asynchronous Programming in Rust" book]. In the book, he learns about Rust's async/await syntax. Using the book he [creates a "Hello, World" example] -- that's fun! He's feeling confident.

Unfortunately, the book goes straight from there to low-level details on how futures are implemented, and that's confusing. The next few sections also seem to be kind of detailed. He just wants to make some working code! He skips ahead a bit.

He gets to the [Async Ecosystem chapter] and he realizes that he has to pick a runtime. This is kind of stressful. What if he picks the wrong one? What if he starts building a project and then realizes that the runtime he chose isn't compatible with some library he wants to use?[^cite-FOMO] It's already been an hour or two, so he decides to just go to bed and stream [Bob Ross] on PBS instead. Rust will be there tomorrow.

[Rust By Example]: https://doc.rust-lang.org/rust-by-example/
[The Rust Book]: https://doc.rust-lang.org/book/
["learning" section on rust-lang.org]: https://www.rust-lang.org/learn
["Asynchronous Programming in Rust" book]: https://rust-lang.github.io/async-book/
[creates a "Hello, World" example]: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
[Async Ecosystem chapter]: https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html
[Bob Ross]: https://www.pbs.org/show/best-joy-painting/

## Bob has misleading intutions

Bob eventually picks a runtime and starts working through the tutorial[^other_sections]. He extends his first async-await program to one that invokes a few helper routines and awaits them:

```rust,edition2018
# struct X { }
# struct Y { }
# struct Z { }
# async fn first_thing() -> X { X { } }
# async fn second_thing() -> Y { Y { } }
# async fn third_thing() -> Z { Z { } }
async fn do_the_thing() -> (X, Y, Z) {
    let x = first_thing();
    let y = second_thing();
    let z = third_thing();
    (x.await, y.await, z.await)
}
```

When Bob runs the program, he is surprised to see that these three functions seem to run sequentially. That's strange, he thought that asynchronous functions were supposed to run...asynchronously? Bob is frustrated.

Reading into the book a bit more, Bob learns that Rust's asynchronous functions are *lazy*, and they don't start running until they are awaited. He reads about the various options (`FuturesUnordered`, etc) and ultimately opts to add in some calls to `spawn` from his favorite runtime:

```rust,edition2018
# struct X { }
# struct Y { }
# struct Z { }
# async fn first_thing() -> X { X { } }
# async fn second_thing() -> Y { Y { } }
# async fn third_thing() -> Z { Z { } }
# mod runtime { 
#    pub async fn spawn<T>(x: impl std::future::Future<Output = T>) -> T { x.await } 
# }
async fn do_the_thing() -> (X, Y, Z) {
    let x = runtime::spawn(first_thing());
    let y = runtime::spawn(second_thing());
    let z = runtime::spawn(third_thing());
    (x.await, y.await, z.await)
}
```

Now it's behaving more like he expected.

## Citations, action items, and other thoughts

[^nobook]: Action item: No link to the async book.

[^cite-overwhelming]: [based on this tweet](https://twitter.com/richardsabow/status/1345815115745140736)

[^cite-FOMO]: [based on this tweet](https://twitter.com/EchoRior/status/1359965313979346944)

[^other_sections]: We'll dig into those in specific in other sections, this section is more general.
