# 😱 Status quo stories: Alan tries to cache requests, which doesn't always happen.

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!


## The story

[Alan][] is working on an HTTP server. The server makes calls to some other service. The performance of the downstream service is somewhat poor, so Alan would like to implement some basic caching.

Alan writes up some code which does the caching:

```rust,ignore
async fn get_response(&mut self, key: String) {
    if let Some(cached_response) self.cache.get(key) {
        self.channel.send(cached_response).await;
        return;
    }

    let response = self.http_client.make_request(key).await;
    
    self.channel.send(response).await;
    
    self.cache.set(key, response);
}
```

Alan is happy with how things are working, but notices every once in a while the downstream service hangs. To prevent that, Alan implements a timeout. 
He remembers from the documentation for his favorite runtime that there is the `race` function which can kick off two futures and polls both until one completes:

```rust ,ignore
runtime::race(timeout(), get_response(key)).await
```

## The bug

Alan ships to production but after several weeks he notices some users complaining that the receive old data. 

Alan tries debugging. He uses his old friend `println!`. After hours of working through, he notices that sometimes the line that sets the response in the cache never gets called. 

## The solution

Alan goes to [Barbara][] and asks why in the world that might be ⁉️

💡 Barbara looks through the code and notices that there is an await point between sending the response over the channel and setting the cache.

This means the cache might not be set.

Alan fixes it by setting the cache before sending the result over the channel. 🎉


## Morals 

* Futures can be "canceled" at any await point. Authors of futures must be aware that after an await, the code might not run. 
* This is similar to `panic` safety but way more likely to happen 
* Futures might be polled to completion causing the code to work. But then many years later, the code is changed and the future might conditionally not be polled to completion which breaks things. 
* The burden falls on the user of to poll to completion, and there is no way for the lib author to enforce this - they can only document this invariant. 
* Diagnosing and ultimately fixing this issue requires a fairly deep understanding of the semantics of futures. 
* Without a Barbara, it might be hard to even know where to start.
