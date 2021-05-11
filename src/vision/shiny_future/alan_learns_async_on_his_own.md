# ✨ Shiny future stories: Alan learns async on his own

## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the
brainstorming period. It is derived from what actual Rust users wish
async Rust should be, and is meant to deal with some of the challenges
that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the
FAQ, feel free to open a PR making edits (but keep in mind that, as
peoples needs and desires for async Rust may differ greatly, shiny
future stories [cannot be wrong]. At worst they are only useful for a
small set of people or their problems might be better solved with
alternative solutions). Alternatively, you may wish to [add your own
shiny vision story][htvsq]!

## The story

[Alan] is trying to pick up Rust, and wants to build a command-line web
scraper since it's a project he's recently written in Go. The program
takes a URL, and recursively downloads all URLs named in all fetched
pages.

Alan goes to crates.io and searches for "http client", and finds a
library called `reqwest`. He opens its documentation, and see that the
library has him choose between an "async" and a "blocking" client.
Confused, Alan types in "rust async" in his favorite search engine, and
finds the [Rust async book]. On the very first page there's a summary of
where async is useful and where it's not, as well as some of the
downsides of each approach. Alan sees that for "make a single web
request", async is not generally necessary, whereas for "making many
network requests concurrently" async is recommended. Since Alan expects
his crawler to make many requests, he decides he probably wants
async for this application.

The async book tells Alan that he should mark his `main` function as
`async fn`, so he does. He then follows the `reqwest` async examples,
and is able to successfully make is crawler download a single web page.
Next, he wants to parse each page to extract additional URLs to fetch.
So, he finds a library that can parse HTML, `quick-xml`. He sets up his
application with a `HashSet` to store all the yet-to-be-parsed URLs, and
then writes a loop that pulls out a URL from the set, issues a HTTP
request, awaits the response bytes, and passes them to `quick-xml`. Alan
first tried to give the `http::Response` directly to
`quick_xml::Reader::from_reader`, but the compiler told him:

```text
error: This type does not implement `Read`, which is required by `Reader::from_reader`.

    let page = Reader::from_reader(request.await?);
                                   ^^^^^^^^^^^^^^

      help: The type does implement `AsyncRead`, but the method does not support asynchronous inputs.
suggestion: Use a method that supports asynchronous readers or read the data to a `Vec<u8>` first,
            and then pass that to `Reader::from_reader` instead (`Vec<u8>` implements `Read`).
```

Alan has his program iterate over all the links on the fetched page, and
add any URLs he finds to the `HashSet`, before he then goes around the
loop again. He is pretty satisfied -- the program seems to work well.
However, it's fairly slow, as it only fetches one page at a time. Alan
looks in the async book he discovered earlier, and sees a chapter titled
"Doing many things at once". The chapter tells Alan that he has three
options:

 - use _select_ to wait for the first of many futures to complete;
 - use _join_ to wait on many futures to all complete; and
 - use _spawn_ to run a future in the background.

Alan figures that his program should keep many requests in flight at the
same time, and then parse each one as it finishes, so he goes for the
select approach. He writes:

```rust
let mut requests = Select::new();
requests.insert(client.get(start_url).send());
while !requests.is_empty() {
    let response = requests.await;
    // Use quick-xml to extract urls from response.
    // For each url:
        if seen_urls.insert(url.clone()) {
            requests.insert(client.get(url).send());
        }
}
```

This works, and Alan is delighted. But it seems to work a bit _too_ well
-- his crawler is so fast that it starts getting rate-limited by the
servers he runs it against. So, Alan decides to make his crawler a bit
less aggressive, and adds a call to `std::thread::sleep` after he parses
each page. He compiles his application again, and sees a new warning
from the compiler:

```text
warning: blocking call in asynchronous code

    std::thread::sleep(Duration::from_secs(1));
    ^^^^^^^^^^^^^^^^^^

      help: If the thread is put to sleep, other asynchronous code running
            on the same thread does not get to run either.
suggestion: Use the asynchronous std::future::sleep method instead of std::thread::sleep in async code.
   reading: See the "Blocking in async code" chapter in the Rust async book for more details.
```

Alan is happy that the compiler told him about this problem up front,
rather than his downloads being held up during the entire sleep period!
He does as the compiler instructs, and replaces `thread::sleep` with its
asynchronous alternative and an `await`. He then runs his code again,
and the warning is gone, and everything seems to work correctly.

While looking at his code in his editor, however, Alan notices a little
yellow squiggly line next to his `while` loop. Hovering over it, he sees
a warning from a tool called "Clippy", that says:

```text
warning: 

    while !requests.is_empty() {
    ^^^^^^^^^^^^^^^^^^^^^^^^^^ this loop

        let response = requests.await;
                       ^^^^^^^^^^^^^^ awaits one future from a `Select`
    
    
        std::future::sleep(Duration::from_secs(1)).await;
        ^^^^^^^^^^^^^^^^^^ and then pauses, which prevents progress on the `Select`
    

      help: Futures do nothing when they're not being awaited,
            so while the task is asleep, the `Select` cannot make progress.
suggestion: Consider spawning the futures in the `Select` so they can run in the background.
   reading: See the "Doing many things at once" chapter in the Rust async book for more details.
```

Alan first searches for "rust clippy" on his search engine of choice,
and learns that it is a linter for Rust that checks for common mistakes
and cases where code can be more idiomatic. He makes a mental note to
always run Clippy from now on. 

Alan recognizes the recommended chapter title from before, and sure
enough, when he looks back on the page that made him choose select, he
sees a box explaining that, as the warning suggests, a `Select` only
makes progress on the asynchronous tasks it contains when it is being
awaited. The same box also suggests to _spawn_ the tasks before placing
them in the `Select` to have them continue to run even after the
`Select` has yielded an item.

So, Alan modifies his code to spawn each request:

```rust
// For each url:
if seen_urls.insert(url.clone()) {
    requests.insert(std::future::spawn(async { 
        client.get(url).send().await
    }));
}
```

But now his code doesn't compile any more:

```text
error: borrow of `client` does not live long enough:

    let client = request::Client::new();
        ^^^^^^ client is created here

    requests.insert(std::future::spawn(async {
                    ^^^^^^^^^^^^^^^^^^ spawn requires F: 'static

        client.get(url).send().await
        ^^^^^^ this borrow of client makes the `async` block have lifetime 'a

    }
    ^ the lifetime 'a ends here when `client` is dropped.

      help: An async block that needs access to local variables cannot be spawned,
            since spawned tasks may run past the end of the current function.
suggestion: Consider using `async move` to move `client` if it isn't needed elsewhere,
            or keep `client` around forever by using `Arc` for reference-counting,
            and then `clone` it before passing it into each call to `spawn`.
   reading: See the "Spawning and 'static" chapter in the Rust async book for more details.
```

> Author note: the recommendation `Arc` above should be inferred from
> the `Send` bound on `spawn`. If such a bound isn't present, we should
> recommend `Rc` instead. Ideally we would also tailor the suggestion to
> whether changing `async` to `async move` would _actually_ make the
> code compile.

Alan is amazed at how comprehensive the compiler errors are, and is glad
to see a reference to the async book, which he now realizes he should
probably just make time to read start-to-finish, as it covers everything
he's running into. Alan first tries to change `async` to `async move` as
the compiler suggests, but the compiler then tells him that `client` may
be used again in the next iteration of the loop, which makes Alan
facepalm. Instead, he does as the compiler tells him, and puts the
`client` in an `Arc` and `clone`s that `Arc` for each `spawn`.

At this point, the code looks a little messy, so Alan decides to open
the referenced chapter in the async book as well. It suggests that
while the pattern he's used is a good fallback, it's often possible to
_construct_ the future outside the spawn, and then `await` it inside the
spawn. Alan gives that a try by removing the `Arc` again and writing:

```rust
let fut = client.get(url).send();
requests.insert(std::future::spawn(async move {
    fut.await
}));
```

> Author note: how would the compiler tell Alan about this
> transformation rather than him having to discover it in the book?

This works, and Alan is happy! Doubly-so when he notices the yellow
Clippy squiggles telling him that the `async move { fut.await }` can be
simplified to just `fut`.

Alan runs his crawler again, and this time it doesn't run afoul of any
rate limiting. However, Alan notices that it's still just parsing one
page's HTML at a time, and wonders if he can parallelize that part too.
He figures that since each spawned future runs in the background, he can
just do the XML parsing in there too! So, he refactors the code for
going from a URL to a list of URLs into its own `async fn urls`, and
then writes:

```rust
async fn urls(client: &Client, url: Url) -> Vec<Url> { /* .. */ }

let mut requests = Select::new();
requests.insert(spawn(urls(&client, start_url)));
while !requests.is_empty() {
    let urls = requests.await;
    for url in urls {
        if seen_urls.insert(url.clone()) {
            requests.insert(spawn(urls(&client, url)));
        }
    }
    sleep(Duration::from_secs(1)).await;
}
```

However, to Alan's surprise, this no longer compiles, and is back to the 
old `'static` error:

```text
error: borrow of `client` does not live long enough:

    let client = request::Client::new();
        ^^^^^^ client is created here

    requests.insert(spawn(urls(&client, start_url)));
                    ^^^^^ spawn requires F: 'static

    requests.insert(spawn(urls(&client, start_url)));
                               ^^^^^^^ but the provided argument is tied to the lifetime of this borrow

    }
    ^ which ends here when `client` is dropped.

      help: When you call an `async fn`, it does nothing until it is first awaited.
            For that reason, the `Future` that it returns borrows all of the `async fn`'s arguments.
suggestion: If possible, write the `async fn` (`urls`) as a regular `fn() -> impl Future` that
            first uses any arguments that aren't needed after the first `await`, and then
            returns an `async move {}` with the remainder of the function body.

            Otherwise, consider making the arguments reference-counted with `Arc` so that the async
            function's return value does not borrow anything from its caller.
   reading: See the "Spawning and 'static" chapter in the Rust async book for more details.
```

With the compiler's helpful explanation, Alan realizes that this is
another instance of the same problem he had earlier, and changes his
`async fn` to:

```rust
fn urls(client: &Client, url: Url) -> impl Future<Output = Vec<Url>> {
    let fut = client.get(url).send();
    async move {
        let response = fut.await;
        // Use quick-xml to extract URLs to return.
    }
}
```

At which point the code once again compiles, and runs faster than ever
before! However, when Alan runs his crawler against a website with
particularly large pages, he notices a new warning in his terminal when
the crawler is running:

```text
******************** [ Scheduling Delay Detected ] *********************
The asynchronous runtime has detected that asynchronous tasks are
occasionally prevented from running due to a long-running synchronous
operation holding up the executing thread.

In particular, the task defined at src/lib.rs:88 can make progress, but
the executor thread that would run it hasn't executed a new asynchronous
task in a while. It was last seen executing at src/lib.rs:96.

This warning suggests that your program is running a long-running or
blocking operation somewhere inside of an `async fn`, which prevents
that thread from making progress on concurrent asynchronous tasks. In
the worst instance, this can lead to deadlocks if the blocking code
blocks waiting on some asynchronous task that itself cannot make
progress until the thread continues running asynchronous tasks.

You can find more details about this error in the "Blocking in async
code" chapter of the Rust async book.

This warning is only displayed in debug mode.
************************************************************************
```

Looking at the indicated lines, Alan sees that line 88 is:

```rust
requests.insert(spawn(urls(&client, url)));
```

And line 96 is the `loop` around:

```rust
match html_reader.read_event(&mut buf) {
    // ...
}
```

Alan thinks he understands what the warning is trying to tell him, but
he's not quite sure what he should do to fix it. So he goes to the
indicated chapter in the async book, which says:

> If you have to run a long-running synchronous operation, or issue a
> blocking system call, you risk holding up the execution of
> asynchronous tasks that the current thread is responsible for
> managing until the long-running operation completes. You have many
> options for mitigating the impact of such synchronous code, each with
> its own set of trade-offs.

It then suggests:

 - Try to make the synchronous code asynchronous if possible. This could
   even just consist of inserting occasional voluntary scheduling points
   into long-running loops using `std::future::yield().await` to allow
   the thread to continue to make progress on asynchronous tasks.
 - Run the synchronous code in a dedicated thread using
   `spawn_blocking` and simply `await` the resulting `JoinHandle` in the
   asynchronous code.
 - Inform the runtime that the current thread (with `block_in_place`)
   that it should give away all of its background tasks to other runtime
   threads (if applicable), and only then execute the synchronous code.

The document goes into more detail about the implications of each
choice, but Alan likes the first option the best for this use-case, and
augments his HTML reading loop to occasionally call
`std::future::yield().await`. The runtime warning goes away.

[Rust async book]: https://rust-lang.github.io/async-book/

## 🤔 Frequently Asked Questions

### What status quo stories are you retelling?

 - [Alan tries to debug a hang](../status_quo/alan_tries_to_debug_a_hang.html)
 - [Barbara anguishes over HTTP](../status_quo/barbara_anguishes_over_http.html)
 - [Barbara bridges sync and async in perf.rust-lang.org](../status_quo/barbara_bridges_sync_and_async.html)
 - [Barbara compares some C++ code](../status_quo/barbara_compares_some_cpp_code.html)
 - [Barbara makes their first foray into async](../status_quo/barbara_makes_their_first_steps_into_async.html)
 - [Niklaus wants to share knowledge](../status_quo/niklaus_wants_to_share_knowledge.html)

### What are the key attributes of this shiny future?

 - Not every use-case requires async, and users should be told early on
   that that's the case, and enough to make the decision themselves!
 - Compiler errors and warnings should recognize _specific_ common
   mistakes and recommend good general patterns for solutions.
 - Warnings and errors should refer users to more comprehensive
   documentation for in-depth explanations and best practices.
 - A shared terminology (`AsyncRead`) and standard locations for key
   primitives (`sleep`, `spawn`, `Select`) is needed to be able to
   provide truly helpful, actionable error messages.
 - Async Rust has some very particular problem patterns which are
   important to handle correctly. Misleading error messages like "add
   `'static` to your `&mut`" or "add `move`" can really throw developers
   for a loop by sending them down the wrong rabbit hole.
 - Detecting known cases of blocking (even if imperfect) could help
   users significantly in avoiding foot-guns. Some cases are:
   using `std::thread::sleep`, loops without `.await` in them (or where
   all the `.await`s are on `poll_fn` futures), calling methods that
   transitively call `block_on`.

### What is the "most shiny" about this future? 

The ability to detect issues that _would_ be performance problems at
runtime at compile-time.

### What are some of the potential pitfalls about this future?

Detecting blocking is tricky, and likely subject to both false-positives
and false-negatives. Users _hate_ false-positive warnings, so we'll have
to be careful about when we give warnings based on what _might_ happen
at runtime.

### Did anything surprise you when writing this story? Did the story go any place unexpected?

I wasn't expecting it to end up this long and detailed!

I also wasn't expecting to have to get into the fact that `async fn`s
capture their arguments, but got their very quickly by just walking
through what I imagine Alan's thought process and development would be
like.

### What are some variations of this story that you considered, or that you think might be fun to write? Have any variations of this story already been written?

 - How does Alan realize the difference between `Select` (really
   `FuturesUnordered`) and `select!` (where the branches are known
   statically)?
 - Another common pain-point is forgetting to pin futures when using
   constructs like `select!`. Can the compiler detect this and suggest
   `std::task::pin!` (and can we have that in `std` please)?
 - Tools that allow the user to introspect the program state at runtime
   and detect things like blocking that way are great, but don't help
   newcomers too much. They won't know about the tools, or what to look
   for.
 - How can we detect and warn about async code that transitively ends up
   calling `block_on`?
 - This story didn't get into taking a `Mutex` and holding it across an
   `.await`, and the associated problems. Nor how a user finds other,
   better design patterns to deal with that situation.
 - A story where Alan uses the docs to decide he _shouldn't_ use async
   would be nice. Including if he then needs to use some library that is
   itself `async` -- how does he bridge that gap? And perhaps one where
   he then later changes his mind and has to move from sync to async.
 - [Barbara plays with async](../status_quo/barbara_plays_with_async.html)
   could also use a similar-style "shining future" story.

### What are some of the things we'll have to figure out to realize this future? What projects besides Rust itself are involved, if any? (Optional)

 - Detecting the async "color" of functions to warn about crossing.
 - Detecting long-running code in runtimes.
 - Standardizing enough core terminology and mechanisms that the
   compiler can both detect specific problems and propose actionable
   solutions

[Alan]: ../characters/alan.md
[htvsq]: ../how_to_vision/shiny_future.md
