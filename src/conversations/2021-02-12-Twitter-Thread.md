# 🐦 2021-02-12 Twitter thread

Notes taken from the thread in response to [Niko's tweet](https://twitter.com/nikomatsakis/status/1359454255971770372).

* [Enzo](https://twitter.com/enzo_mdd/status/1359544617121820676)
    * A default event loop. "choosing your own event loop" takes time, then you have to understand the differences between each event loop etc.
    * Standard way of doing `for await (variable of iterable)` would be nice.
    * Standard promise combinators.
* [creepy_owlet](https://twitter.com/creepy_owlet/status/1359649695103131649)
    * https://github.com/dtantsur/rust-osauth/blob/master/src/sync.rs
* async trait --
    * https://twitter.com/jcsp_tweets/status/1359820431151267843
    * "I thought async was built-in"?
    * nasty compiler errors
    * [ownership puzzle](https://www.fpcomplete.com/blog/ownership-puzzle-rust-async-hyper/) blog post
* [rubdos](https://twitter.com/rubdos/status/1359462402702606336)
    * [blog post](https://www.rubdos.be/corona/qt/rust/tokio/actix/2020/05/23/actix-qt.html) describes integrating two event loops
    * mentions desire for runtime independent libraries
    * qt provides a mechanism to integrate one's own event loop
    * [llvm bug](https://github.com/rust-lang/rust/issues/60605) generates invalid arm7 assembly
* [alexmiberry](https://twitter.com/alexmiberry/status/1359559299161325581)
    * kotlin/scala code, blocked by absence of async trait
* helpful blog post
    * [jamesmcm](http://jamesmcm.github.io/blog/2020/05/06/a-practical-introduction-to-async-programming-in-rust/)
        * note that `join` and `Result` play poorly together
            * [async-std version](https://github.com/jamesmcm/async-rust-example/blob/async-std/client_async/src/main.rs#L50-L59)
            * [tokio version](https://github.com/jamesmcm/async-rust-example/blob/master/client_async/src/main.rs#L40-L61) has some wild "double question marks" -- I guess that spawn must be adding a layer of `Result`?
    * the post mentions rayon but this isn't really a case where one ought to use rayon -- still, Rayon's APIs here are SO much nicer :)
    * [rust aws and lambda](http://jamesmcm.github.io/blog/2020/04/19/data-engineering-with-rust-and-aws-lambda/#en)
* [issue requiring async drop](https://github.com/jamesmcm/s3rename/issues/16)
* [fasterthanlime](https://fasterthanli.me/articles/getting-in-and-out-of-trouble-with-rust-futures) -- 
    * this post is amazing
    * the discussion on Send bounds and the ways to debug it is great
* [bridging different runtimes using GATs](https://github.com/thanethomson/async-channel-abs/blob/master/src/runtime.rs)
* first server app, [great thread with problems](https://twitter.com/richardsabow/status/1345815109201842178)
    * "I wasn't expecting that it will be easy but after Go and Node.js development it felt extremely hard to start off anything with Rust."
    * "felt like I have to re-learn everything from scratch: structuring project and modules, dependency injection, managing the DB and of course dealing with concurrency"
    * common thread: poor docs, though only somewhat in async libraries
        * I had enums in the DB and it was a bit more complex to map them to my custom Rust enums but I succeeded with the help of a couple of blog posts – and not with Diesel documentation
        * I used Rusoto for dealing with AWS services. It's also pretty straightforward and high quality package – but again the documentation was sooo poor. 
* [implaustin](https://t.co/4rlyfUlFES?amp=1) wrote a [very nice post](https://t.co/4rlyfUlFES?amp=1) but it felt more like a "look how well this worked" post than one with actionable feedback
    * "Async has worked well so far.  My top wishlist items are Sink and Stream traits in std.  It's quite difficult to abstract over types that asynchronously produce or consume values."
    * "AsyncRead/AsyncWrite work fine for files, tcp streams, etc.  But once you are past I/O and want to pass around structs, Sink and Stream are needed.  One example of fragmentation is that Tokio channels used to implement the futures Sink/Stream traits, but no longer do in 1.0."
    * "I usually use Sink/Stream to abstract over different async channel types.  Sometimes to hide the details of external dependencies from a task (e.g. where is this data going?).  And sometimes to write common utility methods."
    * "One thing I can think of: there are still a lot of popular libraries that don't have async support (or are just getting there).  Rocket, Criterion, Crossterm's execute, etc."
* [EchoRior](https://twitter.com/EchoRior/status/1359964691305496579):
    * "I've written a bit of rust before, but rust is my first introduction to Async. My main gripes are that it's hard to figure our what the "blessed" way of doing async is. I'd love to see async included in the book, but I understand that async is still evolving too much for that."
    * "Adding to the confusion: theres multiple executors, and they have a bit of lock in. Async libraries being dependent on which executor version I use is also confusing for newcomers. In other langs, it seems like one just uses everything from the stdlib and everything is compatible"
    * "That kind of gave me a lot of hesitation/fomo in the beginning, because it felt like I had to make some big choices around my tech stack that I felt I would be stuck with later. I ended up chatting about this in the discord & researching for multiple days before getting started."
    * "Also, due to there not being a "blessed" approach, I don't know if I'm working with some misconceptions around async in rust, and will end up discovering I will need to redo large parts of what I wrote."
