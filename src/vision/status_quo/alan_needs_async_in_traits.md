# 😱 Status quo stories: Alan needs async in traits

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Alan is working on a project with Barbara which has already gotten off to a [somewhat rocky start](./barbara_anguishes_over_http.md). He is working on abstracting away the HTTP implementation the library uses so that users can provide their own. He wants the user to implement an async trait called `HttpClient` which has one method `perform(request: Request) -> Response`. Alan tries to create the async trait:

```rust
trait HttpClient {
    async fn perform(request: Request) -> Response;
}
```

When Alan tries to compile this, he gets an error:

```
 --> src/lib.rs:2:5
  |
2 |     async fn perform(request: Request) -> Response;
  |     -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     `async` because of this
  |
  = note: `async` trait functions are not currently supported
  = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
```

Alan, who has been using Rust for a little while now, has learned to follow compiler error messages and adds `async-trait` to his `Cargo.toml`. Alan follows the README of `async-trait` and comes up with the following code:

```rust
#[async_trait]
trait HttpClient {
    async fn perform(request: Request) -> Response;
}
```

Alan's code now compiles, but he also finds that his compile times have gone from under a second to around 6s, at least for a clean build.

After Alan finishes adding the new trait, he shows his work off to Barbara and mentions he's happy with the work but is a little sad that compile times have worsened. Barbara, an experienced Rust developer, knows that using `async-trait` comes with some additional issues. In this particular case she is especially worried about tying their public API to a third-party dependency. Even though it is technically possible to implement traits annotated with `async_trait` without using `async_trait`, doing so in practice is very painful. For example `async_trait`:

* handles lifetimes for you if the returned future is tied to the lifetime of some inputs.
* boxes and pins the futures for you.

which the implementer will have to manually handle if they don't use `async_trait`. She decides to not worry Alan with this right now. Alan and Barbara are pretty happy with the results and go on to publish their crate which gets lots of users.

Later on, a potential user of the library wants to use their library in a `no_std` context where they will be providing a custom HTTP stack. Alan and Barbara have done a pretty good job of limiting the use of standard library features and think it might be possible to support this use case. However, they quickly run into a show stopper: `async-trait` boxes all of the futures returned from a async trait function. They report this to Alan through an issue.

Alan, feeling (over-) confident in his Rust skills, decides to try to see if he can implement async traits without using `async-trait`. 

```rust 
trait HttpClient {
   type Response: Future<Output = Response>;

   fn perform(request: Request) -> Self::Response; 
}
```

Alan seems to have something working, but when he goes to update the examples of how to implement this trait in his crate's documentation, he realizes that he either needs to:

* use trait object:

  ```rust
  struct ClientImpl;

  impl HttpClient for ClientImpl {
      type Response = Pin<Box<dyn Future<Output = Response>>>;

      fn perform(request: Request) -> Self::Response {
          Box::pin(async move {
              // Some async work here creating Reponse
          })
	  }
  }
  ```

  which wouldn't work for `no_std`.

* implement `Future` trait manually, which isn't particulary easy/straight-forward for non-trivial cases, especially if it involves making other async calls (likely).

After a lot of thinking and discussion, Alan and Barbara accept that they won't be able to support `no_std` users of their library and add mention of this in crate documentation.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

* `async-trait` is awesome, but has some drawbacks
    * compile time increases
    * performance cost of boxing and dynamic dispatch
    * not a standard solution so when this comes to language, it might break things
* Trying to have a more efficient implementation than `async-trait` is likely not possible. 

### **What are the sources for this story?**

* [Zeeshan](https://github.com/zeenix/) is looking for a way to implement async version of the [service-side zbus API](https://docs.rs/zbus/1.9.1/zbus/trait.Interface.html).
* [Ryan](https://github.com/rylev) had to use `async-trait` in an internal project.

### **Why did you choose Alan to tell this story?**

We could have used Barbara here but she'd probably know some of the work-arounds (likely even the details on why they're needed) and wouldn't need help so it wouldn't make for a good story. Having said that, Barbara is involved in the story still so it's not a pure Alan story.

### **How would this story have played out differently for the other characters?**

* Barbara: See above.
* Grace: Probably won't know the solution to these issues much like Alan, but might have an easier time understanding the **why** of the whole situation. 
* Niklaus: would be lost - traits are somewhat new themselves. This is just more complexity, and Niklaus might not even know where to go for help (outside of compiler errors).
