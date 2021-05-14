# Status quo of an AWS engineer: Solving a deadlock

Alan logs into work the next morning to see a message in Slack:

> Alan, I've noticed that the code to replicate the shards across the hosts is sometimes leading to a deadlock. Any idea what's going on?

This is the same code that Alan tried to parallelize earlier. He pulls up the function, but everything *seems* correct! It's not obvious what the problem could be.

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
        Some(Err(e)) => return Err(e).map_err(box_err)?,
        None => return Ok(()),
    }
}
```

He tries to reproduce the deadlock. He is able to reproduce the problem readily enough, but only with larger requests. He had always used small tests before. He connects to the process with the debugger but he can't really make heads or tails of what tasks seem to be stuck (see [Alan tries to debug a hang](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_tries_to_debug_a_hang.html) or [Barbara wants async insights](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/barbara_wants_async_insights.html)). He resorts to sprinkling logging everywhere.

At long last, he starts to see a pattern emerging. From the logs, he sees the data from each chunk is being sent to the hyper channel, but it never seems to be sent over the HTTP connection to the backend hosts. He is pretty confused by this -- he thought that the futures he pushed into `host_futures` should be taking care of sending the request body out over the internet. He goes to talk to Barbara -- [who, as it happens, has been through this very problem in the past](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/barbara_battles_buffered_streams.html) -- and she explains to him what is wrong.

"When you push those futures into `FuturesUnordered`", she says, "they will only make progress when you are actually awaiting on the stream. With the way the loop is setup now, the actual sending of data won't start until that third loop. Presumably your deadlock is because the second loop is blocked, waiting for some of the data to be sent."

"Huh. That's...weird. How can I fix it?", asks Alan.

"You need to spawn a separate task," says Barbara. "Something like this should work." She modifies the code to spawn a task that is performing the third loop. That task is spawned before the second loop starts:

```rust=
// Prepare the outgoing HTTP requests to each host:
let mut host_senders: Vec<hyper::body::Sender> = vec![];
let mut host_futures = FuturesUnordered::new();
for host in hosts {
    let (sender, body) = hyper::body::Body::channel();
    host_senders.push(sender);
    host_futures.push(create_future_to_send_request(body));
}

// Make sure this runs in parallel with the loop below!
let send_future = tokio::spawn(async move {
    // Wait for all HTTP requests to complete, aborting on error:
    loop {
        match host_futures.next().await {
            Some(Ok(response)) => handle_response(response)?,
            Some(Err(e)) => break Err(e)?,
            None => break Ok(()),
        }
    }
});

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

send_future.await
```

"Oof", says Alan, "I'll try to remember that!"
