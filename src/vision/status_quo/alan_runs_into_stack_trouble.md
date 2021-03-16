# 😱 Status quo stories: Alan runs into stack allocation trouble

[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md

[Alan runs into stack allocation trouble and is able to fix problems]: TODO

### The problem

Alan is working on a networking service written using async Rust and the tokio async runtime. One day, after many changes to his code base, Alan runs his application and hits an error:

```
$ .\target\debug\application.exe
thread 'main' has overflowed its stack
```

Perplexed, Alan sees if anything with his application works by seeing if he can get output when the `--help` flag is passed, but he has no luck:

```
$ .\target\debug\application.exe --help
thread 'main' has overflowed its stack
```

### Searching for the solution

Having really only ever seen stack overflow issues caused by recursive functions, Alan desperately tries to find the source of the bug but searching through the codebase for recursive functions only to find none. Having learned that Rust favors stack allocation over heap allocation (a concept Alan didn't really need to worry about before), he decided to also search for structs that looked large but none appeared in his code.

Confused, Alan reached out to Grace for her advice. She suggested making the stack size larger. Although she wasn't a Windows expert, she remembers hearing that stack sizes on Windows might be smaller than on Linux. After much searching, Alan discovers an option do just that: `RUSTFLAGS = "-C link-args=-Wl,-zstack-size=<size in bytes>"`.

While eventually Alan gets the program to run, the stack size must be set to 4GB before it does! This seems untenable, and Alan goes back to the drawing board.

Alan reaches out to Barbara for her expertise in Rust to see if she has something to suggest. Barbara recommends using `RUSTFLAGS = "-Zprint-type-sizes` to print some type sizes and see if anything jumps out. Barbara noted that if Alan does find a type that stands out, it's usually as easy as putting some boxes in that type to provide some indirection and not have everything be stack allocated. Alan never needs the nightly toolchain, but this option requires it so he installs it using `rustup`. After searching through types, one did stand out as being quite large. Ultimately, this was a red herring, and putting parts of it in `Box`es did not help.

### Finding the solution

After getting no where, Alan went home for the weekend defeated. On Monday, he decided to take another look. One piece of code, stuck out to him: the use of the `select!` macro from the `futures` crate. This macro allowed multiple futures to race against each other, returning the value of the first one to finish. This macro required the futures to be pinned which the docs had shown could be done by using `pin_mut!`. Alan didn't fully grasp what `pin_mut!` was actually doing when he wrote that code. The compiler had complained to him that the futures he was passing to `select!` needed to be pinned, and `pin_mut!` was what he found to make the compiler happy.

Looking back at the documents made it clear to Alan that this could potentially be the issue: `pin_mut!` pins futures to the stack. It was relatively clear that a possible solution would be to pin to the heap instead of the stack. Some more digging in the docs lead Alan to `Box::pin` which did just that. An extra heap allocation was of no consequence to him, so he gave it a try. Lo and behold, this fixed the issue! 

While Alan knew enough about pinning to know how to satisfy the compiler, he didn't originally take the time to fully understand what the consequences were of using `pin_mut!` to pin his futures. Now he knows!

## 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * When coming from a background of GCed languages, taking the time to understand the allocation profile of a particular piece of code is not something Alan was used to doing.
    * It was hard to tell where in his code the stack was being exhausted. Alan had to rely on manually combing his code to find the culprit.
    * Pinning is relatively confusing, and although the code compiled, Alan didn't fully understand what he wrote and what consequences his decision to use `pin_mut!` would have.
* **What are the sources for this story?**
    * This story is adapted from the experiences of the team working on the [Krustlet](https://github.com/deislabs/krustlet) project. You can read about this story in their own words [here](https://deislabs.io/posts/a-heaping-helping-of-stacks/).
* **Why did you choose Alan to tell this story?**
    * The programmers this story was based on have an experience mostly in Go, a GCed language.
    * The story is rooted in the explicit choice of using stack vs heap allocation, a choice that in GCed languages is not in the hands of the programmer.
* **How would this story have played out differently for the other characters?**
    * Grace would have likely had a similar hard time with this bug. While she's used to the tradeoffs of stack vs heap allocations, the analogy to the `Pin` API is not present in languages she's used to.
    * Barbara, as an expert in Rust, may have had the tools to understand that `pin_mut` is used for pinning to the stack while `Box::pin` is for pinning heap allocations.
    * This problem is somewhat subtle, so someone like Niklaus would probably have had a much harder time figuring this out (or even getting the code to compile in the first place).
* **Could Alan have used another API to achieve the same objectives?**
    * Perhaps! Tokio's `select!` macro doesn't require explicit pinning of the futures it's provided, but it's unclear to this author whether it would have been smart enough to avoid pinning large futures to the stack. However, pinning is a part of the way one uses futures in Rust, so it's possible that such an issue would have arisen elsewhere.