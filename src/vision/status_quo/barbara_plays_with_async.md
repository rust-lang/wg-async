# 😱 Status quo stories: Barbara plays with async


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[Barbara] has been following async rust for a long time, in eager anticipation
of writing some project using async.  The last time she tried to do anything
with futures in rust was more than a year ago (before async functions), and
when you had to chain futures together with many calls to `then`
(often leading to inscrutable error messages hundreds of characters long).
This was not a pleasant experience for Barbara.

After watching the development of rust async/await (by following
discussions on [/r/rust][reddit] and the [internals] forums), she wants
to start to play around with writing async code.  Before starting on any real
project, she starts with a "playground" where she can try to write some simple
async rust code to see how it feels and how it compares to how async code feels
in other languages she knows (like C# and JavaScript).

She starts by opening a blank project in VSCode with [rust-analyzer].  Because she's
been following the overall state of rust async, she knows that she needs a runtime,
and quickly decides to use tokio, because she knows its quite popular and well documented.

After looking the long length of the [tokio tutorial], she decides to not read
most of it right now, and tries to dive right in to writing code.  But she does
look at the "[Hello Tokio]" section that shows what feature flags are required by tokio:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Poking around the tokio API docs in search of something to play with, she sees
a simple future that looks interesting: the [`sleep`][tokio sleep] future
that will wait for a certain duration to elapse before resolving.

Borrowing again from the "Hello Tokio" tutorial to make sure she has the correct
spelling for the tokio macros, she writes up the following code:

```rust,ignore
#[tokio::main]
pub async fn main() {
    let mut rng = thread_rng();
    let t = Uniform::new(100, 5000);

    let mut futures = Vec::new();
    for _ in 0..10 {
        let delay = rng.sample(t);
        futures.push(tokio::time::sleep(Duration::from_millis(delay)));
    }
    println!("Created 10 futures");

    for f in futures {
        f.await;
    }

    println!("Done waiting for all futures");
}
```

This very first version she wrote compiled on the first try and had no errors
when running it.  Barbara was pleased about this.

However, this example is pretty boring.  The program just sits there for a few
seconds doing nothing, and giving no hints about what it's actually doing.
So for the next iteration, Barbara wants to have a message printed out
when each future is resolved.  She tries this code at first:

```rust,ignore
let mut futures = Vec::new();
for _ in 0..10 {
    let delay = rng.sample(t);
    futures.push(tokio::time::sleep(Duration::from_millis(delay)).then(|_| {
        println!("Done!");
    }));
}
println!("Created 10 futures");
```

But the compiler gives this error:

```ignore
error[E0277]: `()` is not a future
  --> src\main.rs:13:71
   |
13 |         futures.push(tokio::time::sleep(Duration::from_millis(delay)).then(|_| {
   |                                                                       ^^^^ `()` is not a future
   |
   = help: the trait `futures::Future` is not implemented for `()`
```

Even though the error is pointing at the `then` function, Barbara pretty quickly
recognizes the problem -- her closure needs to return a future, but `()` is not
a future (though she wonders "why not?").  Looking at the tokio docs is not very
helpful.  The `Future` trait isn't even defined in the tokio docs, so
she looks at the docs for the Future trait in the rust standard library docs
and she sees it only has 5 implementors; one of them is called `Ready`
which looks interesting.
Indeed, this struct is a future that will resolve instantly, which is what
she wants:

```rust,ignore
for _ in 0..10 {
    let delay = rng.sample(t);
    futures.push(tokio::time::sleep(Duration::from_millis(delay)).then(|_| {
        println!("Done!");
        std::future::ready(())
    }));
}
```

This compiles without error, but when Barbara goes to run the code, the output
surprises her a little bit:  After waiting running the program, nothing happened
for about 4 seconds.  Then the first "Done!" message was printed, followed very
quickly by the other 9 messages.   Based on the code she wrote, she expected 10
"Done!" messages to be printed to the console over the span of about 5 seconds,
with roughly a uniform distribution.

After running the program few more times, she always observes that while the
first view messages are printed after some delay, the last few messages are
always printed all at once.

Barbara has experience writing async code in JavaScript, and so she thinks for
a moment about how this toy code might have looked like if she was using JS:

```javascript,ignore
async function main() {
    const futures = [];
    for (let idx = 0; idx < 10; idx++) {
        const delay = 100 + (Math.random() * 4900);
        const f = new Promise(() => {
            setTimeout(() => console.log("Done!"), delay)
        })
        futures.push(f);
    }

    Promise.all(futures);
}
```

After imagining this code, Barbara has an "ah-ha!" moment, and realizes the
problem is likely how she is waiting for the futures in her rust code.
In her rust code, she is waiting for the futures one-by-one, but in the
JavaScript code she is waiting for all of them simultaneously.

So Barbara looks for a way to wait for a Vec of futures.  After a bunch of
searching in the tokio docs, she finds nothing.  The closet thing she finds
is a `join!` macro, but this appears to only work on individually specified
futures, not a Vec of futures.

Disappointed, she then looks at the [future module] from the rust standard
library, but module is tiny and very
clearly doesn't have what she wants.  Then Barbara has another "ah-ha!" moment
and remembers that there's a 3rd-party crate called "[futures][futures crate]"
on crates.io that she's seen mentioned in some /r/rust posts.  She checks the
docs and finds the `join_all` function which looks like what she wants:

```rust,ignore
let mut futures = Vec::new();
for _ in 0..10 {
    let delay = rng.sample(t);
    futures.push(tokio::time::sleep(Duration::from_millis(delay)).then(|_| {
        println!("Done!");
        std::future::ready(())
    }));
}
println!("Created 10 futures");

futures::future::join_all(futures).await;
println!("Done");
```
It works exactly as expected now!  After having written the code, Barbara begins
to remember an important detail about rust futures that she once read somewhere:
rust futures are lazy, and won't make progress unless you await them.

Happy with this success, Barbara continues to expand her toy program by making
a few small adjustments:

```rust,ignore
for counter in 0..10 {
    let delay = rng.sample(t);
    let delay_future = tokio::time::sleep(Duration::from_millis(delay));

    if counter < 9 {
        futures.push(delay_future.then(|_| {
            println!("Done!");
            std::future::ready(())
        }));
    } else {
        futures.push(delay_future.then(|_| {
            println!("Done with the last future!");
            std::future::ready(())
        }));
    }
}
```

This fails to compile:

```ignore
error[E0308]: mismatched types

   = note: expected closure `[closure@src\main.rs:16:44: 19:14]`
              found closure `[closure@src\main.rs:21:44: 24:14]`
   = note: no two closures, even if identical, have the same type
   = help: consider boxing your closure and/or using it as a trait object
```

This error doesn't actually surprise Barbara that much, as she is familiar with
the idea of having to box objects sometimes.  She does
notice the "consider boxing your closure" error, but thinks that this is not
likely the correct solution.  Instead, she thinks that she should box the
entire future.

She first adds explicit type annotations to the Vec:

```rust,ignore
let mut futures: Vec<Box<dyn Future<Output=()>>> = Vec::new();
```

She then notices that her IDE (VSCode + rust-analyzer) has a new error on
each call to push.  The code assist on each error says `Store this in the heap
by calling 'Box::new'`.  She is exactly what she wants, and it happy that
rust-analyzer perfectly handled this case.

Now each future is boxed up, but there is one final error still,
this time on the call to `join_all(futures).await`:

```ignore
error[E0277]: `dyn futures::Future<Output = ()>` cannot be unpinned
  --> src\main.rs:34:31
   |
34 |     futures::future::join_all(futures).await;
```

Barbara has been around rust for long enough to know that there is a `Box::pin`
API, but she doesn't really understand what it does, nor does she have a good
intuition about what this API is for.  But she is accustomed to just trying
things in rust to see if they work.  And indeed, after changing `Box::new` to
`Box::pin`:

```rust,ignore
futures.push(Box::pin(delay_future.then(|_| {
    println!("Done!");
    std::future::ready(())
})));
```

and adjusting the type of the Vec:

```rust,ignore
let mut futures: Vec<Pin<Box<dyn Future<Output=()>>>> = Vec::new();
```

the code compiles and runs successfully.

But even though the run is working correctly, she wishes she had a better idea
why pinning is necessary here and feels a little uneasy having to use something
she doesn't yet understand well.

As one final task, Barbara wants to try to replace the chained call to `then`
with a async block.  She remembers that these were a big deal in a recent
release of rust, and that they looked a lot nicer than a long chain of `then`
calls.  She doesn't remember the exact syntax for this, but she read a blog
post about async rust a few weeks ago, and has a vague idea of how it looks.

She tries writing this:

```rust,ignore
futures.push(Box::pin(async || {
    tokio::time::sleep(Duration::from_millis(delay)).await;
    println!("Done after {}ms", delay);
}));
```

The compiler gives an error:

```ignore
error[E0658]: async closures are unstable
  --> src\main.rs:14:31
   |
14 |         futures.push(Box::pin(async || {
   |                               ^^^^^
   |
   = note: see issue #62290 <https://github.com/rust-lang/rust/issues/62290> for more information
   = help: add `#![feature(async_closure)]` to the crate attributes to enable
   = help: to use an async block, remove the `||`: `async {`
```

Barbara knows that async is stable and using this nightly feature isn't what
she wants.  So the tries the suggestion made by the compiler and removes the `||` bars:

```rust,ignore
futures.push(Box::pin(async {
    tokio::time::sleep(Duration::from_millis(delay)).await;
    println!("Done after {}ms", delay);
}));
```

A new error this time:

```ignore
error[E0597]: `delay` does not live long enough
15 | |             tokio::time::sleep(Duration::from_millis(delay)).await;
   | |                                                      ^^^^^ borrowed value does not live long enough
```

This is an error that Barbara is very familiar with.  If she was working with
a closure, she knows she can use a move-closure (since her `delay` type is `Copy`).
But she not using a closure (she just tried, but the compiler told her to switch
to an async block), but Barbara's experience with rust tells her that it's a very
consistent language.  Maybe the same keyword used in move closures will work here?
She tries it:

```rust,ignore
futures.push(Box::pin(async move {
    tokio::time::sleep(Duration::from_millis(delay)).await;
    println!("Done after {}ms", delay);
}));
```

It works!  Satisfied but still thinking about async rust, Barbara takes a break
to eat a cookie.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **Why did you choose Barbara to tell this story?**
[Barbara] has years of rust experience that she brings to bear in her async learning experiences.

### **What are the morals of the story?**
    
* Due to Barbara's long experience with rust, she knows most of the language
  pretty well (except for things like async, and advanced concepts like pinned objects).
  She generally [trusts the rust compiler], and she's learned over the years that she
  can learn how to use an unfamiliar library by reading the API docs.  As long
  as she can get the types to line up and the code to compile, things generally
  work as she expects.

  But this is not the case with rust async:
   
   * There can be new syntax to learn (e.g. async blocks)
   * It can be hard to find basic functionality (like `futures::future::join_all`)
   * It's not always clear how the ecosystem all fits together
     (what functionality is part of tokio?  What is part of the
     standard library?  What is part of other crates like the
     `futures` crate?)
   * Sometimes it looks like there multiple ways to do something:
     * What's the difference between `futures::future::Future` and `std::future::Future`?
     * What's the difference between `tokio::time::Instant` and `std::time::Instant`?
     * What's the difference between `std::future::ready` and ` futures::future::ok`?


* Barbara's has a lot to learn.  Her usual methods of learning how to use
  new crates doesn't really work when learning tokio and async.  She wonders
  if she actually should have read the long tokio tutorial before starting.
  She realizes it will take her a while to build up the necessary foundation
  of knowledge before she can be proficient in async rust.
* There were several times where the compiler or the IDE gave helpful error
  messages and Barbara appreciated these a lot.
      
### **What are the sources for this story?**
Personal experiences of the author
  
### **How would this story have played out differently for the other characters?**
Other characters would likely have written all the same code as Barbara,
and probably would have run into the same problems.  But other characters
might have needed quite a bit longer to get to the solution.  

For example, it was Barbara's experience with move-closures that led her to try 
adding the `move` keyword to the async block.  And it was her general
"ambient knowledge" of things that allowed her to remember that things
like the `futures` crate exist.  Other characters would have likely needed
to resort to an internet search or asking on a rust community.

### What are other related stories?
* [Barbara makes their first steps in async] is Barbara in a slightly different universe.
* [Alan started trusting the rust compiler][trusts the rust compiler] is a similar story about a different character.

[status quo stories]: ./status_quo.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
[trusts the rust compiler]: ./alan_started_trusting_the_rust_compiler_but_then_async.md
[Barbara makes their first steps in async]: ./barbara_makes_their_first_steps_into_async.md
[reddit]: https://www.reddit.com/r/rust
[internals]: https://internals.rust-lang.org/
[rust-analyzer]: https://rust-analyzer.github.io/
[tokio tutorial]: https://tokio.rs/tokio/tutorial
[Hello Tokio]: https://tokio.rs/tokio/tutorial/hello-tokio
[tokio sleep]: https://docs.rs/tokio/1.4.0/tokio/time/fn.sleep.html
[future module]: https://doc.rust-lang.org/nightly/std/future/index.html
[futures crate]: https://crates.io/crates/futures
