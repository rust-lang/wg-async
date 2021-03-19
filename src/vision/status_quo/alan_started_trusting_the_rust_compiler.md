# 😱 Status quo stories: Alan started trusting the Rust compiler


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

## The story
### Trust the compiler
Alan has a a lot of experience in _GC'd language_, but in the meantime has created some successful projects in Rust.
He has dealt with his fair share of race conditions/thread safety issues during runtime in _GC'd language_, but is now starting to trust that if his Rust code compiles,
he won't have those annoying runtime problems to deal with.

This allows him to try to squeeze his programs for as much performance as he wants, because the compiler will stop him when he tries things that could result in runtime problems.
After seeing the perfomance and the lack of runtime problems, he starts to trust the compiler more and more with each project finished.

He knows what he can do with external libraries, he does not need to fear concurrency issues if the library cannot be used from multiple threads, because the compiler would tell him.

His trust in the compiler solidifies further the more he codes in Rust.

### The first async project
Alan now starts with his first async project. He sees that there is no async in the standard library, but finds a crate that provides some async versions of the standard library functions.
He has some code written, but the compiler complains that `await` is only allowed in `async` functions. He now notices that all the examples use `#[async-version-of-std::main]` 
as an attribute on the `main` function in order to be able to turn it into an `async main`, so he does the same to get his code compiling.

This aligns with what he knows from _GC'd language_, where you also change the entry point of the program to be async, in order to use `await`.
Everything is great now, the compiler is happy, so no runtime problems, so Alan is happy.

The project is working like a charm.

### Fractured futures, fractured trust
The project Alan is building is starting to grow, and he decides to add a new feature. He starts using _async-library-that-does-not-run-on-current-executor_ in order to help him achieve this task.
After a lot of refactoring to make the compiler accept the program again, Alan is satisfied that his refactoring is done.
He runs his project but is suddenly greeted with a runtime error? How is this even possible? His project doesn't contain any out-of-bounds accesses, he never uses `.unwrap` or `.expect`?
At the top of the error message he sees: `This async thing needs to run in the context of this other async library, and not the one you are using.` 

Coming from a "simpler" async country, he now learns about Executors, Wakers, `Pin`,... Things he had not need to heed in _GC'd language_. 
He understands the current problems and why there is no one-size-fits-all executor (yet).

But now he realizes that there is a whole new area of runtime problems that he did not have to deal with in _GC'd language_, but he does in Rust.
Can he even trust the Rust compiler anymore? What other kinds of runtime problems can occur in Rust that can't in _GC'd language_?
If his projects keep increasing in complexity, will other new kinds of runtime problems keep popping up? Maybe it's better to stick with _GC'd language_, since Alan 
already knows all the runtime problems you can have over there.


## 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * The compile time guarantees that the Rust compiler gives, prevents a lot of runtime problems. 
If there is no way to "unify" all Executors behind some Traits, then something as important as being able `await` a future seems like something the compiler should help you with,
certainly because this is a "class" of runtime problems you don't encounter in mainstream GC'd languages w.r.t async code.
* **What are the sources for this story?**
    * Personal experience of the author.
* **Why did you choose Alan to tell this story?**
    * With his experience in _GC'd language_, Alan probably has experience with async code. Even though his _GC'd language_ protects him from certain classes of errors,
he can still encounter other classes of errors, which the Rust compiler prevents.
* **How would this story have played out differently for the other characters?**
    * For everyone except Barbara, I think these would play out pretty similarly, as this is a kind of problem unique to Rust. Since Barbara has a lot of Rust experience,
she would probably already be familiar with this aspect.
