# 😱 Status quo stories: Alan finds dropping database handles is hard.

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!


## The problem

Alan has just written a function that extends YouBuy to connect to a database when his application receives a request. The request returns all the data for a single user to a customized page on his website. After launching the beta version of his application, Alan begins receiving complaints that his website sometimes displays a page with an error message and other times it seems to work as intended. When the page with an error message appears the message reads: "Can not open a connection to the database, because a connection is already open".

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

Alan sees the reports coming in from his beta users, but hasn't received any alarms from his application. He realizes he must start debugging. His initial testing produces the customized page for the current user as expected, but after a number of attempts he sees the page with the error message. His application shows an error that says that there is already an open database handle.


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
