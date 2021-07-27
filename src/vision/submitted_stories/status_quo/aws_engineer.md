# 😱 Status quo stories: Status quo of an AWS engineer

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

This tells the story of Alan, an engineer who works at AWS.

* [Writing a Java-based service at AWS](aws_engineer/writing_a_java_based_service.md): Alan is accustomed to using many convenient tools for writing Java-based services.
* [Getting started with Rust](aws_engineer/getting_started_with_rust.md): Alan gets tapped to help spin up a new project on a tight timeline. He hasn't used Rust before, so he starts trying to setup an environment and learn the basics.
* [Coming from Java](aws_engineer/coming_from_java.md): Alan finds that some of the patterns he's accustomed to from Java don't translate well to Rust.
* [Exploring the ecosystem](aws_engineer/ecosystem.md): The Rust ecosystem has a lot of useful crates, but they're hard to find. "I don't so much find them as stumble upon them by accident."
* At first, Rust feels quite ergonomic to Alan. The async-await system seems pretty slick. But as he gets more comfortable with Rust, he starts to encounter situations where he can't quite figure out how to get things setup the way he wants, and he has to settle for suboptimal setups:
    * [Juggling error handling](aws_engineer/juggling_error_handling.md): Alan tries to use `?` to process errors in a stream.
    * [Failure to parallelize](aws_engineer/failure_to_parallelize.md): Alan can't figure out how to parallelize a loop.
    * [Borrow check errors](aws_engineer/borrow_check_errors.md): Alan tries to write code that fills a buffer and returns references into it to the caller, only to learn that Rust's borrow checker makes that pattern difficult.
* As Alan goes deeper into Async Rust, he learns that its underlying model can be surprising. One particular [deadlock](aws_engineer/solving_a_deadlock.md) takes him quite a long time to figure out.
* [Encountering pin](aws_engineer/encountering_pin.md): Wrapping streams, `AsyncRead` implementations, and other types requires using `Pin` and it is challenging.
* [Figuring out the best option](aws_engineer/figuring_out_the_best_option.md): Alan often encounters cases where he doesn't know what is the best way to implement something. He finds he has to implement it both ways to tell, and sometimes even then he can't be sure.
* [Testing his service](aws_engineer/testing_the_service.md): Alan invents patterns for Dependency Injection in order to write tests.
* [Missed Waker leads to lost performance](aws_engineer/missed_waker_leads_to_lost_performance.md): Alan finds his service his not as fast as the reference server; the problem is ultimately due to a missed `Waker`, which was causing his streams to wake up much later than it should've.
* [Debugging performance problems](aws_engineer/debugging_performance_problems.md): Alan finds more performance problems and tries to figure out their cause using tooling like `perf`. It's hard.
* [Using JNI](aws_engineer/using_jni.md): Alan uses JNI to access services that are only available using Java libraries.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**

* Building services in Rust can yield really strong results, but a lot of hurdles remain:
    * 'If it compiles, it works' is not true: there are lots of subtle variations.
    * Debugging correctness and performance problems is hard, and the tooling is not what folks are used to.
    * Few established patterns to things like DI.
    * The ecosystem has a lot of interesting things in it, but it's hard to navigate.

### **What are the sources for this story?**

This story is compiled from discussions with service engineers in various AWS teams.

### **Why did you choose Alan to tell this story?**

Because Java is a very widely used language at AWS.

### **How would this story have played out differently for the other characters?**

Most parts of it remain the same; the main things that were specific to Java are some of the patterns Alan expected to use. Similarly, few things are specific to AWS apart from some details of the setup.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
