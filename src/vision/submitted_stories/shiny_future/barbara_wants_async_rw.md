# ✨ Barbara Wants Async Read Write

## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

Character: Barbara.

Barbara is the creator of a `sans-io` library for Rust. She designed her library to 
integrate with `async` and her goal was to make it runtime agnostic; so that it could
be as broadly used as possible.  Unfortunately, when she first wrote the library `async`
did not have a standard abstraction for Buffered IO. So her first implementation did
not use buffered IO.  When she tried to update her library to use buffered IO so as to
improve performance she was confronted with the problem that each runtime had its own
implementation and abstractions. The result was several unavoidable compromises on her
runtime-agnostic design goals.  She was able to achieve her performance improvements
but only with runtime specific implementations; leaving her with a larger more complex
code base.

But today is a fantastic day for Barbara.  The Rust async team has recently released
the latest version of `async` and part of that release was a standard Buffered Async
Read/Write abstraction.  Since then, several runtimes have been updated to implement
the new abstraction and Barbara refactored the buffered IO module to use this new
abstraction and she deprecated the runtime specific solutions. Today is the day that
Barbara gets to release her new version of `sans-io` which takes full advantage of the
buffered Async Read/Write abstractions now defined in `async`.  The result is a library
that maintains the same performance gains that it had with the runtime specific modules
while greatly reducing the amount of code.

## 🤔 Frequently Asked Questions

*NB: These are generic FAQs. Feel free to customize them to your story or to add more.*

### What status quo stories are you retelling?

*Link to status quo stories if they exist. If not, that's ok, we'll help find them.*

### What are the key attributes of this shiny future?

- Just like AsyncRead/AsyncWrite there are no standard traits for buffered I/O

    - This is made worse by the fact that there isn’t even ecosystem traits for buffered writes.

- There are no standard (or even present in futures-io) concrete types for async buffered I/O.

    - Each major runtime has their own async BufReader, BufWriter types.

- All the issues with creating runtime agnostic libraries are very present here. (TODO: link with runtime agnostic lib story)
std::io doesn’t have a BufWrite trait for sync I/O.

    - It’s less of an issue than in async Rust because of the existence of the standardized std::io::BufWriter.



### What is the "most shiny" about this future? 

*Thing about Rust's core "value propositions": performance, safety and correctness, productivity. Which benefit the most relative to today?*
This benefits productivity and correctness the most. The problem is not performance, in particular, as each runtime provides buffered IO solutions.  The problem is that they are inconsistent and not compatible. This means that writing code that is compatible with any async runtime becomes both: much more difficult and much more likely to be wrong when runtimes change.

### What are some of the potential pitfalls about this future?

*Thing about Rust's core "value propositions": performance, safety and correctness, productivity. Are any of them negatively impacted? Are there specific application areas that are impacted negatively? You might find the sample [projects] helpful in this regard, or perhaps looking at the goals of each [character].*
- Having a design which makes it difficult for existing runtimes to make their buffered IO types compatible or to migrate their runtimes over to the new designs.

### Did anything surprise you when writing this story? Did the story go any place unexpected?

*The act of writing shiny future stories can uncover things we didn't expect to find. Did you have any new and exciting ideas as you were writing? Realize some complications that you didn't foresee?*
The most surprising thing is that there is a buffered read type in `futures` but no buffered *write* type in `futures`.  I would expect both or neither.

### What are some variations of this story that you considered, or that you think might be fun to write? Have any variations of this story already been written?

*Often when writing stories, we think about various possibilities. Sketch out some of the turning points here -- maybe someone will want to turn them into a full story! Alternatively, if this is a variation on an existing story, link back to it here.*
No variations.

### What are some of the things we'll have to figure out to realize this future? What projects besides Rust itself are involved, if any? (Optional)

*Often the 'shiny future' stories involve technical problems that we don't really know how to solve yet. If you see such problems, list them here!*



[character]: ../characters.md
[comment]: ./comment.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[projects]: ../projects.md
[htvsq]: ../how_to_vision/shiny_future.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
