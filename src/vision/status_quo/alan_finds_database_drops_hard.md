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


Alan tries to figure out what happened from the logs, but is unable to do so. Alan decides to attempt to recreate the scenario by doing extra load testing and trying to emulate the increased traffic. After some time, Alan gets lucky and is able to recreate the error, but he still doesn't know what could be the cause. Alan turns to the documentation for the `sqlx` crate to see if there are flags that might enable extra instrumentation but he can't find any (XXX check what crate offers).

Alan, next, takes the following steps to determine the most likely source to explain the problem behavior in his application:

1. Check the `sqlx` documentation for possible flags to explicitly close the connection but none are found.
2. Examine the source code to figure out where the connection gets closed.
3. Discover that the connection is closed through the destructor, Drop::drop only when it gets cleaned up aftering going out of scope.

Alan realizes he must seek help from someone more experienced and knowledgable. He goes to his friend Barbara, and asks, "How can I get a log statement when the destructor runs?". Barbara tells him to wrap his database handle in another struct which has a destructor and to insert a log into the destructor for that struct we'll call `Foo`. Alan implements `Foo` and sees the new log when the destructor runs in the output. Now that the log works Alan runs program with full load in order to reproduce the problem. Alan analyzes the logs and notices that the new connection is being created and panicing before the destructor has run.

Next, Alan seeks advice from the `sqlx` forum. He learns that the DropImpl for new connections spawns a task in order to close a handle, and therefore, it may not execute right away. His advisor states that Rust doesn't have a way to execute async operations in a destructor.

Alan evalutes possible fixes to this problem. His first consideration is whether there is an explicit async method that will close the connection. He then realizes that when a user disconnects and the active future for that user is dropped, his call wouldn't run anyway. 

```rust=
let handle = ...
logic(handle).await; // if user disconnects here, call to `handle.close()` will never execute
handle.close();
```

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
