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
so that the library can be compatible with the async Rust ecosystem. The OS
kernel does not expose a Future-based interface, so Barbara has to implement
`Future` by hand rather than using async/await syntax. She starts with a
skeleton:

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
            return Poll::Ready(kernel_get_buffer_back());
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

`PRINT_WAKER` is stored in `.bss`, which occupies space in RAM but not flash. It
is two words in size. It points to a
[`RawWakerVTable`](https://doc.rust-lang.org/stable/core/task/struct.RawWakerVTable.html)
that is provided by the executor. `RawWakerVTable`'s design is a compromise that
supports environments both with and without `alloc`. In no-`alloc` environments,
`drop` and `clone` are generally no-ops, and `wake`/`wake_by_ref` seem like
duplicates. Looking at `RawWakerVTable` makes Barbara realize that even though
`Future` was designed to work in embedded contexts, it may have too much
overhead for her use case.

Barbara decides to do some benchmarking. She comes up with a sample application
-- an app that blinks a led and responds to button presses -- and implements it
twice. One implementation does not use `Future` at all, the other does. Both
implementations have two asynchronous interfaces: a timer interface and a GPIO
interface, as well as an application component that uses the interfaces
concurrently. In the `Future`-based app, the application component functions
like a future combinator, as it is a `Future` that is almost always waiting for
a timer or GPIO future to finish.

To drive the application future, Barbara implements an executor. The executor
functions like a background thread. Because `alloc` is not available, this
executor contains a single future. The executor has a `spawn` function that
accepts a future and starts running that future (overwriting the existing future
in the executor if one is already present). Once started, the executor runs
entirely in kernel callbacks.

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
1. A kernel callback may call `Waker::wake` after its future returns
   `Poll::Ready`. After `poll` returns `Poll::Ready`, the executor should not
   `poll` the future again, so Barbara adds code to ignore those wakeups. This
   duplicates the "ignore spurious wakeups" functionality that exists in the
   future itself.

Ultimately, this made the [executor
logic](https://github.com/tock/design-explorations/blob/master/size_comparison/futures/src/task.rs)
nontrivial, and it compiled into 96 bytes of code. The executor logic is
monomorphized for each future, which allows the compiler to make inlining
optimizations, but results in a significant amount of duplicate code.
Alternatively, it could be adapted to use function pointers or vtables to avoid
the code duplication, but then the compiler *definitely* cannot inline
`Future::poll` into the kernel callbacks.

Barbara publishes an
[analysis](https://github.com/tock/design-explorations/tree/master/size_comparison)
of the relative sizes of the two app implementations, finding a large percentage
increase in both code size and RAM usage (note: stack usage was not
investigated). Most of the code size increase is from the future
combinator code.

In the no-`Future` version of the app, a kernel callback causes the following:

1. The kernel callback calls the application logic's event-handling function for
   the specific event type.
2. The application handles the event.

The call in step 1 is inlined, so the compiled kernel callback consists only of
the application's event-handling logic.

In the `Future`-based version of the app, a kernel callback causes the
following:

1. The kernel callback updates some global state to indicate the event happened.
2. The kernel callback invokes `Waker::wake`.
3. `Waker::wake` calls `poll` on the application future.
4. The application future has to look at the state saved in step 1 to determine
   what event happened.
5. The application future handles the event.

LLVM is unable to devirtualize the call in step 2, so the optimizer is unable to
simplify the above steps. Steps 1-4 only exist in the future-based version of
the code, and add over 200 bytes of code (note: Barbara believes this could be
reduced to between 100 and 200 bytes at the expense of execution speed).

Barbara concludes that **`Future` is not suitable for
highly-resource-constrained environments** due to the amount of code and RAM
required to implement executors and combinators.

Barbara redesigns the library she is building to use a [different
concept](https://github.com/tock/design-explorations/tree/master/zst_pointer_async)
for implementing async APIs in Rust that are much lighter weight. She has moved
on from `Future` and is refining her [async
traits](https://github.com/tock/libtock-rs/blob/master/core/platform/src/async_traits.rs)
instead. Here are some ways in which these APIs are lighter weight than a
`Future` implementation:

1. After monomorphization, kernel callbacks directly call application code. This
   allows the application code to be inlined into the kernel callback.
2. The callback invocation is more precise: these APIs don't make spurious
   wakeups, so application code does not need to handle spurious wakeups.
3. The async traits lack an equivalent of `Waker`. Instead, all callbacks are
   expected to be `'static` (i.e. they modify global state) and passing pointers
   around is replaced by static dispatch.

## 🤔 Frequently Asked Questions

### What are the morals of the story?

* `core::future::Future` isn't suitable for every asynchronous API in Rust.
  `Future` has a lot of capabilities, such as the ability to spawn
  dynamically-allocated futures, that are unnecessary in embedded systems.
  These capabilities have a cost, which is unavoidable without
  backwards-incompatible changes to the trait.
* We should look at embedded Rust's relationship with `Future` so we don't
  fragment the embedded Rust ecosystem. Other embedded crates use `Future`
  -- `Future` certainly has a lot of advantages over lighter-weight
  alternatives, if you have the space to use it.
  
### Why did you choose *Barbara* to tell this story?

* This story is about someone who is an experienced systems programmer and
  an experienced Rust developer. All the other characters have "new to Rust"
  or "new to programming" as a key characteristic.

### How would this story have played out differently for the other characters?

* [Alan] would have found the `#![no_std]` crate ecosystem lacking async
  support. He would have moved forward with a `Future`-based implementation,
  unaware of its impact on code size and RAM usage.
* [Grace] would have handled the issue similarly to Barbara, but may not
  have tried as hard to use `Future`. Barbara has been paying attention to
  Rust long enough to know how significant the `Future` trait is in the Rust
  community and ecosystem.
* [Niklaus] would really have struggled. If he asked for help, he probably
  would've gotten conflicting advice from the community.

### `Future` has a lot of features that Barbara's traits don't have -- aren't those worth the cost?
* `Future` has many additional features that are nice-to-have:
    1. `Future` works smoothly in a multithreaded environment. Futures can
       be `Send` and/or `Sync`, and do not need to have interior mutability,
       which avoids the need for internal locking.
       * Manipulating arbitrary Rust types without locking allows `async fn`
         to be efficient.
    1. Futures can be spawned and dropped in a dynamic manner: an executor
       that supports dynamic allocation can manage an arbitrary number of
       futures at runtime, and futures may easily be dropped to stop their
       execution.
       * Dropping a future will also drop futures it owns, conveniently
         providing good cancellation semantics.
       * A future that creates other futures (e.g. an `async fn` that calls
         other `async fn`s) can be spawned with only a single memory
         allocation, whereas callback-based approaches need to allocate for
         each asynchronous component.
    1. Community and ecosystem support. This isn't a feature of `Future` per
       se, but the Rust language has special support for `Future`
       (`async`/`await`) and practically the entire async Rust ecosystem is
       based on `Future`. The ability to use existing async crates is a very
       strong reason to use `Future` over any alternative async abstraction.
* However, the code size impact of `Future` is a deal-breaker, and no number
  of nice-to-have features can outweigh a deal-breaker. Barbara's traits
  have every feature she *needs*.
* Using `Future` saves developer time relative to building your own async
  abstractions. Developers can use the time they saved to minimize code size
  elsewhere in the project. In some cases, this may result in a net decrease
  in code size for the same total effort. However, code size reduction
  efforts have diminishing returns, so projects that expect to optimize code
  size regardless likely won't find the tradeoff beneficial.
  
### Is the code size impact of `Future` fundamental, or can the design be tweaked in a way that eliminates the tradeoff?

* `Future` isolates the code that determines a future should wake up (the
  code that calls `Waker::wake`) from the code that executes the future (the
  executor). The only information transferred via `Waker::wake` is "try
  waking up now" -- any other information has to be stored somewhere. When
  polled, a future has to run logic to identify how it can make progress --
  in many cases this requires answering "who woke me up?" -- and retrieve
  the stored information. Most completion-driven async APIs allow
  information about the event to be transferred directly to the code that
  handles the event. According to Barbara's analysis, the code required to
  determine what event happened was the majority of the size impact of
  `Future`.
  
### I thought `Future` was a zero-cost abstraction?

* Aaron Turon [described futures as zero-cost
  abstractions](https://aturon.github.io/blog/2016/08/11/futures/#zero-cost).
  In the linked post, he elaborated on what he meant by zero-cost
  abstraction, and eliminating their impact on code size was not part of
  that definition. Since then, the statement that future is a zero-cost
  abstraction has been repeated many times, mostly without the context that
  Aaron provided. Rust has many zero-cost abstractions, most of which do not
  impact code size (assuming optimization is enabled), so it is easy for
  developers to see "futures are zero-cost" and assume that makes them
  lighter-weight than they are.
  
### How does Barbara's code handle thread-safety? Is her executor unsound?

* The library Barbara is writing only works in Tock OS' userspace
  environment. This environment is single-threaded: the runtime does not
  provide a way to spawn another thread, hardware interrupts do not execute
  in userspace, and there are no interrupt-style callbacks like Unix
  signals. All kernel callbacks are invoked synchronously, using a method
  that is functionally equivalent to a function call.

[Alan]: ../../characters/alan.md
[Grace]: ../../characters/grace.md
[Niklaus]: ../../characters/niklaus.md
[Barbara]: ../../characters/barbara.md
[htvsq]: ../status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
