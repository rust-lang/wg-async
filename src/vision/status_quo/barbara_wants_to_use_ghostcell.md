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

Barbara quite likes using statically-checked cell borrowing.  This is
implemented in various ways in
[GhostCell](http://plv.mpi-sws.org/rustbelt/ghostcell/) and
[`qcell`](https://docs.rs/qcell/0.4.1/qcell/).  She would like to use
statically-checked cell borrowing within futures, but there is no way
to get the owner borrow through the `Future::poll` call.

So she is forced to use `RefCell` instead and be very careful not to
cause panics.  This seems like a step back.  It feels dangerous to use
`RefCell` and to have to manually verify that her cell borrows are
panic-free.

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

### **Why did you choose *NAME* to tell this story?**

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
