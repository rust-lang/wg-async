# Barbara wants to use GhostCell-like cell borrowing with futures

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the
brainstorming period. It is derived from real-life experiences of
actual Rust users and is meant to reflect some of the challenges that
Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to
the FAQ, feel free to open a PR making edits (but keep in mind that,
as they reflect peoples' experiences, status quo stories [cannot be
wrong], only inaccurate). Alternatively, you may wish to [add your own
status quo story][htvsq]!

## The story

Barbara quite likes using statically-checked cell borrowing.  "Cell"
in Rust terminology refers to types like `Cell` or `RefCell` that
enable interior mutability, i.e. modifying or mutably borrowing stuff
even if you've only got an immutable reference to it.
Statically-checked cell borrowing is a technique whereby one object
(an "owner") acts as a gatekeeper for borrow-access to a set of other
objects ("cells").  So if you have mutable borrow access to the owner,
you can temporarily transfer that mutable borrow access to a cell in
order to modify it.  This is all checked at compile-time, hence
"statically-checked".

In comparison `RefCell` does borrow-checking, but it is checked at
runtime and it will panic if you make a coding mistake.  The advantage
of statically-checked borrowing is that it cannot panic at runtime,
i.e. all your borrowing bugs show up at compile time.  The history
goes way back, and the technique has been reinvented at least 2-3
times as far as Barbara is aware.  This is implemented in various
forms in [GhostCell](http://plv.mpi-sws.org/rustbelt/ghostcell/) and
[`qcell`](https://docs.rs/qcell/0.4.1/qcell/).

Barbara would like to use statically-checked cell borrowing within
futures, but there is no way to get the owner borrow through the
`Future::poll` call, i.e. there is no argument or object that the
runtime could save the borrow in.  Mostly this does not cause a
problem, because there are other ways for a runtime to share data,
e.g. data can be incorporated into the future when it is created.
However in this specific case, for the specific technique of
statically-checked cell borrows, we need an active borrow to the owner
to be passed down the call stack through all the poll calls.

So Barbara is forced to use `RefCell` instead and be very careful not
to cause panics.  This seems like a step back.  It feels dangerous to
use `RefCell` and to have to manually verify that her cell borrows are
panic-free.

There are good habits that you can adopt to offset the dangers, of
course.  If you are very careful to make sure that you call no other
method or function which might in turn call code which might attempt
to get another borrow on the same cell, then the `RefCell::borrow_mut`
panics can be avoided.  However this is easy to overlook, and it is
easy to fail to anticipate what indirect calls will be made by a given
call, and of course this may change later on due to maintenance and
new features.  A borrow may stay active longer than expected, so calls
which appear safe might actually panic.  Sometimes it's necessary to
manually drop the borrow to be sure.  In addition you'll never know
what indirect calls might be made until all the possible code-paths
have been explored, either through testing or through running in
production.

So Barbara prefers to avoid all these problems, and use
statically-checked cell borrowing where possible.


### Example 1: Accessing an object shared outside the runtime

In this minimized example of code to interface a stream to code
outside of the async/await system, the buffer has to be accessible
from both the stream and the outside code, so it is handled as a
`Rc<RefCell<StreamBuffer<T>>>`.

```rust
pub struct StreamPipe<T> {
    buf: Rc<RefCell<StreamBuffer<T>>>,
}

impl<T> Stream for StreamPipe<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<T>> {
        let mut buf = self.buf.borrow_mut();
        if let Some(item) = buf.value.take() {
            return Poll::Ready(Some(item));
        }
        if buf.end {
            return Poll::Ready(None);
        }
        self.req_more();  // Callback to request more data
        Poll::Pending
    }
}
```

Probably `req_more()` has to schedule some background operation, but
if it doesn't and attempts to modify the shared `buf` immediately then
we get a panic, because `buf` is still borrowed.  The real life code
could be a lot more complicated, and the required combination of
conditions might be harder to hit in testing.

With statically-checked borrowing, the borrow would be something like
`let mut buf = self.buf.rw(cx);`, and the `req_more` call would either
have to take the `cx` as an argument (forcing the previous borrow to
end) or would not take `cx`, meaning that it would always have to
defer the access to the buffer to other code, because without the `cx`
there is no possible way to access the buffer.


### Example 2: Shared monitoring data

In this example, the app keeps tallies of various things in a
`Monitor` structure.  This might be data in/out, number of errors
detected, maybe a hashmap of current links, etc.  Since it is accessed
from various components, it is kept behind an `Rc<RefCell<_>>`.

```rust
// Dependency: futures-lite = "1.11.3"
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let monitor0 = Rc::new(RefCell::new(Monitor { count: 0 }));
    let monitor1 = monitor0.clone();

    let fut0 = async move {
        let mut borrow = monitor0.borrow_mut();
        borrow.count += 1;
    };

    let fut1 = async move {
        let mut borrow = monitor1.borrow_mut();
        borrow.count += 1;
        fut0.await;
    };

    futures_lite::future::block_on(fut1);
}

struct Monitor {
    count: usize,
}
```

The problem is that this panics with a borrowing error because the
borrow is still active when the `fut0.await` executes and attempts
another borrow.  The solution is to remember to drop the borrow before
awaiting.

In this example code the bug is obvious, but in real life maybe `fut0`
only borrows in rare situations, e.g. when an error is detected.  Or
maybe the future that borrows is several calls away down the
callstack.

With statically-checked borrowing, there is a slight problem in that
currently there is no way to access the poll context from `async {}`
code.  But if there was then the borrow would be something like `let
mut borrow = monitor1.rw(cx);`, and since the `fut0.await` implicitly
requires the `cx` in order to poll, the borrow would be forced to end
at that point.


## Further investigation by Barbara

### The mechanism

Barbara understands that statically-checked cell borrows work by
having an owner held by the runtime, and various instances of a cell
held by things running on top of the runtime (these cells would
typically be behind `Rc` references).  A mutable borrow on the owner
is passed down the stack, which enables safe borrows on all the cells,
since a mutable borrow on a cell is enabled by temporarily holding
onto the mutable borrow of the owner, which is all checked at
compile-time.

So the mutable owner borrow needs to be passed through the `poll`
call, and Barbara realizes that this would require support from the
standard library.

Right now a `&mut Context<'_>` is passed to `poll`, and so within
`Context` would be the ideal place to hold a borrow on the cell owner.
However as far as Barbara can see there are difficulties with all the
current implementations:

- GhostCell (or qcell::LCell) may be the best available solution,
  because it doesn't have any restrictions on how many runtimes might
  be running or how they might be nested.  But Rust insists that the
  lifetimes `<'id>` on methods and types are explicit, so it seems
  like that would force a change to the signature of `poll`, which
  would break the ecosystem.

  Here Barbara experiments with a working example of a modified Future
  trait and a future implementation that makes use of LCell:

```rust
// Requires dependency: qcell = "0.4"
use qcell::{LCell, LCellOwner};
use std::pin::Pin;
use std::rc::Rc;
use std::task::Poll;

struct Context<'id, 'a> {
    cell_owner: &'a mut LCellOwner<'id>,
}

struct AsyncCell<'id, T>(LCell<'id, T>);
impl<'id, T> AsyncCell<'id, T> {
    pub fn new(value: T) -> Self {
        Self(LCell::new(value))
    }
    pub fn rw<'a, 'b: 'a>(&'a self, cx: &'a mut Context<'id, 'b>) -> &'a mut T {
        cx.cell_owner.rw(&self.0)
    }
}

trait Future<'id> {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'id, '_>) -> Poll<Self::Output>;
}

struct MyFuture<'id> {
    count: Rc<AsyncCell<'id, usize>>,
}
impl<'id> Future<'id> for MyFuture<'id> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'id, '_>) -> Poll<Self::Output> {
        *self.count.rw(cx) += 1;
        Poll::Ready(())
    }
}

fn main() {
    LCellOwner::scope(|mut owner| {
        let mut cx = Context { cell_owner: &mut owner };
        let count = Rc::new(AsyncCell::new(0_usize));
        let mut fut = Box::pin(MyFuture { count: count.clone() });
        let _ = fut.as_mut().poll(&mut cx);
        assert_eq!(1, *count.rw(&mut cx));
    });
}
```

- The other `qcell` types (QCell, TCell and TLCell) have various
  restrictions or overheads which might make them unsuitable as a
  general-purpose solution in the standard library.  However they do
  have the positive feature of not requiring any change in the
  signature of `poll`.  It looks like they could be added to `Context`
  without breaking anything.

  Here Barbara tries using `TLCell`, and finds that the signature of
  `poll` doesn't need to change:

```rust
// Requires dependency: qcell = "0.4"
use qcell::{TLCell, TLCellOwner};
use std::pin::Pin;
use std::rc::Rc;
use std::task::Poll;

struct AsyncMarker;
struct Context<'a> {
    cell_owner: &'a mut TLCellOwner<AsyncMarker>,
}

struct AsyncCell<T>(TLCell<AsyncMarker, T>);
impl<T> AsyncCell<T> {
    pub fn new(value: T) -> Self {
        Self(TLCell::new(value))
    }
    pub fn rw<'a, 'b: 'a>(&'a self, cx: &'a mut Context<'b>) -> &'a mut T {
        cx.cell_owner.rw(&self.0)
    }
}

trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

struct MyFuture {
    count: Rc<AsyncCell<usize>>,
}
impl Future for MyFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        *self.count.rw(cx) += 1;
        Poll::Ready(())
    }
}

fn main() {
    let mut owner = TLCellOwner::new();
    let mut cx = Context { cell_owner: &mut owner };
    let count = Rc::new(AsyncCell::new(0_usize));
    let mut fut = Box::pin(MyFuture { count: count.clone() });
    let _ = fut.as_mut().poll(&mut cx);
    assert_eq!(1, *count.rw(&mut cx));
}
```

  (For comparison, `TCell` only allows one owner per marker type in
  the whole process.  `QCell` allows many owners, but requires a
  runtime check to make sure you're using the right owner to access a
  cell.  `TLCell` allows only one owner per thread per marker type,
  but also lets cells migrate between threads and be borrowed locally,
  which the others don't -- see [qcell
  docs](https://docs.rs/qcell/0.4.1/qcell/).)

So the choice is GhostCell/LCell and lifetimes everywhere, or various
other cell types that may be too restrictive.

Right now Barbara thinks that none of these solutions is likely to be
acceptable for the standard library.  However still it is a desirable
feature, so maybe someone can think of a way around the problems.  Or
maybe someone has a different perspective on what would be acceptable.

### Proof of concept

The [Stakker](https://crates.io/crates/stakker) runtime makes use of
qcell-based statically-checked cell borrowing.  It uses this to get
zero-cost access to actors, guaranteeing at compile time that no actor
can access any other actor's state.  It also uses it to allow
inter-actor [shared
state](https://docs.rs/stakker/0.2.1/stakker/struct.Share.html) to be
accessed safely and zero-cost, without RefCell.

(For example within a Stakker actor, you can access the contents of a
`Share<T>` via the actor context `cx` as follows: `share.rw(cx)`,
which blocks borrowing or accessing `cx` until that borrow on `share`
has been released.  `Share<T>` is effectively a `Rc<ShareCell<T>` and
`cx` has access to an active borrow on the `ShareCellOwner`, just as
in the long examples above.)

Stakker doesn't use GhostCell (LCell) because of the need for `<'id>`
annotations on methods and types.  Instead it uses the other three
cell types according to how many Stakker instances will be run, either
one Stakker instance only, one per thread, or multiple per thread.
This is selected by cargo features.

Switching implementations like this doesn't seem like an option for
the standard library.

### Way forward

Barbara wonders whether there is any way this can be made to work.
For example, could the compiler derive all those `<'id>` annotations
automatically for GhostCell/LCell?

Or for multi-threaded runtimes, would
[`qcell::TLCell`](https://docs.rs/qcell/0.4.1/qcell/) be acceptable?
This allows a single cell-owner in every thread.  So it would not
allow nested runtimes of the same type.  However it does allow borrows
to happen at the same time independently in different threads, and it
also allows the migration of cells between threads, which is safe
because that kind of cell isn't `Sync`.

Or is there some other form of cell-borrowing that could be devised
that would work better for this?

The interface between cells and `Context` should be straightforward
once a particular cell type is demonstrated to be workable with the
`poll` interface and futures ecosystem.  For example copying the API
style of Stakker:

```
let rc = Rc::new(AsyncCell::new(1_u32));
*rc.rw(cx) = 2;
```

So logically you obtain read-write access to a cell by naming the
authority by which you claim access, in this case the poll context.
In this case it really is naming rather than accessing since the
checks are done at compile time and the address that `cx` represents
doesn't actually get passed anywhere or evaluated, once inlining and
optimisation is complete.


## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

The main problem is that Barbara has got used to a safer environment
and it feels dangerous to go back to RefCell and have to manually
verify that her cell borrows are panic-free.

### **What are the sources for this story?**

The author of Stakker is trying to interface it to async/await and
futures.

### **Why did you choose Barbara to tell this story?**

Barbara has enough Rust knowledge to understand the benefits that
GhostCell/qcell-like borrowing might bring.

### **How would this story have played out differently for the other characters?**

The other characters perhaps wouldn't have heard of statically-checked
cell borrows so would be unaware of the possibility of making things
safer.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
