# Status quo: Grace waits for gdb next

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Grace wants to walk through the behavior of a toy program.

She first fires up `cargo run --verbose` to remind herself what the path to the target binary is. Part of the resulting Cargo output is:

```ignore
     Running `target/debug/toy`
```

From that, Grace tries running `gdb` on the printed path.

```ignore
    gdb target/debug/toy
```

and then

```ignore
(gdb) start
```

to start the program and set a breakpoint on the `main` function.

Grace hits Ctrl-x a and gets a TUI mode view that includes this:

```ignore
│   52          }                                                                                                                                                                                                                    │
│   53                                                                                                                                                                                                                               │
│   54          #[tokio::main]                                                                                                                                                                                                       │
│B+>55          pub(crate) async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {                                                                                                                                   │
│   56              println!("Hello, world!");                                                                                                                                                                                       │
│   57              let record = Box::new(Mutex::new(Record::new()));                                                                                                                                                                │
│   58              let record = &*Box::leak(record);                                                                                                                                                                                │
│   59                                                                                                                                                                                                                              
```

Excitedly Grace types `next` to continue to the next line of the function.

And waits. And the program does not stop anywhere.

...

Eventually Grace remembers that `#[tokio::main]` injects a *different* main function that isn't the one that she wrote as an `async fn`, and so the `next` operation in `gdb` isn't going to set a breakpoint within Grace's `async fn main`.

So Grace restarts the debugger, and then asks for a breakpoint on the first line of her function:

```ignore
(gdb) start
(gdb) break 56
(gdb) continue
```

And now it stops on the line that she expected:

```                                                                                                                                                                                                     │
│   53                                                                                                                                                                                                                               │
│   54          #[tokio::main]                                                                                                                                                                                                       │
│   55          pub(crate) async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {                                                                                                                                   │
│B+>56              println!("Hello, world!");                                                                                                                                                                                       │
│   57              let record = Box::new(Mutex::new(Record::new()));                                                                                                                                                                │
│   58              let record = &*Box::leak(record);                                                                                                                                                                                │
│   59                                                                                                                                                                                                                               │
│   60              let (tx, mut rx) = channel(100);                                                                                                                                                                                 │
```

Grace is now able to use `next` to walk through the main function. She does notice that the calls to `tokio::spawn` are skipped over by `next`, but that's not as much of a surprise to her, since those are indeed function calls that are taking async blocks. She sets breakpoints on the first line of each async block so that the debugger will stop when control reaches them as she steps through the code.


## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**
* A common usage pattern: hitting `next` to go to what seems like the next statement, breaks down due to implementation details of `#[tokio::main]` and `async fn`.
* This is one example of where `next` breaks, in terms of what a user is likely to *want*. The other common scenario where the behavior of `next` is non-ideal is higher-order functions, like `option.and_then(|t| { ... }`, where someone stepping through the code probably *wants* `next` to set
  a temporary breakpoint in the `...` of the closure.

### **What are the sources for this story?**
Personal experience. I haven't acquired the muscle memory to stop using `next`, even though it breaks down in such cases.

### **Why did you choose Grace to tell this story?**
I needed someone who, like me, would actually be tempted to use `gdb` even when println debugging is so popular.

### **How would this story have played out differently for the other characters?**
    * Alan might have used whatever debugger is offered by his IDE, which might have the same problem (via a toolbar button that has the same semantics as `next`); but many people using IDE's to debugger just naturally set breakpoints by hand on the lines in their IDE editor, and thus will not run into this.
    * Most characters would probably have abandoned using gdb much sooner. E.g. Grace may have started out by adding `println` or `tracing` instrumentation to the code, rather than trying to open it up in a debugger.


[character]: ../../characters.md
[status quo stories]: ../status_quo.md
[Alan]: ../../characters/alan.md
[Grace]: ../../characters/grace.md
[Niklaus]: ../../characters/niklaus.md
[Barbara]: ../../characters/barbara.md
[htvsq]: ../status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
