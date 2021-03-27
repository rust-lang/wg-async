# 😱 Status quo stories: Alan finds dropping database handles is hard.

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!


## The problem

Alan has been adding an extension to YouBuy that launches a singleton actor which interacts with a Sqlite database using the `sqlx` crate. The Sqlite database only permits a single active connection at a time, but this is not a problem, because the actor is a singleton, and so there only should be one at a time. He consults the documentation for `sqlx` and comes up with the following code to create a connection and do the query he needs:
  
```rust=
use sqlx::Connection;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
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

Things seem to be working fairly well but sometimes when he refreshes the page he encounters a panic with the message "cannot open a new connecton: connection is already open". He is flummoxed.


## Searching for the Solution


Alan tries to figure out what happened from the logs, but the only information he sees is that a new connection has been received. Alan turns to the documentation for the `sqlx` crate to see if there are flags that might enable extra instrumentation but he can't find any (XXX check what crate offers).

* He does find the [`close` method] which mentions "This method is not required for safe and consistent operation. However, it is recommended to call it instead of letting a connection drop as the database backend will be faster at cleaning up resources."
* He adds a call to `close` into his code and it helps some but he is still able to reproduce the problem if he refreshes often enough. 
* He adds a log statement right before calling `close` to see if it is working:

```rust=
use sqlx::Connection;

#[async_std::main]
async fn do_the_thing() -> Result<(), sqlx::Error> {
    // Create a connection
    let conn = SqliteConnection::connect("sqlite::memory:").await?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&conn).await?; // <----- if this await is cancelled, doesn't help

    assert_eq!(row.0, 150);
    
    // he adds this:
    log!("closing the connection");
    conn.close();

    Ok(())
}
```
  
* He observes that in the cases where he has the problem the log statement never executes.
* He asks Barbara for help and she explains how `await` can be canceled and it will the destructors for things that are in scope.
* He reads the [source for the SqliteConnection destructor](https://github.com/launchbadge/sqlx/blob/0ed524d65c2a3ee2e2a6706910b85bf2bb72115f/sqlx-core/src/pool/connection.rs#L70-L74) and finds that destructor spawns a task to actually close the connection.
* He realizes there is a race condition:
    * the task may not have actually closed the connection before `do_the_thing` is called a second time
* He gives up and gets frustrated

Next, Alan seeks verification and validation of his understanding of the source code from the `sqlx` forum. His advisor states that Rust doesn't have a way to execute async operations in a destructor.

## Finding the Solution

Alan briefly considers rearchitecting his application in more extreme ways to retain use of async, but he gives up and seeks a more straight forward solution. He discovers `rusqlite`, a sychronous database library and adopts it. This requires some rearchitecting but solves the problem.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

* **What are the morals of the story?**
    * *Talk about the major takeaways-- what do you see as the biggest problems.*
* **What are the sources for this story?**
    * *Talk about what the story is based on, ideally with links to blog posts, tweets, or other evidence.*
* **Why did you choose *NAME* to tell this story?**
    * *Talk about the character you used for the story and why.*
* **How would this story have played out differently for the other characters?**
    * *In some cases, there are problems that only occur for people from specific backgrounds, or which play out differently. This question can be used to highlight that.*

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-t
