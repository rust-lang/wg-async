# Status quo of an AWS engineer: Juggling error handling

For example, one day Alan is writing a loop. In this particular part of [DistriData], the data is broken into "shards" and each shard has a number of "chunks". He is connected to various backend storage hosts via HTTP, and he needs to send each chunk out to all of them. He starts by writing some code that uses [`hyper::body::channel`](https://docs.rs/hyper/0.14.7/hyper/body/struct.Body.html#method.channel) to generate a pair of a channel where data can be sent and a resulting HTTP body. He then creates a future for each of those HTTP bodies that will send it to the appropriate host once it is complete. He wants those sends to be executing in the background as the data arrives on the channel, so he creates a [`FuturesUnordered`](https://docs.rs/futures/0.3.14/futures/stream/struct.FuturesOrdered.html) to host them:

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

"Ah," he says, "of course, I have a vector of potential errors, not a single error." He remembers seeing a trick for dealing with this in his Rust training. Pulling up the slides, he finds the example. It takes him a little bit to get the type annotations just right, but he finally lands on:

```rust=
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

```rust=
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

```rust=
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