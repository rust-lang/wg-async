# 😱 Status quo: DistriData

This is the story of [Alan, Barbara, Grace, and Niklaus][ABG and N] (ABG and N, hereafter) as they work on [DistriData]. It shows the various problems they hit in getting started and how they work around them today.

## Background: building services in Java and C++

Alan and Grace are starting a new project, DistriData. As so often happens, they're trying to move on a tight deadline and to stand up the new service as quickly as they can.

Most services at the company are implemented in Java and have a fairly well supported set of tools. Getting started on a new service can be done in minutes simply by creating a description of your service API in an IDL language and generating the support code from a template. This service comes pre-packaged with a lot of useful metrics and logic for things like dropped and cancelling dead connections. Unfortunately, using Java has its downsides, too. The services can consume a lot of memory, and while most requests complete very quickly, the "tail latencies" for the slowest requests can be very long.

For DistriData, performance is absolutely crucial, and so Alan and Grace are investigating alternatives to Java. They're both experienced engineers at the company, but Alan has only worked on Java services befre. Grace has built a number of C++-based services in the past, but she has had mixed experiences. Performance and resource usage is great, but the maintenance burden is high, and she has been through her share of fire drills related to security vulnerabilities. She's also seen that it's really hard to onboard people onto those services, since every patch potentially introduces so many serious problems. 

Alan and Grace have heard that a lot of people in their company are starting to adopt Rust, and it seems like a promising option. It offers the kind of performance they need, but the type system implies that they won't have to worry about crashes and bugs. This makes Alan feel a bit better: he was nervous about hacking on a C++ system and what kinds of weird problems he might cause without realizing it!

## Finding a Rust IDE

Getting started with Rust is a bit different than what Alan and Grace are used to. There's not much infrastructure. They still define their service interface using the same modeling language, but there is no tooling to generate a server from it, they'll have to do that themselves. Before that, though, they have to learn the basics of the language.

Naturally, the very first thing that Alan does is to tweak his IDE setup. He's accustomed to IntelliJ from his Java work, and he's happy to learn that IntelliJ has support for Rust too. Still, as he plays around with Rust code, he realizes that IntelliJ's support is not nearly at the level of Java. Autocomplete often gets confused. For example, when there are two traits with the same name but coming from different crates, IntelliJ often picks the wrong one. It also has trouble with macros. Still, it works well enough.

Grace, meanwhile, decides to use VSCode. She goes to the Extensions list and finds an extension for the RLS. She installs it but finds that it has quite simple support. Asking around on the `#rust` Slack channel, she learns that most people are using the rust-analyzer plugin. Installing that, she finds the experience is much better. "Why isn't this the default", she wonders.

## Alan learns Rust basics

It's time for Alan to buckle down and learn Rust. He starts by trying to read the Rust book. He spends a week reading it, and gets through a lot of chapters, but he hasn't actually written any code yet. Some of the concepts are feeling pretty confused.

In the past, learning Kotlin, he used the [Kotlin Koans](https://kotlinlang.org/docs/koans.html) and found that it was really fun to work with examples. For Rust, he decides to try some of the projects from [leetcode.com](https://leetcode.com/). Unfortunately, uing leetcode examples turns out to be a horrible idea. The examples are all data structure questions, and he is finding them very difficult. which he has heard is the hardest thing to do in Rust, and of course it's also the thing you do the least in real life. One particular question involves a tree and merging linked lists; searching around he finds the question on leetcode is ["Why is it so damn hard to implement?"](https://leetcode.com/problems/merge-two-sorted-lists/discuss/212315/Rust-solution-%22why-it%27s-so-damn-hard-to-implement%22) He is starting to feel frustrated and to question to decision to use Rust in the first place. (The answer does, however, point him at ["Learning Rust using way too many linked lists"](https://rust-unofficial.github.io/too-many-lists/), which he reads and really enjoys.)

Adapting from Java proves to be trickier than Alan expected. Even simple things, like a *variable*, seems to work differently in Rust. In the JVM, a variable is always a pointer. You can have two variables that refer to the same object, but you can't have other people changing your variables to point to fresh values. Alan can tell from the learning materials that Rust variables are different, but he doesn't really get what they *are*. 

Eventually, things start to fall in place. After working through more programs, Alan starts to get an intuition for what a variable in Rust actually was, and the relationship between borrowing and pointers. Watching some of [Jon Ghengset's videos](https://www.youtube.com/c/jongjengset), he starts to understand how interior mutablity works. Soon he has some prototype projects working, and he feels ready to try and prototype the service itself.

## Grace learns Rust basics

Grace's path to Rust is a bit different. She's familiar with C++, so concepts like a variable come natural to her, but there are other things about Rust that are confusing. XX talk to some C++ folk to get a better idea what these are?

## They start to build something in async Rust

Now that Alan and Grace understand the basics of Rust, they decide to try and build something in Async Rust. The Rust book doesn't seem to have any coverage for async I/O, so they google for "async Rust book". They come to a book called "Asynchronous Programming in Rust" that looks official. It has a promising intro, but it quickly dives into all kinds of details they don't really understand.

Frustrated, they ask in the #rust channel on the company slack. Barbara, a more experienced Rust user, explains how Rust offers a number of runtimes, many of which are tailored to specific purposes. She advises them that most people in the company are using tokio, which is a good "general purpose" runtime. They decide to try that out.

Reading the tokio website, they find it has a nice tutorial. "This is great, it'd be nice if the official async book were more like this", they think. Using the tutorial, they learn a lot of the core concepts of tokio, and construct a simple redis server. They're feeling good.

## Exploring the ecosystem

At its heart, the DistriData service is fairly simple. It accepts HTTP requests with data and then forwards those requests to various backend services to ensure the data is replicated and safe. In Java, their "base service" setup would include an HTTP server, but in Rust they have to roll their own. The tokio tutorial, unfortunately, didn't cover anything like this. Alan and Grace start asking in #rust for advice. They are directed to hyper.

They find this pattern of "lots of choices" repeats as they work on other parts of the service. For each simple thing, there seem to be a number of differet variations: multiple lock implementations, muliple traits for interop, and so forth. Even though they've committed to tokio, they often find references to these other libraries online, which is sometimes confusing.

The "choices" question extends beyond async, as well. It's often hard to evaluate which of the various crates in crates.io is the best. For example, Alan spent some time evaluating crates that do md5 hashing, for example, and found tons of choices. He does some quick performance testing and finds huge differences: openssl seems to be the fastest, so he takes that, but he is worried he may have missed some crates.

## Getting error handling right is tricky

As they make progress on the server, they are feeling increasingly confident in Rust, but some things still seem surprisingly challenging. For example, one day Alan is writing the core loop of DistriData which distributes data. The way it works is that the data is broken into "shards" and each shard has a number of "chunks". He is connected to various backend storage hosts via HTTP, and he needs to send each chunk out to all of them. 

Alan starts by writing some code that uses [`hyper::body::channel`](https://docs.rs/hyper/0.14.7/hyper/body/struct.Body.html#method.channel) to generate a pair of a channel where data can be sent and a resulting HTTP body. He then creates a future for each of those HTTP bodies that will send it to the appropriate host once it is complete. He wants those sends to be executing in the background as the data arrives on the channel, so he creates a [`FuturesUnordered`](https://docs.rs/futures/0.3.14/futures/stream/struct.FuturesOrdered.html) to host them:

```rust
let mut host_senders: Vec<hyper::body::Sender> = vec![];
let mut host_futures = FuturesUnordered::new();
for host in hosts {
    let (sender, body) = hyper::body::Body::channel();
    host_senders.push(sender);
    host_futures.push(create_future_to_send_request(body));
}
```

Next, he wants to iterate through each of the shards. For each shard, he will send each chunk to each of the hosts:

```rust
let mut shards = /* generate a stream of Shards */;
while let Some(chunks) = shards.next().await {
    let chunk_futures = chunks
        .into_iter()
        .zip(&mut host_senders)
        .map(|(chunk, sender)| sender.send_data(chunk)?);

    futures::join_all(chunk_futures).await;
}
```

The last line is giving him a bit of trouble. Each of the requests to send the futures could fail, and he would like to propagate that failure. He's used to writing `?` to propagate an error, but when he puts `?` in `sender.send_data` he gets an error:

```
error[E0277]: the `?` operator can only be applied to values that implement `Try`
  --> src/lib.rs:18:40
   |
18 |                 .map(|(chunk, sender)| sender.send_data(chunk)?);
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `impl futures::Future`
   |
   = help: the trait `Try` is not implemented for `impl futures::Future`
   = note: required by `into_result`
```

"Right," Alan thinks, "I need to await the future." He tries to move the `?` to the result of `join_all`:

```rust
let mut shards = /* generate a stream of Shards */;
while let Some(chunks) = shards.next().await {
    let chunk_futures = chunks
        .into_iter()
        .zip(&mut host_senders)
        .map(|(chunk, sender)| sender.send_data(chunk));

    futures::join_all(chunk_futures).await?;
}
```

But now he sees:

```
error[E0277]: the `?` operator can only be applied to values that implement `Try`
  --> src/lib.rs:20:9
   |
20 |         join_all(chunk_futures).await?;  
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `Vec<std::result::Result<(), hyper::Error>>`
   |
   = help: the trait `Try` is not implemented for `Vec<std::result::Result<(), hyper::Error>>`
   = note: required by `into_result`
```

"Ah," he says, "of course, I have a vector of potential errors, not a single error." He remembers seeing a trick for this somewhere. Searching the web, he finds the example. It takes him a little bit to get the type annotations just right, but he finally lands on:

```rust
while let Some(chunks) = shards.next().await {
    let chunk_futures = chunks
        .into_iter()
        .zip(&mut host_senders)
        .map(|(chunk, sender)| sender.send_data(chunk));

    join_all(chunk_futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
}
```

[playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=aa16c1901a13603df7cd050ecc47e61e)

The loop now works: it sends each chunk from each shard to each host, and propagates errors in a reasonable way. The last step is to write for those writes to complete. To do this, he has until all the data has actually been sent, keeping in mind that there could be errors in these sends too. He writes a quick loop to iterate over the stream of sending futures `host_futures` that he created earlier:

```rust
loop {
    match host_futures.next().await {
        Some(Ok(response)) => handle_response(response)?,
        Some(Err(e)) => return Err(e)?,
        None => return Ok(()),
    }
}
```

It takes him a few tries to get this loop right too. The `Some(Err(e))` case in particular is a bit finnicky. He tried to just `return Err(e)` but it gave him an error, because the of `e` didn't match the more generic `Box<dyn Error>` type that his function returns. He remembered that the `?` operator performs some interconversion, though, and that you can do `Err(e)?` to workaround this particular problem.

He surveys the final function he has built, feeling a sense of satisfaction that he got it to work. Still, he can't help but think that this was an awful lot of work just to propagate errors. Plus, he knows from experience that the errors in Rust are often less useful for finding problems than the ones he used to get in Java. Rust errors don't capture backtraces, for example. He tried to add some code to capture backtraces at one point but it seemed really slow, taking 20ms or so to snag a backtrace, and he knew that would be a problem in production.

```rust
// Prepare the outgoing HTTP requests to each host:
let mut host_senders: Vec<hyper::body::Sender> = vec![];
let mut host_futures = FuturesUnordered::new();
for host in hosts {
    let (sender, body) = hyper::body::Body::channel();
    host_senders.push(sender);
    host_futures.push(create_future_to_send_request(body));
}

// Send each chunk from each shared to each host:
while let Some(chunks) = shards.next().await {
    let chunk_futures = chunks
        .into_iter()
        .zip(&mut host_senders)
        .map(|(chunk, sender)| sender.send_data(chunk));

    join_all(chunk_futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
}

// Wait for all HTTP requests to complete, aborting on error:
loop {
    match host_futures.next().await {
        Some(Ok(response)) => handle_response(response)?,
        Some(Err(e)) => return Err(e)?,
        None => return Ok(()),
    }
}
```

## Trying to parallelize a loop

As Alan reads the loop he just built, he realizes that he ought to be able to process each shared independently. He decides to try spawning the tasks in parallel. He starts by trying to create a stream that spawns out tasks:

```rust
// Send each chunk from each shared to each host:
while let Some(chunks) = shards.next().await {
    tokio::spawn(async move {
        let chunk_futures = chunks
            .into_iter()
            .zip(&mut host_senders)
            .map(|(chunk, sender)| sender.send_data(chunk));

        join_all(chunk_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
    })
}
```

But this is giving him errors about the `?` operator again:

```
error[E0277]: the `?` operator can only be used in an async block that returns `Result` or `Option` (or another type that implements `Try`)
  --> src/lib.rs:21:13
   |
15 |            tokio::spawn(async move {
   |   _________________________________-
16 |  |             let chunk_futures = chunks
17 |  |                 .into_iter()
18 |  |                 .zip(&mut host_senders)
...   |
21 | /|             join_all(chunk_futures)
22 | ||                 .await
23 | ||                 .into_iter()
24 | ||                 .collect::<Result<Vec<_>, _>>()?;
   | ||________________________________________________^ cannot use the `?` operator in an async block that returns `()`
25 |  |         });
   |  |_________- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `Try` is not implemented for `()`
   = note: required by `from_error`
```

Annoyed, he decides to convert those to `unwrap` calls temporarily (which will just abort the process on error) just to see if he can get something working:

```rust
    while let Some(chunks) = shards.next().await {
        tokio::spawn(async move {
            let chunk_futures = chunks
                .into_iter()
                .zip(&mut host_senders)
                .map(|(chunk, sender)| sender.send_data(chunk));
    
            join_all(chunk_futures)
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
        });
    }
```

But now he gets this error ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=fd22ae9a8a7ec68cb73b2a10ecd702f4)):

```
error[E0382]: use of moved value: `host_senders`
  --> src/lib.rs:15:33
   |
12 |       let mut host_senders: Vec<hyper::body::Sender> = vec![];
   |           ---------------- move occurs because `host_senders` has type `Vec<hyper::body::Sender>`, which does not implement the `Copy` trait
...
15 |           tokio::spawn(async move {
   |  _________________________________^
16 | |             let chunk_futures = chunks
17 | |                 .into_iter()
18 | |                 .zip(&mut host_senders)
   | |                           ------------ use occurs due to use in generator
...  |
24 | |                 .collect::<Result<Vec<_>, _>>().unwrap();
25 | |         });
   | |_________^ value moved here, in previous iteration of loop
```

He removes the `move` keyword from `async move`, but then he sees:

```
error[E0373]: async block may outlive the current function, but it borrows `host_senders`, which is owned by the current function
  --> src/lib.rs:15:28
   |
15 |           tokio::spawn(async {
   |  ____________________________^
16 | |             let chunk_futures = chunks
17 | |                 .into_iter()
18 | |                 .zip(&mut host_senders)
   | |                           ------------ `host_senders` is borrowed here
...  |
24 | |                 .collect::<Result<Vec<_>, _>>().unwrap();
25 | |         });
   | |_________^ may outlive borrowed value `host_senders`
   |
   = note: async blocks are not executed immediately and must either take a reference or ownership of outside variables they use
help: to force the async block to take ownership of `host_senders` (and any other referenced variables), use the `move` keyword
   |
15 |         tokio::spawn(async move {
16 |             let chunk_futures = chunks
17 |                 .into_iter()
18 |                 .zip(&mut host_senders)
19 |                 .map(|(chunk, sender)| sender.send_data(chunk));
20 |     
 ...

error[E0499]: cannot borrow `host_senders` as mutable more than once at a time
  --> src/lib.rs:15:28
   |
15 |            tokio::spawn(async {
   |   ______________________-_____^
   |  |______________________|
   | ||
16 | ||             let chunk_futures = chunks
17 | ||                 .into_iter()
18 | ||                 .zip(&mut host_senders)
   | ||                           ------------ borrows occur due to use of `host_senders` in generator
...  ||
24 | ||                 .collect::<Result<Vec<_>, _>>().unwrap();
25 | ||         });
   | ||         ^
   | ||_________|
   | |__________`host_senders` was mutably borrowed here in the previous iteration of the loop
   |            argument requires that `host_senders` is borrowed for `'static`
```

At this point, he gives up and leaves a `// TODO` comment:

```rust
// TODO: This loop should be able to execute in parallel,
// but I can't figure out how to make it work. -Alan
while let Some(chunks) = shards.next().await {
    ...
}
```

*Editorial comment:* In this case, the channel to which he is sending the data can only receive data from a single sender at a time (it has an `&mut self`). Rust is potentially saving Alan from a nasty data race here. He could have used a mutex around the senders, but he would still hit issues trying to spawn parallel threads because he lacks an API that lets him borrow from the stack.

## Implementing a stream

* As DistriData development continues, Alan finds that 

## Deadlock from nested awaits

* XXX Adapt [this story](../submitted_stories/status_quo/aws_engineer/solving_a_deadlock.md)

## Slowdown from missing waker

## Packets arriving quickly lead to surprising problems

* XXX Adapt stories from Fuchsia engineers about cancellation, select, etc
* Talk about eventually arriving at standard patterns ..?

[ABG and N]: ../characters.md
[DistriData]: ../projects/DistriData.md