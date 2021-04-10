# ✨ Shiny future stories: Barbara makes a wish

## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is a speculative sketch of what life might be like in 2 to 3 years, if things go well.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' dreams, shiny future stories [cannot be wrong], only unclear or over-ambitious). Alternatively, you may wish to [add your own shiny future story][htvsf]!

## The story

[Barbara] has an initial prototype of a new service she wrote in sync Rust. She then decides, since the service is extremely I/O bound, to port it to async Rust and her benchmarks have led her to believe that performance is being left on the table.

She does this by sprinkling `async/.await` everywhere, picking an executor, and moving dependencies from sync to async.

Once she has the program compiling, she thinks "oh that was easy". She runs it for the first time and surprisingly she finds out that when hitting an endpoint, nothing happens.

Barbara, always prepared, has already added logging to her service and she checks the logs. As she expected, she sees here that the endpoint handler has been invoked but then... nothing. Barbara exclaims, "Oh no! This was not what I was expecting, but let's dig deeper." 

She checks the code and sees that the endpoint spawns several tasks, but unfortunately those tasks don't have much logging in them.

Barbara now remembers hearing something about a `wish4-async-insight` crate, which has gotten some buzz on her Rust-related social media channels. She decides to give that a shot.

She adds the crate as a dependency to her `Cargo.toml`, renaming it to just `insight` to make it easier to reference in her code, and then initializes it in her main async function.

```rust,ignore
async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    insight::init(); // new code
    ...
}
```

Barbara rebuilds and runs her program again. She doesn't see anything different in the terminal output for the program itself though, and the behavior is the same as before: hitting an endpoint, nothing happens. She double-checks the readme for the `async-executor-insight` crate, and realizes that she needs to connect other programs to her service to observe the insights being gathered. Barbara decides that she wants to customize the port that `insight` is listening on before she starts her experiments with those programs.

```rust,ignore
async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    insight::init(listen_port => 8080); // new code, leveraging keyword arguments feature added in 2024
    ...
}
```

While her code rebuilds, Barbara investigates what programs she might use to connect to the insight crate.

One such program, `consolation`, can run in the terminal. Barbara is currently just deploying her service locally on her development box, so she opts to try that out and see what it tells her.

```
% rustup install wish4-consolation
...
% consolation --port 8080
```

This brings up a terminal window that looks similar to the Unix `top` program, except that instead of a list of OS processes, this offers a list of tasks, with each task having a type, ID, and status history (i.e. percentage of time spent in running, ready to poll, or blocked). Barbara skims the output in the list, and sees that one task is listed as currently blocked.

Barbara taps the arrow-keys and sees that this causes a cursor to highlight different tasks in the list. She highlights the blocked task and hits the Enter key. This causes the terminal to switch to a Task view, describing more details about that task and its status.

The Task view here says that the task is blocked, references a file and line number, and also includes the line from the source code, which says `chan.send(value).await`. The blocked task also lists the resources that the task is waiting on: `prototype_channel`, and next to that there is text on a dark red background: "waiting on channel capacity." Again, Barbara taps the arrow-keys and sees that she can select the line for the resource.

Barbara notices that this whole time, at the bottom of the terminal, there was a line that says "For help, hit `?` key"; she taps question mark. This brings up a help message in a scrollable subwindow explaining the task view in general as well as link to online documentation. The help message notes that the user can follow the chain: One can go from the blocked task to the resource it's waiting on, and from that resource to a list of tasks responsible for freeing up the resource.

Barbara hits the Escape key to close the help window. The highlight is still on the line that says "prototype_channel: waiting on channel capacity"; Barbara hits Enter, and this brings up a list with just one task on it: The channel reader task. Barbara realizes what this is saying: The channel resource is blocking the sender because it is full, and the only way that can be resolved is if the channel reader manages to receive some inputs from the channel.

Barbara opens the help window again, and brings up the link to the online documentation. There, she sees discussion of resource starvation and the specific case of a bounded channel being filled up before its receiver makes progress. The main responses outlined there are 1. decrease the send rate, 2. increase the receive rate, or 3. increase the channel's internal capacity, noting the extreme approach of changing to an unbounded channel (with the caveat that this risks resource exhaustion).

Barbara skims the task view for the channel reader, since she wants to determine why it is not making progress. However, she is eager to see if her service as a whole is workable apart from this issue, so she also adopts the quick fix of swapping in an unbounded channel. Barbara is betting that if this works, she can use the data from `wish4-async-insight` about the channel sizes to put a bounded channel with an appropriate size in later.

Barbara happily moves along to some initial performance analysis of her "working" code, eager to see what other things `wish4-async-insight` will reveal during her explorations.

### Alternate History

*The original status quo story just said that Barbara's problem was resolved (sort of) by switching to an unbounded channel. I, much like Barbara, could not tell *why* this resolved her problem. In particular, I could not tell whether there was an outright deadlock due to a cycle in the task-resource dependency chain that, or if there something more subtle happening. In the story above, I assumed it was the second case: something subtle.*

*Here's an important alternate history though, for the first case of a cycle. Its ...* the same story, right up to when Barbara first runs `consolation`:

```
% rustup install wish4-consolation
...
% consolation --port 8080
```

This brings up a terminal window that looks similar to the Unix `top` program, except that instead of a list of OS processes, this offers a list of tasks, and shows their status (i.e. running, ready to poll, or blocked), as well as some metrics about how long the tasks spend in each state.

At the top of the screen, Barbara sees highlighted warning: "deadlock cycle was detected. hit `P` for more info."

Barbara types capital `P`. The terminal switches to "problem view," which shows

 * The task types, ID, and attributes for each type.
 * The resources being awaited on
 * The location / backtrace of the await.
 * A link to a documentation page expanding on the issue.

The screen also says "hit `D` to generate a graphviz `.dot` file to disk describing the cycle."

Barbara hits `D` and stares at the resulting graph.

Barbara suddenly realizes her mistake: She had constructed a single task that was sometimes enqueuing work (by sending messages on the channel), and sometimes dequeuing work, but she had not put any controls into place to ensure that the dequeuing (via `recv`) would get prioritized as the channel filled up.

Barbara reflects on the matter: she knows that she could swap in an unbounded channel to resolve this, but she thinks that she would be better off thinking a bit more about her system design, to see if she can figure out a way to supply back-pressure so that the send rate will go down as the channel fills up.


## 🤔 Frequently Asked Questions

### **What status quo story or stories are you retelling?**

[Barbara wants Async Insights](../status_quo/barbara_wants_async_insights.md)

### **What is [Alan] most excited about in this future? Is he disappointed by anything?**

Alan is happy to see a tool that gives one a view into the internals of the async executor.

Alan is not so thrilled about using the `consolation` terminal interface; but luckily there are other options, namely a web-browser based client that offers even richer functionality, such as renderings of the task/resource dependency graph.

### **What is [Grace] most excited about in this future? Is she disappointed by anything?**

Grace is happy to see a tool, but wonders whether it could have been integrated into `gdb`.

Grace is not so thrilled to learn that this tool is not going to try to provide specific insight into performance issues that arise solely from computational overheads in her own code. (The readme for `wish4-async-insight` says on this matter "for that, use perf," which Grace finds unsatisfying.)

### **What is [Niklaus] most excited about in this future? Is he disappointed by anything?**

Niklaus is happy to learn that the `wish4-async-insight` is supported by both `async-std` and `tokio`, since he relies on friends in both communities to help him learn more about Async Rust.

Niklaus is happy about the tool's core presentation oriented around abstractions he understands (tasks and resources). Niklaus is also happy about the integrated help.

However, Niklaus is a little nervous about some of the details in the output that he doesn't understand.

### **What is [Barbara] most excited about in this future? Is she disappointed by anything?**

Barbara is thrilled with

*Think about Barbara's top priority (productivity, maintenance over time) and the expectations she brings (fits well with Rust). How do they fare in this future?*

### **What [projects] benefit the most from this future?**

Any async codebase that can hook into the `wish4-async-insight` crate and supply its data via a network port during development would benefit from this. So, I suspect any codebase that uses a sufficiently popular (i.e. appropriately instrumented) async executor will benefit.

The main exception I can imagine right now is [MonsterMesh][]: its resource constraints and `#![no_std]` environment  are almost certainly incompatible with the needs of the `wish4-async-insight` crate.

### **Are there any [projects] that are hindered by this future?**

The only "hindrance" is that the there is an expectation that the async-executor be instrumented appropriately to feed its data to the `wish4-async-insight` crate once it is initialized.

### **What are the incremental steps towards realizing this shiny future?** (Optional)

 * Get tracing crate to 1.0 so that async executors can rely on it.

 * Prototype an insight console atop a concrete async executor (e.g. `tokio`)

 * Develop a shared protocol atop `tracing` that all async executors will use to provide the insightful data.

### **Does realizing this future require cooperation between many projects?** (Optional)

Yes. Yes it does.

At the very least, as mentioned among the "incremental steps", we will need a common protocol that the async executors use to communicate their internal state.


[character]: ../characters.md
[comment]: ./comment.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsf]: ../how_to_vision/shiny_future.md
[projects]: ../projects.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
[MonsterMesh]: ../projects/MonsterMesh.md
