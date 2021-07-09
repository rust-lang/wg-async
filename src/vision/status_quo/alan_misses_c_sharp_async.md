# 😱 Status quo stories: Alan misses C# async

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async
Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR
making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories
[cannot be wrong], only inaccurate). Alternatively, you may wish to
[add your own status quo story][htvsq]!

## The story

### First attempt

[Alan] has finally gotten comfortable working in rust and finally decides to try writing async code.
He's used C#'s async and mostly loved the experience, so he decides to try writing it the same way:

```rust
async fn run_async() {
    println!("Hello async!");
}

fn main() {
    run_async();
}
```

But the compiler didn't like this:
```
warning: unused implementer of `Future` that must be used
 --> src/main.rs:6:5
  |
6 |     run_async();
  |     ^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: futures do nothing unless you `.await` or poll them
```

Alan has no idea what `Future` is; he's never seen this before and it's not in his code. He sees the
note in the warning and adds `.await` to the line in `main`:
```rust
fn main() {
    run_async().await;
}
```

The compiler does't like this either.
```
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src/main.rs:6:5
  |
5 | fn main() {
  |    ---- this is not `async`
6 |     run_async().await;
  |     ^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
```

... so Alan adds `async` to `main`:
```rust
async fn main() {
    run_async().await;
}
```

which prompts yet another error from the compiler:
```
error[E0277]: `main` has invalid return type `impl Future`
 --> src/main.rs:5:17
  |
5 | async fn main() {
  |                 ^ `main` can only return types that implement `Termination`
  |
  = help: consider using `()`, or a `Result`

error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:5:1
  |
5 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

So Alan decides to do a lot of research online and hunting around on StackOverflow. He learns that
`async fn` returns a value, but it's not the same as the value returned from async functions in C#.
In C#, the object he gets back can only be used to query the result of an already running thread of
work. The rust one doesn't seem to do anything until you call `.await` on it. Alan thinks this is
really nice because he now has more control over when the processing starts. You seem to get the same
control as constructing a `Task` [manually] in C#, but with a lot less effort.

[manually]: https://docs.microsoft.com/en-us/dotnet/api/system.threading.tasks.task?view=net-5.0#task-instantiation

He also ends up finding out a little about executors. `tokio` seems to be really popular, so he
incorporates that into his project:

```rust
async fn run_async() {
    println!("Hello async!");
}

#[tokio::main]
async fn main() {
    run_async().await;
}
```

And it works!
```
Hello async!
```

### Attempting concurrency

Alan decides to try running two async functions concurrently. "This is pretty easy in C#," he
thinks, "This can't be too hard in rust."

In C# Alan would usually write something like:
```csharp
async Task expensive1() {
    ...
}

async Task expensive2() {
    ...
}

public static async Main() {
    Task task = expensive1();
    await expensive2();
    task.Wait();
}
```

If the code was more dynamic, Alan could have also used the Task API to simplify the await:
```csharp
public static Main() {
    List<Task> tasks = new List<Task>();
    tasks.push(expensive1());
    tasks.push(expensive2());
    try {
        Task.WaitAll(tasks.ToArray());
    }
    // Ignore exceptions here.
    catch (AggregateException) {}
}
```

So Alan tries the first approach in rust:

```rust
use std::sync::mpsc::{self, Sender, Receiver};

async fn expensive1(tx: Sender<()>, rx: Receiver<()>) {
    println!("Doing expensive work in 1");
    tx.send(()).ok();
    let _ = rx.recv();
    println!("Got result, finishing processing in 1");
    println!("1 done");
}

async fn expensive2(tx: Sender<()>, rx: Receiver<()>) {
    println!("Doing simple setup in 2");
    let _ = rx.recv();
    println!("Got signal from 1, doing expensive processing in 2");
    tx.send(()).ok();
    println!("2 done");
}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    expensive1(tx1, rx2).await;
    expensive2(tx2, rx1).await;
}
```

But this just hangs after printing:
```
Doing expensive work in 1
```

Alan wonders if this means he can't run code concurrently... he does some research and learns about
`join`, which doesn't seem to be part of the std. This seems like the second example in C#, but Alan
is surprised it doesn't come with the standard library. He has to import `futures` as a dependency
and tries again:
```rust
use futures::join;
...

#[tokio::main]
async fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let fut1 = expensive1(tx1, rx2);
    let fut2 = expensive2(tx2, rx1);
    join!(fut1, fut2);
}
```

But this still hangs the same way as the first attempt. After more research, Alan learns that he
can't use the standard `mpsc::channel` in async contexts. He needs to use the ones in the external
`futures` crate. This requires quite a few changes since the API's don't line up with the one's in
std:
* `rx` has to be `mut`
* there's bounded and unbounded mpsc channels, Alan went with unbounded since the API seemed simpler
for now
* you need to import the `StreamExt` trait to be able to get a value out of `rx`, this took a lot of
research to get right.

```rust
use futures::{
    join,
    channel::mpsc::{self, UnoundedSender, UnboundedReceiver},
    StreamExt,
};
use std::sync::mpsc::{self, Sender, Receiver};

async fn expensive1(tx: Sender<()>, mut rx: Receiver<()>) {
    println!("Doing expensive work in 1");
    tx.unbounded_send(()).ok();
    let _ = rx.next().await;
    println!("Got result, finishing processing in 1");
    println!("1 done");
}

async fn expensive2(tx: Sender<()>, mut rx: Receiver<()>) {
    println!("Doing simple setup in 2");
    let _ = rx.next().await;
    println!("Got signal from 1, doing expensive processing in 2");
    tx.unbounded_send(()).ok();
    println!("2 done");
}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let fut1 = expensive1(tx1, rx2);
    let fut2 = expensive2(tx2, rx1);
    join!(fut1, fut2);
}
```

And now it works!
```
Doing expensive work in 1
Doing simple setup in 2
Got signal from 1, doing expensive processing in 2
2 done
Got result, finishing processing in 1
1 done
```

While this is more similar to using the `Task.WaitAll` from C#, there were a lot more changes needed
than Alan expected.

### Cancelling tasks

Another pattern Alan had to use frequently in C# was accounting for cancellation of tasks. Users in
GUI applications might not want to wait for some long running operation or in a web server some
remote calls might time out. C# has a really nice API surrounding [`CancellationTokens`].

[`CancellationTokens`]: https://docs.microsoft.com/en-us/dotnet/api/system.threading.cancellationtoken?view=net-5.0

They can be used in a fashion similar to (overly simplified example):
```csharp
async Task ExpensiveWork(CancellationToken token) {
    while (not_done) {
        // Do expensive operations...
        if (token.IsCancellationRequested) {
            // Cleanup...
            break;
        }
    }
}

public static async Main() {
    // Create the cancellation source and grab its token.
    CancellationTokenSource source = new CancellationTokenSource();
    CancellationToken token = source.Token;

    // Setup a handler so that on user input the expensive work will be canceled.
    SetupInputHandler(() => {
        // on user cancel
        source.Cancel();
    });

    // Pass the token to the functions that should be stopped when requested.
    await ExpensiveWork(token);
}
```

Alan does some research. He searches for "rust async cancellation" and can't find anything similar.
He reads that "dropping a future is cancelling it". In his junior dev days, Alan might have run with
that idea and moved on to the next task, but experienced Alan knows something is wrong here. If he
drops a `Future` how does he control the cleanup? Which `await` point is the one that will not be
processed? This scares Alan since he realized he could get some really nasty bugs if this happens
in production. In order to work around this, Alan needs to make sure *every* future around critical
code is carefully reviewed for drops in the wrong places. Alan also decided he needs to come up with
some custom code to handle cancelling.

Alan decides to ask around, and gets suggestions for searching with "rust cancel future" or
"rust cancel async". He finds out about tokio's [`tokio_util::sync::CancellationToken`], and also
the [`stop-token`] and [`stopper`] crates. He decides to try working with the version in
`tokio_util` since he's already using `tokio`. Looking at the docs for each, they all seem to
behave how Alan expected, though he couldn't use `stop-token` since that only works with
`async-std`. `stopper` also seems like a good alternative, but he decides to go with the type that
is built by the tokio team.

Reading the docs it seems that the tokio `CancellationToken` acts more like a combination of C#'s
`CancellationTokenSource` and `CancellationToken`. He needs to pass the tokens generated from a call
to `child_token()` and keep the main token for triggering cancellation. One advantage that all of
the token crates seem to have is that they can also integrate directly with streams and futures,
or be polled directly (as a stream or boolean).

[`tokio_util::sync::CancellationToken`]: https://docs.rs/tokio-util/0.6.7/tokio_util/sync/struct.CancellationToken.html
[`stop-token`]: https://docs.rs/stop-token/0.2.0/stop_token/
[`stopper`]: https://docs.rs/stopper/0.2.0/stopper/

```rust
use tokio_util::sync::CancellationToken;
use futures::StreamExt;
// ...

fn generate_work() -> impl Stream<Item = Work> {
    // ...
}

async fn expensive_work(token: CancellationToken) {
    let mut work_stream = generate_work();
    loop {
        if let Some(op) = work_stream.next().await {
            op.work().await;
        } else {
            break;
        }

        if token.is_cancelled() {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let child_token = token.child_token();
    setup_input_handler(move || {
        token.cancel();
    });

    expensive_work(child_token).await;
}
```

This seems relatively straightforward!

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* First Attempt
    * Unused implementer warnings for `Futures` are less clear than they are for, e.g. `Result`.
    * It's not as easy to jump into experimenting with async as compared to synchronous code. It
    requires a lot more front-end research on the user's end.
    * Developers might need to unlearn async behavior from other languages in order to understand
    async rust.
    * Dynamic languages with async provide async main, but rust does not. We could be more helpful
    by explaining this in compiler errors.
* Attempting Concurrency
    * Trying to use items from std is the obvious thing to try, but wrong because they are blocking.
    * The corresponding async versions of the std items don't exist in std, but are in `futures`
    crate. So it's hard to actually develop in async without the `futures` crates.
* Cancelling Tasks
    * It's not obvious that futures could only run part-way.
    * Async types and crates can be bound to certain ecosystems, limiting developers' ability to
    reuse existing code.

### **What are the sources for this story?**
* The docs for [`oneshot::Canceled`] mentions that dropping a `Sender` will cancel the future.
Someone inexperienced might accidentally apply this to a broader scope of types.
* [This IRLO post] has a nice discussion on cancellation, where the [linked gist]
is a thorough overview of problems surrounding cancelation in async rust, with comparisons to other
languages.

[`oneshot::Canceled`]: https://docs.rs/futures/0.3.15/futures/channel/oneshot/struct.Canceled.html
[This IRLO post]: https://internals.rust-lang.org/t/async-await-the-challenges-besides-syntax-cancellation/10287
[linked gist]: https://gist.github.com/Matthias247/ffc0f189742abf6aa41a226fe07398a8

### **Why did you choose Alan to tell this story?**
C# is a garbage collected language that has had async for a long time. Alan best fit the model for
a developer coming from such a language.

### **How would this story have played out differently for the other characters?**
* [Barbara] may already be used to the ideosynchracies of async in rust. She may not realize how
difficult it could be for someone who has a very different model of async engrained into them.
* [Grace] has likely never used async utilities similar to the ones in C# and other GC languages. C
and C++ tend to use callbacks to manage async workflows. She may have been following the C++
proposals for coroutines (e.g. `co_await`, `co_yield`, `co_return`), but similar to rust, the
utilities are not yet thoroughly built out in those spaces. She may be familiar with cancelation in
external libraries like [`cppcoro`](https://github.com/lewissbaker/cppcoro#Cancellation), or async in
general with [`continuable`](https://github.com/Naios/continuable)
* [Niklaus] may not have had enough experience to be wary of some of the pitfalls encountered here.
He might have introduced bugs around dropping futures (to cancel) without realizing it.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
