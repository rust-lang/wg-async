# 😱 Status quo stories: Niklaus Wants to Share Knowledge


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Niklaus, who sometimes goes by the pen name "Starol Klichols", has authored some long-form documentation about Rust that people have found helpful. One could even go so far as to call this documentation a ["book"][trpl].

Niklaus has typically minimized the use of crates in documentation like this as much as possible. Niklaus has limited time to dedicate to keeping the documentation up to date, and given the speed at which the ecosystem sometimes evolves, it's hard to keep up when crates are involved. Also, Niklaus would like to avoid limiting the readership of the documentation to the users of a particular crate only, and would like to avoid any accusations of favoritism.

But Niklaus would really really like to document async to avoid disappointing [people like Barbara]!

Niklaus was excited about [the RFC proposing that `block_on` be added to the stdlib][block-on-rfc], because it seemed like that would solve Niklaus' problems. Niklaus would really like to include `async` in a big update to the documentation. No pressure.

[trpl]: https://doc.rust-lang.org/stable/book/
[people like Barbara]: https://github.com/rust-lang/wg-async/blame/5ce418ac4076850f515034010cc51b707441f695/src/vision/status_quo/barbara_makes_their_first_steps_into_async.md#L22
[block-on-rfc]: https://github.com/rust-lang/rust/pull/65875
[htvsq]: ../status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade


## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
Writing documentation to go with the language/stdlib for something that is half in the language/stdlib and half in the ecosystem is hard.
This is related to [Barbara's story](https://rust-lang.github.io/wg-async/vision/status_quo/barbara_makes_their_first_steps_into_async.html) about wanting to get started without needing to pick an executor.
There are topics of async that apply no matter what executor you pick, but it's hard to explain those topics without picking an executor to demonstrate with.
We all have too much work to do and not enough time.

### **What are the sources for this story?**
* It me and Steve. Surprise!
* [We've wanted to add async to the book for a long time](https://github.com/rust-lang/book/issues/1275).
* So far, we use exactly one crate in the book, `rand`, and a recent update to `rand` caused readers confusion and caused a bunch of work on our part. [Take a look at all the issues linked to this PR](https://github.com/rust-lang/book/pull/2542). I really really really don't want to use more crates in the book.

### **Why did you choose *Niklaus* to tell this story?**
Niko said I couldn't add new characters.

### **How would this story have played out differently for the other characters?**
I happen to know that the next version of Programming Rust, whose authors might be described as different characters, includes `async` and uses `async-std`. So it's possible to just pick an executor and add async to the book, but I don't wanna.
