# 😱 Status quo stories: Alan thinks he needs async locks

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

One of Alan's first Rust related tasks in his job at YouBuy is writing an HTTP based service. This service is a simple internal proxy router that inspects an incoming HTTP request and picks the downstream service to call based on certain aspects of the HTTP request.

Alan decides that he'll simply use some shared state that request handlers can read from in order to decide how to proxy the request.

Alan, having read the Rust book and successfully completed the challenge in the [last chapters](https://doc.rust-lang.org/book/ch20-02-multithreaded.html), knows that shared state can be achieved in Rust with reference counting (using `std::sync::Arc`) and locks (using `std::sync::Mutex`). Alan starts by throwing his shared state (a `std::collections::HashMap<String, url::Url>`) into an `Arc<Mutex<T>>`.

Alan, smitten with how quickly he can write Rust code, ends up with some code that compiles that looks roughly like this:

```rust 
#[derive(Clone)]
struct Proxy {
   routes: Arc<Mutex<HashMap<String, String>>,
}

impl Proxy {
  async fn handle(&self, key: String, request: Request) -> crate::Result<Response> {
      let routes = self.state.lock().unwrap();
      let route = routes.get(key).unwrap_or_else(crate::error::MissingRoute)?;
      Ok(self.client.perform_request(route, request).await?)
  }
}
```

Alan is happy that his code seems to be compiling! The short but hard learning curve has been worth it. He's having fun now!

Unfortunately, Alan's happiness soon comes to end as he starts integrating his request handler into calls to `tokio::spawn` which he knows will allow him to manage multiple requests at a time. The error message is somewhat cryptic, but Alan is confident he'll be able to figure it out:

```
189 |     tokio::spawn(async {
    |     ^^^^^^^^^^^^ future created by async block is not `Send`
::: /home/alan/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.5.0/src/task/spawn.rs:129:21
    |
129 |         T: Future + Send + 'static,
    |                     ---- required by this bound in `tokio::spawn`

note: future is not `Send` as this value is used across an await
   --> src/handler.rs:787:9
      |
786   |         let routes = self.state.lock().unwrap();
      |             - has type `std::sync::MutexGuard<'_, HashMap<String, Url>>` which is not `Send`
787   |         Ok(self.client.perform_request(route, request).await?)
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here, with `routes` maybe used later
788   |     })
      |     - `routes` is later dropped here
```

Alan stops and takes a deep breath. He tries his best to make sense of the error message. He sort of understands the issue the compiler is telling him. Apparently `routes` is not marked as `Send`, and because it is still alive over a call to `await`, it is making the future his handler returns not `Send`. And tokio's `spawn` function seems to require that the future it received be `Send`. 

Alan reaches the boundaries of his knowledge of Rust, so he reaches out over chat to ask his co-worker Barbara for help. Not wanting to bother her, Alan provides the context he's already figured out for himself.

Barbara knows that mutex guards are not `Send` because sending mutex guards to different threads is not a good idea. She suggests looking into async locks which can be held across await points because they are `Send`. Alan looks into the tokio documentation for more info and is easily able to move the use of the standard library's mutex to tokio's mutex. It compiles!

Alan ships his code and it gets a lot of usage. After a while, Alan notices some potential performance issues. It seems his proxy handler does not have the throughput he would expect. Barbara, having newly joined his team, sits down with him to take a look at potential issues. Barbara is immediately worried by the fact that the lock is being held much longer than it needs to be. The lock only needs to be held while accessing the route and not during the entire duration of the downstream request.

She suggests to Alan to switch to not holding the lock across the I/O operations. Alan first tries to do this by explicitly cloning the url and dropping the lock before the proxy request is made:

```rust
impl Proxy {
  async fn handle(&self, key: String, request: Request) -> crate::Result<Response> {
      let routes = self.state.lock().unwrap();
      let route = routes.get(key).unwrap_or_else(crate::error::MissingRoute)?.clone();
      drop(routes);
      Ok(self.client.perform_request(route, request).await?)
  }
}
```

This compiles fine and works in testing! After shipping to production, they notice a large increase in throughput. It seems their change made a big difference. Alan is really excited about Rust, and wants to write more!

Alan continues his journey of learning even more about async Rust. After some enlightening talks at the latest RustConf, he decides to revisit the code that he and Barbara wrote together. He asks himself, is using an *async* lock the right thing to do? This lock should only be held for a very short amount of time. Yielding to the runtime is likely more expensive than just synchronously locking. But he remembers vaguely hearing that you should never use blocking code in async code as this will block the entire async executor from being able to make progress, so he doubts his intuition.

After chatting with Barbara, who encourages him to benchmark and measure, he decides to switch back to synchronous locks. 

Unfortunately, switching back to synchronous locks brings back the old compiler error message about his future not being `Send`. Alan is confused as he's dropping the mutex guard before it ever crosses an await point.

Confused Alan goes to Barbara for advice. She is also confused, and it takes several minutes of exploration before she comes to a solution that works: wrapping the mutex access in a block and implicitly dropping the mutex.

```rust
impl Proxy {
  async fn handle(&self, key: String, request: Request) -> crate::Result<Response> {
      let route = {
        let routes = self.state.lock().unwrap();
        routes.get(key).unwrap_or_else(crate::error::MissingRoute)?.clone()
      };
      Ok(self.client.perform_request(route, request).await?)
  }
}
```

Barbara mentions she's unsure why explicitly dropping the mutex guard did not work, but they're both happy that the code compiles. In fact it seems to have improved the performance of the service when its under extreme load. Alan's intuition was right!

In the end, Barbara decides to write a blog post about how blocking in async code isn't always such a bad idea. 

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
    * Locks can be quite common in async code as many tasks might need to mutate some shared state.
    * Error messages can be fairly good, but they still require a decent understanding of Rust (e.g., `Send`, `MutexGuard`, drop semantics) to fully understand what's going on.
    * This can lead to needing to use certain patterns (like dropping mutex guards early) in order to get code working.
    * The advice to never block in async code is not always true: if blocking is short enough, is it even blocking at all?
### **What are the sources for this story?**
    * Chats with [Alice](https://github.com/Darksonn) and [Lucio](https://github.com/LucioFranco).
    * Alice's [blog post](https://ryhl.io/blog/async-what-is-blocking/) on the subject has some good insights.
    * The issue of conservative analysis of whether values are used across await points causing futures to be `!Send` is [known](https://rust-lang.github.io/async-book/07_workarounds/03_send_approximation.html), but it takes some digging to find out about this issue. A tracking issue for this can be [found here](https://github.com/rust-lang/rust/issues/57478).
### **Why did you choose [Alan](../characters/alan.md) to tell this story?**
    * While Barbara might be tripped up on some of the subtlties, an experienced Rust developer can usually tell how to avoid some of the issues of using locks in async code. Alan on the other hand, might be surprised when his code does not compile as the issue the `Send` error is protecting against (i.e., a mutex guard being moved to another thread) is not protected against in other languages.
### **How would this story have played out differently for the other characters?**
    * Grace would have likely had a similar time to Alan. These problems are not necessarily issues you would run into in other languages in the same way.
    * Niklaus may have been completely lost. This stuff requires a decent understanding of Rust and of async computational systems.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
