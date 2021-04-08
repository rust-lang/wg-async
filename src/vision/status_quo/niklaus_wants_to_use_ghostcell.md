# Niklaus wants to use GhostCell-like cell borrowing with futures

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

Niklaus quite likes using statically-checked cell borrowing.  This is
implemented in various ways in
[GhostCell](http://plv.mpi-sws.org/rustbelt/ghostcell/) and
[`qcell`](https://docs.rs/qcell/0.4.1/qcell/).  He would like to use
statically-checked cell borrowing within futures, but there is no way
to get the owner borrow through the `Future::poll` call.

So he is forced to use `RefCell` instead and be very careful not to
cause panics.  This seems like a step back.  It feels dangerous to use
`RefCell` and to have to manually-verify that his cell borrows are
panic-free.

### The mechanism

Statically-checked cell borrows work by having an owner held by the
runtime and various instances of a cell held by things running on top
of the runtime (these cells would typically be behind `Rc`
references).  A mutable borrow on the owner is passed down the stack,
which enables safe borrows on all the cells, since a mutable borrow on
a cell is enabled by temporarily holding onto the mutable borrow of
the owner.  This is all checked at compile-time.

So the mutable borrow needs to be passed through the `poll` call, and
it seems that requires support from the standard library.

Right now a `&mut Context<'_>` is passed, and so within `Context`
would be the ideal place to hold a borrow on the cell owner.  However
as far as Niklaus can see there are difficulties with all the current
implementations:

- GhostCell (or qcell::LCell) is the best available solution, because
  it doesn't have any restrictions on how many runtimes might be
  running or whatever.  But Rust insists that the lifetimes `<'id>` on
  methods are explicit, so it seems like that would force a change to
  the signature of `poll`, which would break the ecosystem.

- The other `qcell` types (QCell, TCell and TLCell) have various
  restrictions or overheads which might make them unsuitable as a
  general-purpose solution in the standard library.  However they do
  have the positive feature of not requiring any change in the
  signature of `poll`.  It looks like they could be added to `Context`
  without breaking anything.

So right now Niklaus thinks there isn't an easy way for this to be
done, but still it is a desirable feature so maybe someone can think
of a way around the problems.

### Proof of concept

The [Stakker](https://crates.io/crates/stakker) runtime makes use of
qcell-based statically-checked cell borrowing.  It uses this to get
zero-cost access to actors, guaranteeing at compile time that no
actor can access any other actor's state.  It also uses it to allow
inter-actor [shared
state](https://docs.rs/stakker/0.2.1/stakker/struct.Share.html) to be
accessed safely and zero-cost, without RefCell.

Stakker doesn't use GhostCell (LCell) because of the need for `<'id>`
annotations on methods.  Instead it uses the other three cell types
according to how many Stakker instances will be run, either one
Stakker only, one per thread, or multiple per thread.  This is
selected by cargo features.

Switching implementations like this doesn't seem like an option for
the standard library.

### Way forward

If the feature is of interest, it needs some brain-time from someone
to see if there's any way to add this.  For example could the compiler
derive the `<'id>` annotations automatically for GhostCell?  Or is
there any other way of making this work in the standard library?

Or for multi-threaded runtimes, could
[`qcell::TLCell`](https://docs.rs/qcell/0.4.1/qcell/) work?  This
allows a single cell-owner in every thread.  So several borrows can be
going on at the same time in different threads, but this is safe
because the cell isn't `Sync`.

The interface between cells and `Context` should be straightforward
once a particular cell type is demonstrated to be workable with the
`poll` interface and futures ecosystem.  For example copying the API
style of Stakker:

```
let rc = Rc::new(AsyncCell::new(1_u32));
*rc.rw(cx) = 2;
```

So effectively/logically you obtain read-write access to a cell by
naming the authority by which you claim access, in this case the poll
Context.  In this case it really is naming rather than accessing since
the checks are done at compile time and the address that `cx`
represents doesn't actually get passed anywhere or evaluated.


## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

The main problem is that Niklaus has got used to a safer environment
and it feels dangerous to go back to RefCell and have to
manually-verify that his cell borrows are panic-free.

### **What are the sources for this story?**

The author of Stakker is trying to interface it to async/await and
futures.

### **Why did you choose *NAME* to tell this story?**

Niklaus seems to be the wildcard.  The main thing is that Niklaus is
claimed to have an unconventional background.  So that means he's not
part of the establishment.  He's a new programmer to async/await, but
in this interpretation he isn't a new programmer.  Maybe he's like
Grace or Barbara, but he doesn't have his thinking limited by any
particular existing conventions or trained methodology.

### **How would this story have played out differently for the other characters?**

The other characters perhaps wouldn't have heard of statically-checked
cell borrows (at least until the recent GhostCell announcement) so
would be unaware of the possibility of things being safer.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
