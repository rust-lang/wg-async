# ✨ Shiny future stories: Grace writes a recursive function with auto-box

## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

### Early days

Grace is using async Rust for the first time. She is experimenting with async functions and she begins with her favorite "Hello, World" sort of application, Fibonacci:

```rust
async fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

It doesn't make much sense, but that's ok. She compiles it as part of a webapp, so that when she requests `https://localhost/fib/22` she gets back a page with the correct result. Fun!

### Discovering automatic boxing

Now that Grace has the basics working, she starts adding code to the [MonsterMesh] project that she is developing at work. She is writing a simple recursive function, similar to `fibonacci`, but she finds that she gets an error:

```
error: `Box` required for async function
 --> src/lib.rs:4:9
  |
4 |     async fn fibonacci(x: usize) -> usize {
  |              ^^^^^^^^^ the function `fibonacci` is recursive and requires a `Box`
  |
note: the lint level is defined here
 --> src/lib.rs:1:9
  |
1 | #![deny(automatic_box)]
  |         ^^^^^^^^^^^^^
help: to request a box explicitly, write `box async fn`
  |
4 |     box async fn fibonacci(x: usize) -> usize {
```

"What's this?" she thinks. "It worked before?" She clicks in her IDE on the "more info" page for the `automatic_box` lint, where she learns that recursive asynchronous functions require some form of extra heap allocation to work. The compiler normally inserts these automatically, but there is an allow-by-default lint that can be used to turn those into warnings or hard errors.

As the message suggests, users can explicit request a box, but for Grace's project that won't work: [MonsterMesh] is very careful about allocation. She decides to write `fibonacci` into an iterative form and avoid the problem.

## 🤔 Frequently Asked Questions

### **What status quo story or stories are you retelling?**
*Link to the status quo stories here. If there isn't a story that you're retelling, [write it](../how_to_vision/status_quo.md)!*

### **What is [Alan] most excited about in this future? Is he disappointed by anything?**
*Think about Alan's top priority (performance) and the expectations he brings (ease of use, tooling, etc). How do they fare in this future?*

### **What is [Grace] most excited about in this future? Is she disappointed by anything?**
*Think about Grace's top priority (memory safety) and the expectations she brings (still able to use all the tricks she knows and loves). How do they fare in this future?*

### **What is [Niklaus] most excited about in this future? Is he disappointed by anything?**
*Think about Niklaus's top priority (accessibility) and the expectations he brings (strong community that will support him). How do they fare in this future?*

### **What is [Barbara] most excited about in this future? Is she disappointed by anything?**
*Think about Barbara's top priority (productivity, maintenance over time) and the expectations she brings (fits well with Rust). How do they fare in this future?*

### **If this is an alternative to another shiny future, which one, and what motivated you to write an alternative?** (Optional)
* *Cite the other story. Be specific, but focus on what you like about your version, not what you dislike about the other.*
* *If this is not an alternative, you can skip this one.*

### **What [projects] benefit the most from this future?**

### **Are there any [projects] that are hindered by this future?**

### **What are the incremental steps towards realizing this shiny future?** (Optional)
* *Talk about the actual work we will do. You can link to [design docs](../design_docs.md) or even add new ones, as appropriate.*
* *You don't have to have the whole path figured out yet!*

### **Does realizing this future require cooperation between many projects?** (Optional)
*For example, if you are describing an interface in libstd that runtimes will have to implement, talk about that.*

### How does this work?

Rough idea: we give each async function a nominal type. Instead of desugaring to

```rust
fn foo() -> impl Future<Output = T> {
    async move { }
}
```

we desugar to something like this

```rust
struct Foo {
    fut: impl Future<Output = T>
}

impl Future for Foo { ... }

fn foo() -> Foo {
    Foo { fut: async move { ... } }
}
```

this ensures the type has a finite representation even when its recursive. Then, for recursive functions, we have to introduce a `Pin<Box<T>>`, which ensure it has finite layout. Then we lint.

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
