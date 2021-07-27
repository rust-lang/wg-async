# ✨ Shiny future stories: Alan's trust in the compiler is rewarded

[How To Vision: Shiny Future]: ../how_to_vision/shiny_future.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async-foundations/master/src/vision/shiny_future/template.md
[`shiny_future`]: https://github.com/rust-lang/wg-async-foundations/tree/master/src/vision/shiny_future
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async-foundations/blob/master/src/SUMMARY.md

## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

### Trust the compiler
Alan has a lot of experience in C#, but in the meantime has created some successful projects in Rust.
He has dealt with his fair share of race conditions/thread safety issues during runtime in C#, but is now starting to trust that if his Rust code compiles,
he won't have those annoying runtime problems to deal with.

This allows him to try to squeeze his programs for as much performance as he wants, because the compiler will stop him when he tries things that could result in runtime problems.
After seeing the performance and the lack of runtime problems, he starts to trust the compiler more and more with each project finished.

He knows what he can do with external libraries, he does not need to fear concurrency issues if the library cannot be used from multiple threads, because the compiler would tell him.

His trust in the compiler solidifies further the more he codes in Rust.

### The first async project

Alan now starts with his first async project. He opens up the Rust book to the "Async I/O" chapter and it guides him to writing his first program. He starts by writing some synchronous code to write to the file system:

```rust
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("a.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
```

Next, he adapts that to run in an async fashion. He starts by converting `main` into `async fn main`:

```rust
use std::fs::File;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("a.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
```

The code compiles, but he gets a warning:

```
warning: using a blocking API within an async function
 --> src/main.rs:4:25
1 | use std::fs::File;
  |     ------------- try changing to `std::async_io::fs::File`
  | ...
4 |     let mut file: u32 = File::create("a.txt")?;
  |                         ^^^^^^^^^^^^ blocking functions should not be used in async fn
help: try importing the async version of this type
 --> src/main.rs:1
1 | use std::async_fs::File;
```

"Oh, right," he says, "I am supposed to use the async variants of the APIs." He applies the suggested fix in his IDE, and now his code looks like:

```rust
use std::async_fs::File;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("a.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
```

His IDE recompiles instantaneously and he now sees two little squiggles, one under each `?`. Clicking on the errors, he sees:

```
error: missing await
 --> src/main.rs:4:25
4 |     let mut file: u32 = File::create("a.txt")?;
  |                                              ^ returns a future, which requires an await
help: try adding an await
 --> src/main.rs:1
4 |     let mut file: u32 = File::create("a.txt").await?;
```

He again applies the suggested fix, and his code now shows:

```rust
use std::async_fs::File;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("a.txt").await?;
    file.write_all(b"Hello, world!").await?;
    Ok(())
}
```

Happily, it compiles, and when he runs it, everything works as expected. "Cool," he thinks, "this async stuff is pretty easy!"

### Making some web requests

Next, Alan decides to experiment with some simple web requests. This isn't part of the standard library, but the `fetch_rs` package is listed in the Rust book. He runs `cargo add fetch_rs` to add it to his `Cargo.toml` and then writes:

```rust
use std::async_fs::File;
use fetch_rs;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("a.txt")?;
    file.write_all(b"Hello, world!")?;

    let body = fetch_rs::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("{}", body);

    Ok(())
}
```

This feels pretty easy!

## 🤔 Frequently Asked Questions

### What status quo story or stories are you retelling?

* [Alan started trusting the Rust compiler, but then async](../status_quo/alan_started_trusting_the_rust_compiler_but_then_async.md)
* [Barbara makes their first foray into async](../status_quo/barbara_makes_their_first_steps_into_async.md)

### What are the key points you were trying to convey with this status quo story?

* Getting started with async should be as automated as possible:
    * change `main` to an `async fn`;
    * use the APIs found in modules like `std::async_foo`, which should map as closely as possible to their non-async equivalents.
* You should get some sort of default runtime that is decent
* Lints should guide you in using async:
    * identifying blocking functions
    * identifying missing `await`
* You should be able to grab libraries from the ecosystem and they should integrate with the default runtime without fuss

### Is there a "one size fits all" runtime in this future?

This particular story doesn't talk about what happens when the default runtime isn't suitable. But you may want to read its sequel, ["Alan Switches Runtimes"](./alan_switches_runtimes.md).

### **What is [Alan] most excited about in this future? Is he disappointed by anything?**

Alan is excited about how easy it is to get async programs up and running. He also finds the performance is good. He's good.

### **What is [Grace] most excited about in this future? Is she disappointed by anything?**

Grace is happy because she is getting strong safety guarantees and isn't getting surprising runtime panics when composing libraries. The question of whether she's able to use the tricks she knows and loves is a good one, though. The default scheduler may not optimize for maximum performance -- this is something to explore in future stories. The ["Alan Switches Runtimes"](./alan_switches_runtimes.md), for example, talks more about the ability to change runtimes.

### **What is [Niklaus] most excited about in this future? Is he disappointed by anything?**

Niklaus is quite happy. Async Rust is fairly familiar and usable for him. Further, the standard library includes "just enough" infrastructure to enable a vibrant crates-io ecosystem without centralizing everything.

### **What is [Barbara] most excited about in this future? Is she disappointed by anything?**

Barbara quite likes that the std APIs for sync and sync fit together, and that there is a consistent naming scheme across them. She likes that there is a flourishing ecosystem of async crates that she can choose from.

### **What [projects] benefit the most from this future?**

A number of projects benefit:

* Projects like [YouBuy] are able to get up and going faster.
* Libraries like [SLOW] become easier because they can target the std APIs and there is a defined plan for porting across runtimes.

[YouBuy]: ../projects/YouBuy.md
[SLOW]: ../projects/SLOW.md

### **Are there any [projects] that are hindered by this future?**

It depends on the details of how we integrate other runtimes. If we wound up with a future where most libraries are "hard-coded" to a single default runtime, this could very well hinder any number of projects, but nobody wants that.

### **What are the incremental steps towards realizing this shiny future?**

This question can't really be answered in isolation, because so much depends on the story for how we integrate with other runtimes. I don't think we can accept a future where is literally a single runtime that everyone has to use, but I wanted to pull out the question of "non-default runtimes" (as well as more details about the default) to other stories.

### **Does realizing this future require cooperation between many projects?**

Yes. For external libraries like `fetch_rs` to interoperate they will want to use the std APIs (and probably traits).

[character]: ../characters.md
[comment]: ./comment.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[projects]: ../projects.md
[htvsq]: ../how_to_vision/shiny_future.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
