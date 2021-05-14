# Status quo of an AWS engineer: Failure to parallelize

As Alan reads the loop he just built, he realizes that he ought to be able to process each shared independently. He decides to try spawning the tasks in parallel. He starts by trying to create a stream that spawns out tasks:

```rust=
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

```rust=
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

```rust=
// TODO: This loop should be able to execute in parallel,
// but I can't figure out how to make it work. -Alan
while let Some(chunks) = shards.next().await {
    ...
}
```

*Editorial comment:* In this case, the channel to which he is sending the data can only receive data from a single sender at a time (it has an `&mut self`). Rust is potentially saving Alan from a nasty data race here. He could have used a mutex around the senders, but he would still hit issues trying to spawn parallel threads because he lacks an API that lets him borrow from the stack.
