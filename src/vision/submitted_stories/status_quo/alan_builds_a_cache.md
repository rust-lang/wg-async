# 😱 Status quo stories: Alan tries to cache requests, which doesn't always happen.

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!


## The story

[Alan][] is working on an HTTP server. The server makes calls to some other service. The performance of the downstream service is somewhat poor, so Alan would like to implement some basic caching.

[Alan]: ../characters/alan.md

Alan writes up some code which does the caching:

```rust
async fn get_response(&mut self, key: String) {
    // Try to get the response from cache
    if let Some(cached_response) = self.cache.get(key) {
        self.channel.send(cached_response).await;
        return;
    }

    // Get the response from the downstream service
    let response = self.http_client.make_request(key).await;
    self.channel.send(response).await;
    
    // Store the response in the cache
    self.cache.set(key, response);
}
```

Alan is happy with how things are working, but notices every once in a while the downstream service hangs. To prevent that, Alan implements a timeout.

He remembers from the documentation for his favorite runtime that there is the `race` function which can kick off two futures and polls both until one completes (similar to tokio's [select](https://docs.rs/tokio/1.5.0/tokio/macro.select.html) and async-std's [race](https://docs.rs/async-std/1.9.0/async_std/future/trait.Future.html#method.race) for example).


```rust
runtime::race(timeout(), get_response(key)).await
```

## The bug

Alan ships to production but after several weeks he notices some users complaining that they receive old data.

Alan looks for help. The compiler unfortunately doesn't provide any hints. He turns to his second best friend clippy, who cannot help either.
Alan tries debugging. He uses his old friend `println!`. After hours of working through, he notices that sometimes the line that sets the response in the cache never gets called.

## The solution

Alan goes to [Barbara][] and asks why in the world that might be ⁉️

💡 Barbara looks through the code and notices that there is an await point between sending the response over the channel and setting the cache.

Since the `get_response` future can be dropped at each available await point, it may be dropped *after* the http request has been made, but *before* the response has successfully been sent over the channel, thus not executing the remaining instructions in the function.

This means the cache might not be set.

Alan fixes it by setting the cache before sending the result over the channel. 🎉

```rust
async fn get_response(&mut self, key: String) {
    // ... cache miss happened here

    // We perform the HTTP request and our code might continue
    // after this .await once the HTTP request is complete
    let response = self.http_client.make_request(key).await;

    // Immediately store the response in the cache
    self.cache.set(key, response);

    self.channel.send(response).await;
}
```

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

* Futures can be "canceled" at any await point. Authors of futures must be aware that after an await, the code might not run.
    * This is similar to `panic` safety but way more likely to happen
* Futures might be polled to completion causing the code to work. But then many years later, the code is changed and the future might conditionally not be polled to completion which breaks things.
* The burden falls on the user of the future to poll to completion, and there is no way for the lib author to enforce this - they can only document this invariant.
* Diagnosing and ultimately fixing this issue requires a fairly deep understanding of the semantics of futures.
* Without a Barbara, it might be hard to even know where to start: No lints are available, Alan is left with a normal debugger and `println!`.

### **What are the sources for this story?**
The relevant sources of discussion for this story have been gathered [in this github issue](https://github.com/rust-lang/wg-async-foundations/issues/65).

### **Why did you choose Alan to tell this story?**
Alan has enough experience and understanding of push based async languages to make the assumptions that will trigger the bug.

### **How would this story have played out differently for the other characters?**
This story would likely have played out the same for almost everyone but Barbara, who has probably been bitten by that already.
The debugging and fixing time would however probably have varied depending on experience and luck.
