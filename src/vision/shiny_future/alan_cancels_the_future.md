# ✨ Shiny future stories: template

[How To Vision: Shiny Future]: ../how_to_vision/shiny_future.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async-foundations/master/src/vision/shiny_future/template.md
[`shiny_future`]: https://github.com/rust-lang/wg-async-foundations/tree/master/src/vision/shiny_future
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async-foundations/blob/master/src/SUMMARY.md

## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The stories

### Alan's tale: Basics of async drop

Alan has been adding an extension to YouBuy that launches a singleton actor which interacts with a Sqlite database using the `sqlx` crate. The Sqlite database only permits a single active connection at a time, but this is not a problem, because the actor is a singleton, and so there only should be one at a time. He consults the documentation for `sqlx` and comes up with the following code to create a connection and do the query he needs:

```rust
use sqlx::Connection;

async fn process_connection() -> Result<(), sqlx::Error> {
    // Create a connection

    let conn = SqliteConnection::connect("sqlite::memory:").await?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&conn).await?;

    assert_eq!(row.0, 150);

    Ok(())
}
```

He tries it out and observes that when the connection is done, the `SqliteConnection` is closed automatically. He experiments a bit with a "torture test" driver he has, which starts connections and then cancels them at random intervals, and everything seems to work smoothly. When the connections drop early, the future can get canceled at the various await points, but the cleanup code for the `SqliteConnection` still runs.

#### Implementing drop

As he continues to implement, Alan is adding in a structure that ties together a few different connects. He needs to do a bit of cleanup in the case of a canceled transaction. He reads into Rust and learns about the `Drop` trait and its async cousin, `AsyncDrop`. This seems like exactly what he wants.

```rust
struct TransactionContext {
    sqlx: SqliteConnection,
    log: DataLog,
    data_record: DataRecord,
}

impl AsyncDrop for TransactionContext {
    async fn drop(&mut self) {
        self.log.log_cancellation(&self.data_record, &self.sqlx).await;
    }
}
```

Now when cancellation occurs, his `drop` code runs first. After it completes, the `SqliteConnection` cleanup runs.

### Grace's tale: Tuning performance

Grace is working on [DistriData]. Things are generally working fairly well, but the latency is higher than she would like. She opens up the [TurboWish](./barbara_makes_a_wish.md) tool and sets it up to generate profiles per request. Looking at the flamegraph, she realizes that the `do_the_thing` function is spending quite a bit of time at the end of an `if` statement, dropping the `conn` variable:

```rust
async fn do_the_thing() {
    if check_database {
        let conn = DbConnection::connect("sqlite::memory:").await?;
        ...
    } // <-- time is being spent here, dropping `conn`
}
```

She realizes that `DbConnection` blocks until the connection with the database is closed, but that isn't really necessary for her use case. She'd rather enqueue the closing of the connection to run asynchronously in a separate task.

```rust
async fn do_the_thing() {
    if check_database {
        let conn = DbConnection::connect("sqlite::memory:").await?;
        ...
        std::async_task::spawn(|| async_drop(conn).await);
    } // <-- time is being spent here, dropping `conn`
}
```

She shows the code to Barbara, who mentions the utility method `spawn_drop` from the `AsyncDrop` trait. `spawn_drop` just takes ownership of the `self` parameter and enqueues it be dropped asynchronously at some point. Barbara rewrites her code as follows:

```rust
async fn do_the_thing() {
    if check_database {
        let conn = DbConnection::connect("sqlite::memory:").await?;
        ...
        conn.spawn_drop();
    } // <-- time is being spent here, dropping `conn`
}
```

The respone latency is looking much better now! "Still, that was a bit subtle," Grace thinks. "I wonder if there are other cases like that in my code."

### Barbara's tale: Smooth interop with the rest of Rust

#### Implementing AsyncDrop for the containers

Barbara is working on extending the `Vec` type in the standard library to support async drop. The `Vec` type has a custom drop impl that first drops all of the items in the vector before freeing the vector's internal buffer. 

```rust
impl<T> AsyncDrop for BVec<T> {
    async fn drop(&mut self) {

    }
}
```

#### Dropping in a sync context

Barbara is working on a separate project in which she has a custom main. One of the first things her program does is to create a future `prepare_the_thing`; before launching it, though, the program sometimes errors out under some conditions.

```rust
fn main() {
    let the_future = prepare_the_thing();

    if some_condition {
        eprintln!("Error: foo");
        return;
    }

    custom_runtime::launch(the_future);
}

fn prepare_the_thing() -> impl Future<Output = ()> {
    ...
    async move { ... }
}
```

As it happens, `prepare_the_thing` returns a future that has ownership of various items that implement `AsyncDrop`. While reading over the code, Barbara wonders, "What happens when this future is dropped synchronously?" Reading the docs for `AsyncDrop`, though, she learns that -- by default, at least -- the synchronous `Drop` invokes the asynchronous drop, using the simple `block_on` mechanism included in the standard library. Testing it, she finds that everything works as expected.

She does wonder, "What if I didn't want `block_on` to block, what could I do then?" The book explains that she can implement the `Drop` trait explicitly and make it to do whatever she wants, including (for example) invoking `spawn_drop` to run in the background.

### TODO

* Work in some part of the story that addresses [the destructor state problem](https://boats.gitlab.io/blog/post/poll-drop/) from boat's blog post. How do we manage generic code that doesn't know whether the future produced by `Drop` is send, and how do we walk about that? Similarly trait objects?

## 🤔 Frequently Asked Questions

### What status quo stories are you retelling?

*Link to status quo stories if they exist. If not, that's ok, we'll help find them.*

### What are the key attributes of this shiny future?

* There is an `AsyncDrop` trait that you can implement instead of `Drop`
    * If you do so, `Drop` will execute your async drop with... `std::block_on`? Something.

### What is the "most shiny" about this future? 

We have seamless interoperability between dropping and async drop. They work quite analogously, and it is possible to write drop implementations that use `await` happily.

### What are some of the potential pitfalls about this future?

The story identifies that sometimes, for performance reasons, teardowns and things should occur asynchronously (no pun intended).

The mechanism desribed in the story doesn't seem that great. Maybe Grace would like to do this when creating the connection, e.g. so she could do something like

```rust
let conn = DbConnection::connect().await?.spawn_when_dropped();
```

and then be assured that the drop of `conn` would not be blocking but would execute in a parallel task?

### Did anything surprise you when writing this story? Did the story go any place unexpected?

*The act of writing shiny future stories can uncover things we didn't expect to find. Did you have any new and exciting ideas as you were writing? Realize some complications that you didn't foresee?*

### What are some variations of this story that you considered, or that you think might be fun to write? Have any variations of this story already been written?

* A `Drop` trait that contains a `poll_drop_ready` method, roughly as proposed in [RFC 2958](https://github.com/rust-lang/rfcs/pull/2958)
    * Writing the `async_drop` method would be much more difficult here, as it would require [using pin](../status_quo/alan_hates_writing_a_stream.md).
    * But it would avoid the need to create and store futures as we unwind.
* Regular drop always run sequential drop, but we have lints that encourage you to write `async_drop(x)` calls explicitly for every type that implements `AsyncDrop`
    * ...or which owns values that implement `AsyncDrop`, which is an interesting semver interaction, but not unprecedented.
* Extending the Rust type system to support "true linear" types that *must* be explicitly dropped in some way
    * For example with `T: ?Drop` annotation; the idea would be that most types implement `Drop`, which allows them to be discarded, but some types can "opt-out" from that.
    * Such types would have to provide various methods that take ownership of the value so that callers can discharge it.
    * The story would want to work through some of the potential pain that arises from this when composing with libraries like iterator and so forth.
    * Need to address libraries like `Vec` that would want to be compatible either way.

### What are some of the things we'll have to figure out to realize this future? What projects besides Rust itself are involved, if any? (Optional)




[comment]: ./comment.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[projects]: ../projects.md
[htvsq]: ../how_to_vision/shiny_future.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
