# 😱 Status quo stories: Barbara carefully dismisses embedded `Future`

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[Barbara] is contributing to an [OS](https://www.tockos.org) that supports
running multiple applications on a single microcontroller. These
microcontrollers have as little as 10's of kilobytes of RAM and 100's of
kilobytes of flash memory for code. Barbara is writing a library that is used by
multiple applications -- and is linked into each application -- so the library
is very resource constrained. The library should support asynchronous operation,
so that multiple APIs can be used in parallel within each (single-threaded)
application.

Barbara begins writing the library by trying to write a console interface, which
allows byte sequences to be printed to the system console. Here is an example
sequence of events for a console print:

1. The interface gives the kernel a callback to call when the print finishes,
   and gives the kernel the buffer to print.
1. The kernel prints the buffer in the background while the app is free to do
   other things.
1. The print finishes.
1. The app tells the kernel it is ready for the callback to be invoked, and the
   kernel invokes the callback.

Barbara tries to implement the API using
[`core::future::Future`](https://doc.rust-lang.org/stable/core/future/trait.Future.html)
so that the library can be compatible with the async Rust ecosystem. To do
this, she decides to make the buffer printing API return a Future. She starts
with a skeleton:

```rust
/// Passes `buffer` to the kernel, and prints it to the console. Returns a
/// future that returns `buffer` when the print is complete. The caller must
/// call kernel_ready_for_callbacks() when it is ready for the future to return. 
fn print_buffer(buffer: &'static mut [u8]) -> PrintFuture {
    // TODO: Set the callback
    // TODO: Tell the kernel to print `buffer`
}

struct PrintFuture;

impl core::future::Future for PrintFuture {
    type Output = &'static mut [u8];

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // TODO: Detect when the print is done, retrieve `buffer`, and return
        // it.
    }
}
```

Note: All error handling is omitted to keep things understandable.

Barbara begins to implement `print_buffer`:

```rust
fn print_buffer(buffer: &'static mut [u8]) -> PrintFuture {
    kernel_set_print_callback(callback);
    kernel_start_print(buffer);
    PrintFuture {}
}

// New! The callback the kernel calls.
extern fn callback() {
    // TODO: Wake up the currently-waiting PrintFuture.
}
```

So far so good. Barbara then works on `poll`:

```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if kernel_is_print_done() {
            Poll::Ready(kernel_get_buffer_back())
        }
        Poll::Pending
    }
```

Of course, there's something missing here. How does the callback wake the
`PrintFuture`? She needs to store the
[`Waker`](https://doc.rust-lang.org/stable/core/task/struct.Waker.html)
somewhere! Barbara puts the `Waker` in a global variable so the callback can
find it (this is fine because the app is single threaded and callbacks do NOT
interrupt execution the way Unix signals do):

```rust
static mut PRINT_WAKER: Option<Waker> = None;

extern fn callback() {
    if let Some(waker) = unsafe { PRINT_WAKER.as_ref() } {
        waker.wake_by_ref();
    }
}
```

She then modifies `poll` to set `PRINT_WAKER`:

```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if kernel_is_print_done() {
            return Poll::Ready(kernel_get_buffer_back());
        }
        unsafe { PRINT_WAKER = Some(cx.waker()); }
        Poll::Pending
    }
```

At this point, Barbara starts to think about the size impact of these changes.
`PRINT_WAKER` is stored in `.bss`, which occupies space in RAM but not flash. It
is two words in size. It points to a
[`RawWakerVTable`](https://doc.rust-lang.org/stable/core/task/struct.RawWakerVTable.html)
that is provided by the executor. Considering that this code is designed to run
without `alloc`, `RawWakerVTable` seems awfully wasteful. `drop` is a no-op,
`clone` seems like it will be a no-op, and `wake`/`wake_by_ref` seem like
duplicates.

At this point, Barbara thought about the overhead and decided it would be worth
it if it made it easy to work with future combinators from external crates, and
could function as a lightweight form of multithreading. To support this, Barbara
decides to implement an executor designed to function like a background thread.
This executor is started using a `spawn` function, and otherwise runs entirely
in kernel callbacks.

Barbara identifies several factors that add branching and error handling code to
the executor:

1. `spawn` should be a safe function, because it is called by high-level
   application code. However, that means it can be called by the future it
   contains. If handled naively, this would result in dropping the future while
   it executes. Barbara adds runtime checks to identify this situation.
1. `Waker` is `Sync`, so on a multithreaded system, a future could give another
   thread access to its `Waker` and the other thread could wake it up. This
   could happen while the `poll` is executing, before `poll` returns
   `Poll::Pending`. Therefore, Barbara concludes that if `wake` is called while
   a future is being polled then the future should be re-polled, even if the
   current `poll` returns `Poll::Pending`. This requires putting a retry loop
   into the executor.
1. A kernel callback may call `Waker::wake` after its future finishes executing,
   and the executor must ignore those wakeups. This duplicates the "ignore
   spurious wakeups" functionality that exists in the future itself.

Ultimately, this made the executor logic [quite
nontrivial](https://github.com/tock/design-explorations/blob/master/size_comparison/futures/src/task.rs).
The executor logic can be monomorphized for each future, which allows the
compiler to make inlining optimizations, but results in a significant amount of
duplicate code. Alternatively, it could be adapted to use function pointers or
vtables to avoid the code duplication, but then the compiler *definitely* cannot
inline `Future::poll` into the kernel callbacks.

At this point, Barbara realizes she needs to do some benchmarking. Barbara comes
up with a sample application -- an app that blinks a led and responds to button
presses -- and implements it twice. One implementation does not use `Future` at
all, the other does. In addition to two async drivers (a timer driver and a GPIO
driver), the `Future`-based app uses Barbara's executor as well as a single
combinator (that manages the state of the app).

Barbara publishes an
[analysis](https://github.com/tock/design-explorations/tree/master/size_comparison)
of the relative sizes of the two app implementations, finding a large percentage
increase in both code size and RAM usage (note: stack usage was not
investigated). Barbara concludes that **`core::future::Future` is not suitable
for highly-resource-constrained environments** due to the amount of code and RAM
required to implement executors and combinators.

Barbara redesigns the library she is building to use a [different
concept](https://github.com/tock/design-explorations/tree/master/zst_pointer_async)
for implementing async APIs in Rust that are much lighter weight. She has moved
on from `Future` and is refining her [async
traits](https://github.com/tock/libtock-rs/blob/master/core/platform/src/async_traits.rs)
instead.

## 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * `core::future::Future` isn't suitable for every asynchronous API in Rust.
      `Future` has a lot of capabilities, such as the ability to spawn
      dynamically-allocated futures, that are unnecessary in embedded system.
    * We should look at embedded Rust's relationship with `Future` so we don't
      fragment the embedded Rust ecosystem. Other embedded crates use `Future`,
      presumably because they are less space constrained than Barbara's
      codebase.
* **Why did you choose *Barbara* to tell this story?**
    * This story is about someone who is an experienced systems programmer and
      an experienced Rust developer. All the other characters have "new to Rust"
      or "new to programming" as a key characteristic.
 **How would this story have played out differently for the other characters?**
    * [Alan] would have found the `#![no_std]` crate ecosystem lacking async
      support. He would have moved forward with a `Future`-based implementation,
      unaware of its impact on code size and RAM usage.
    * [Grace] would have handled the issue similarly to Barbara, but may not
      have tried as hard to use `Future`. Barbara has been paying attention to
      Rust long enough to know how significant the `Future` trait is in the Rust
      community and ecosystem.
    * [Niklaus] would really have struggled. If he asked for help, he probably
      would've gotten conflicting advice from the community.

[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
