# 😱 Status quo stories: Template

*This is a template for adding new "status quo" stories. To propose a new status quo PR, do the following:*

* *Create a new file in the [`status_quo`] directory named something like `Alan_tries_to_foo.md` or `Grace_does_bar.md`, and start from [the raw source from this template]. You can replace all the italicized stuff. :)*
* *Do not add a link to your story to the [`SUMMARY.md`] file; we'll do it after merging, otherwise there will be too many conflicts.*

*For more detailed instructions, see the [How To Vision: Status Quo] page!*

*If you're looking for ideas of what to write about, take a look at the [open issues]. You can also [open an issue of your own] to throw out an idea for others.*

[How To Vision: Status Quo]: ../how_to_vision/status_quo.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async-foundations/master/src/vision/status_quo/template.md
[`status_quo`]: https://github.com/rust-lang/wg-async-foundations/tree/master/src/vision/status_quo
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async-foundations/blob/master/src/SUMMARY.md
[open issues]: https://github.com/rust-lang/wg-async-foundations/issues?q=is%3Aopen+is%3Aissue+label%3Astatus-quo-story-ideas
[open an issue of your own]: https://github.com/rust-lang/wg-async-foundations/issues/new?assignees=&labels=good+first+issue%2C+help+wanted%2C+status-quo-story-ideas&template=-status-quo--story-issue.md&title=


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Content of `Cargo.toml` for reproducibility:

```toml
futures = "0.3.14"
hyper = { version = "0.14.7", features = ["full"] }
pretty_env_logger = "0.4.0"
reqwest = "0.11.3"
tokio = { version = "1.5.0", features = ["macros", "rt-multi-thread"] }
```

There is a HTTP server in hyper which Barbara have to query.

```rust
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
 
    loop {
        let (stream, _) = listener.accept().await?;
 
        tokio::spawn(async move {
            let _ = Http::new()  
                .serve_connection(stream, service_fn(serve))
                .await;
        });
    }
}
 
async fn serve(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let res = Response::builder()
        .header("content-type", "text/plain; charset=utf-8")
        .body(Body::from("Hello World!"))
        .unwrap();
    Ok(res)
}

```

## Nice simple query with high-level reqwest

Barbara do HTTP GET request using TCP socket with reqwest and it works fine, everything is easy.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("http://127.0.0.1:3000").await?;
    println!("{}", res.text().await?);
    Ok(()) 
}
```

## Unix sockets for performance

One day, Barbara heard that using unix socket can provide a better performance by using unix socket when both the server and client is on the same machine, so Barbara decided to try it out.

Barbara starts porting the server code to use unix socket, it was a no brainer for Barbara at least. Barbara changed `TcpListener::bind("127.0.0.1:3000").await?` to `UnixListener::bind("/tmp/socket")?` and it works like a charm.

Barbara search through reqwest doc and github issues to see how to use unix socket for reqwest. Barbara found https://github.com/seanmonstar/reqwest/issues/39#issuecomment-778716774 saying reqwest does not support unix socket but hyper does with an example, which is a lower-level library. Since reqwest is so easy and porting hyper server to use unix socket is easy, Barbara thinks low-level hyper library should be easy too.

## The screen stares at Barbara

Barbara wrote some code according to the comments Barbara saw and read some docs like what is `handshake` to roughly know what it does. Barbara compile and it shows a warning, the `connection` variable is not used:
```
warning: unused variable: `connection`
 --> src/main.rs:9:30
  |
9 |     let (mut request_sender, connection) = conn::handshake(stream).await?;
  |                              ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_connection`
  |
  = note: `#[warn(unused_variables)]` on by default
```

Barbara then runs the program. Barbara was stares at the screen and the screen stares at her. Barbara waited and ... it was stuck. So Barbara decides to look at the logs and run it again with `env RUST_LOG=trace cargo r`, and it was indeed stuck, but not sure where.
```
 TRACE mio::poll > registering event source with poller: token=Token(0), interests=READABLE | WRITABLE
```

Barbara try adding `println!` all over the code but it was still stuck, so Barbara try asking for help. Thanks to the welcoming Rust community, Barbara got help quickly in this case. It seemed like Barbara missed the `connection` which is a culprit and it was in the parent module of the docs Barbara read.

Barbara added the missing piece to `.await` for the `connection`, all the while Barbara thought it will work if it was `.await`-ed but in this case having required to await something else to work is a surprise.

```rust
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("error: {}", e);
        }
    })
    
    let request = ...
```

Barbara run the code and it works now, yay! Barbara want to try to reuse the connection to send subsequent HTTP request. Barbara duplicates the last block and it runs.

## Mysterious second request

Some time later, Barbara was told that the program did not work on second request. Barbara tried it but it works. To double confirm, when Barbara tried it again it did not work. Rather than getting stuck, this time there is a error message, which is somewhat better but Barbara did not understand.

The second request is so mysterious, it is like the second request playing hide and seek with Barbara. Sometimes it works and sometimes it does not work.

```rust
 TRACE mio::poll > registering event source with poller: token=Token(0), interests=READABLE | WRITABLE
Some(Ok(b"Hello World!"))
 TRACE want      > signal: Want
 TRACE mio::poll > deregistering event source from poller
 TRACE want      > signal: Closed
Error: hyper::Error(Canceled, "connection was not ready")
```

As a typical method of solving asynchronous issue. Barbara add prints to every await boundaries in the source code to understand what is going on.

```rust
use hyper::{body::HttpBody, client::conn, Body, Request};
use tokio::net::UnixStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let stream = UnixStream::connect("/tmp/socket").await?;

    let (mut request_sender, connection) = conn::handshake(stream).await?;
    println!("connected"); 
                        
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            println!("closed"); 
            eprintln!("error: {}", e);
        }
        println!("closed"); 
    });
                  
    let request = Request::get("/").body(Body::empty())?;
    let mut response = request_sender.send_request(request).await?;
    println!("{:?}", response.body_mut().data().await);
                  
    let request = Request::get("/").body(Body::empty())?;
    println!("sending 2");
    let mut response = request_sender.send_request(request).await?;
    println!("sent 2"); 
    println!("{:?}", response.body_mut().data().await);
                     
    Ok(())
}                    
```

The logs are now more detailed. Barbara can see that the connection was closed but why? Barbara had no idea and Barbara had to seek help again.
```
 TRACE mio::poll > registering event source with poller: token=Token(0), interests=READABLE | WRITABLE
connected
Some(Ok(b"Hello World!"))
sending 2
 TRACE want      > signal: Want
 TRACE mio::poll > deregistering event source from poller
 TRACE want      > signal: Closed
closed
Error: hyper::Error(Canceled, "connection was not ready")
```

This time as well, Barbara was lucky enough to get a quick reply from the welcoming Rust community. Other users said there is a trick for these kind of cases, which is a tracing stream.

```rust
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
        
pub struct TracingStream<S> {
    pub inner: S,
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for TracingStream<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let poll_result = Pin::new(&mut self.inner).poll_read(cx, buf);
        for line in String::from_utf8_lossy(buf.filled()).into_owned().lines() {
            println!("> {}", line);
        }
        poll_result
    }
}
                                 
impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for TracingStream<S> {
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    } 
    
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
 
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let poll_result = Pin::new(&mut self.inner).poll_write(cx, buf);
        for line in String::from_utf8_lossy(buf).into_owned().lines() {
            println!("< {}", line);
        }
        poll_result
    }
}
```

Barbara happily copy pasted the code and wrap the `stream` within `TracingStream`. Running it with logs gives (same thing, in some cases it works, in some cases it did not work):
```
 TRACE mio::poll > registering event source with poller: token=Token(0), interests=READABLE | WRITABLE
connected
< GET / HTTP/1.1
< 
> HTTP/1.1 200 OK
> content-type: text/plain; charset=utf-8
> content-length: 12
> date: Tue, 04 May 2021 17:02:49 GMT
> 
> Hello World!
Some(Ok(b"Hello World!"))
sending 2
 TRACE want      > signal: Want
 TRACE want      > signal: Want
 TRACE mio::poll > deregistering event source from poller
 TRACE want      > signal: Closed
closed
Error: hyper::Error(Canceled, "connection was not ready")
```

Barbara thought this probably only affects a unix socket but nope, even swapping it back with TCP socket does not work either. Now, not just Barbara was confused, even the other developers who offered help was confused now.

## The single magical line

After some time, a developer found a solution, just a single line. Barbara added the line and it works like a charm but it still feels like magic.

```rust
use futures::future;

    // this new line below was added for second request
    future::poll_fn(|cx| request_sender.poll_ready(cx)).await?;
    let request = Request::get("/").body(Body::empty())?;
    println!("sending 2");
    let mut response = request_sender.send_request(request).await?;
    println!("sent 2");
    println!("{:?}", response.body_mut().data().await);
```

Barbara still have no idea why it needs to be done this way. But at least it works now.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**

Barbara is not able to see the problem right away. Usually missing an `await` is an issue but in this case, not awaiting on another variable or not polling for ready when using a low-level library may the program incorrect, it is also hard to debug and figure out what is the correct solution.

### **What are the sources for this story?**

pickfire was experimenting with HTTP client over unix socket and faced this issue as he though it is easy, still a lot thanks to Programatik for helping out with a quick and helpful response.

### **Why did you choose *Barbara* to tell this story?**

Barbara have some experience with synchronous and high-level asynchronous rust libraries but not with low-level asynchronous libraries.

### **How would this story have played out differently for the other characters?**

Most likely everyone could have faced the same issue unless they are lucky.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
