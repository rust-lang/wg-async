# 😱 Status quo stories: Alan started trusting the Rust compiler, but then... async


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

## The story
### Trust the compiler
Alan has a lot of experience in C#, but in the meantime has created some successful projects in Rust.
He has dealt with his fair share of race conditions/thread safety issues during runtime in C#, but is now starting to trust that if his Rust code compiles,
he won't have those annoying runtime problems to deal with.

This allows him to try to squeeze his programs for as much performance as he wants, because the compiler will stop him when he tries things that could result in runtime problems.
After seeing the perfomance and the lack of runtime problems, he starts to trust the compiler more and more with each project finished.

He knows what he can do with external libraries, he does not need to fear concurrency issues if the library cannot be used from multiple threads, because the compiler would tell him.

His trust in the compiler solidifies further the more he codes in Rust.

### The first async project
Alan now starts with his first async project. He sees that there is no async in the standard library, but after googling for "rust async file open", he finds 'async_std', a crate that provides some async versions of the standard library functions.
He has some code written that asynchrously interacts with some files:
```rust,ignore
use async_std::fs::File;
use async_std::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("a.txt").await?;
    file.write_all(b"Hello, world!").await?;
    Ok(())
}
```
But now the compiler complains that `await` is only allowed in `async` functions. He now notices that all the examples use `#[async_std::main]` 
as an attribute on the `main` function in order to be able to turn it into an `async main`, so he does the same to get his code compiling:
```rust,ignore
use async_std::fs::File;
use async_std::prelude::*;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("a.txt").await?;
    file.write_all(b"Hello, world!").await?;

    Ok(())
}
```

This aligns with what he knows from C#, where you also change the entry point of the program to be async, in order to use `await`.
Everything is great now, the compiler is happy, so no runtime problems, so Alan is happy.

The project is working like a charm.

### Fractured futures, fractured trust
The project Alan is building is starting to grow, and he decides to add a new feature that needs to make some API calls. He starts using `reqwest` in order to help him achieve this task.
After a lot of refactoring to make the compiler accept the program again, Alan is satisfied that his refactoring is done.
His program now boils down to:
```rust,ignore
use async_std::fs::File;
use async_std::prelude::*;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("a.txt").await?;
    file.write_all(b"Hello, world!").await?;

    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("{}", body);

    Ok(())
}
```

He runs his project but is suddenly greeted with a runtime error. He is quite surprised. "How is this even possible?", he thinks. "I don't have any out-of-bounds accesses, and I never use `.unwrap` or `.expect`."
At the top of the error message he sees: `thread 'main' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime'` 

He searches what "Tokio" is in Rust, and he finds that it also provides an attribute to put on `main`, namely `[tokio::main]`, but what is the difference with `[async_std::main]`? His curiosity leads him to watch videos/read blogs/scour reddit,... on why there are multiple runtimes in Rust. This leads him into a rabbit hole and now he learns about Executors, Wakers, `Pin`,... He has a basic grasp of what they are, but does not have a good understanding of them or how they all fit together exactly. These are all things he had not need to know nor heed in C#. (Note: there is another story about troubles/confusion that might arise when learning all these things about async: [Alan hates writing a `Stream`](./alan_hates_writing_a_stream.md))

He does understand the current problems and why there is no one-size-fits-all executor (yet). Trying to get his async Rust code to work, he broadened his knowledge about what async code actually is, he gains another way to reason about asynchronous code, not only in Rust, but also more generally.

But now he realizes that there is a whole new area of runtime problems that he did not have to deal with in C#, but he does in Rust.
Can he even trust the Rust compiler anymore? What other kinds of runtime problems can occur in Rust that can't in C#?
If his projects keep increasing in complexity, will other new kinds of runtime problems keep popping up? Maybe it's better to stick with C#, since Alan 
already knows all the runtime problems you can have over there.

### The Spider-Man effect
Do you recall in Spider-Man, that after getting bitten by the radioactive spider, Peter first gets ill before he gains his powers? Well, imagine instead of being bitten by a radioactive spider, he was bitten by an async-rust spider...

In his work, Alan sees an async call to a C# wrapper around SQLite, his equivalent of a spider-sense (async-sense?) starts tingling. Now knowing from Rust the complexities that arise when trying to create asynchronicity, what kind of complex mechanisms are at play here to enable these async calls from C# that end up in the C/C++ of SQLite?

He quickly discovers that there are no complex mechanism at all! It's actually just a synchronous call all the way down, with just some exta overhead from wrapping it into an asynchronous function. There are no points where the async function will yield. He transforms all these asynchronous calls to their synchronous counterparts, and sees a slight improvement in performance. Alan is happy, product management is happy, customers are happy!


Over the next few months, he often takes a few seconds to reflect about why certain parts of the code are async, if they should be, or how other parts of the code might benefit from being async and if it's possible to make them async. He also uses what he learned from async Rust in his C# code reviews to find similar problems or general issues (With great power...). He even spots some lifetime bugs w.r.t. asynchronous code in C#, imagine that.

His team recognizes that Alan has a pretty good grasp about what async is really about, and he is unofficially crowned the "async guru" of the team.


Even though this spider-man might have gotten "ill" (his negative experience with async Rust), he has now become the superhero he was meant to be!


## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* Async I/O includes a new set of runtime errors and misbehaviors that the compiler can't help you find. These include cases like executing blocking operations
  in an async context but also mixing runtime libraries (something users may not even realize is a factor).
* Rust users get used to the compiler giving them error messages for runtime problems but also helping them to fix them. Pushing error messages to runtimes
  feels surprising and erodes some of their confidence in Rust.
* The "cliff" in learning about async is very steep -- at first everything seems simple and similar to other languages, then suddenly you are thrown into a lot of information. It's hard to know what's important and what is not. **But**, at the same time, dipping your toes into async Rust can broaden the understanding a programmer has of asynchronous coding, which can help them even in other languages than Rust.

### **What are the sources for this story?**
Personal experience of the author.

### **Why did you choose Alan to tell this story?**
With his experience in C#, Alan probably has experience with async code. Even though C# protects him from certain classes of errors,
he can still encounter other classes of errors, which the Rust compiler prevents.

### **How would this story have played out differently for the other characters?**
For everyone except Barbara, I think these would play out pretty similarly, as this is a kind of problem unique to Rust. Since Barbara has a lot of Rust experience,
  she would probably already be familiar with this aspect.

### **How would this story have played out differently if Alan came from another GC'd language?**
It would be very close, since all other languages (that I know of) provide async runtimes out of the box and it's not something the programmer needs to concern themselves with.
