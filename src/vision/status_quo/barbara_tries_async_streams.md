# 😱 Status quo stories: Barbara tries async streams

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

## The story

Barbara has years of experience in Rust and was looking forward to using some of that experience with the brand-new async functionality. Async/await had been a dream of Rust for so long, and it was finally here!

As she began her next side project, she would quickly partner up with other experienced Rust developers. One of these Rust developers, who had more async experience than Barbara, suggested they use 'async streams' as the core abstraction for this project. Barbara trusted the experience of this other developer. Though she didn't yet understand how async streams worked, she was happy to go along with the decision and build her experience over time.

Month after month, the side project grew in scope and number of users. Potential contributors would try to contribute, but some would leave because they found the combination of concepts and the additional set of borrowchecker-friendly code patterns difficult to understand and master. Barbara was frustrated to lose potential contributors but kept going.

Users also began to discover performance bottlenecks as they pushed the system harder. Barbara, determined to help the users as best she could, pulled her thinking cap tight and started to probe the codebase.

In her investigations, she experimented with adding parallelism to the async stream. She knew that if she called `.next()` twice, that in theory she should have two separate futures. There were a few ways to run multiple futures in parallel, so this seemed like it might pan out to be a useful way of leveraging the existing architecture.

Unfortunately, to Barbara's chagrin, async streams do not support this kind of activity. Each `.next()` must be awaited so that the ownership system allowed her to get the next value in the stream. Effectively, this collapsed the model to being a synchronous iterator with a more modern scent. Barbara was frustrated and started to clarify her understanding of what asynchrony actually meant, looking through the implementations for these abstractions.

When she was satisfied, she took a step back and thought for a moment. If optional parallelism was a potential win and the core data processing system actually was going to run synchronously anyway -- despite using async/await extensively in the project -- perhaps it would make more sense to redesign the core abstraction.

With that, Barbara set off to experiment with a new engine for her project. The new engine focused on standard iterators and rayon instead of async streams. As a result, the code was much easier for new users, as iterators are well-understood and have good error messages. Just as importantly, the code was noticeably faster than its async counterpart. Barbara benchmarked a variety of cases to be sure, and always found that the new, simpler approach performed better than the async stream original.

To help those who followed after her, Barbara sat down to write out her experiences to share with the rest of the world. Perhaps future engineers might learn from the twists and turns her project took.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

* **What are the morals of the story?**
    * Easy to get the wrong idea. The current state of documentation does not make the use cases clear, so it's easy to grab this as an abstraction because it's the closest that fits.
    * Async streams are just iterators. Async streams do not offer useful asynchrony in and of themselves. A possible help here might be renaming "async streams" to "async iterators" to help underscore their use case and help developers more quickly understand their limitations.
    * A single async stream can not be operated on in parallel. They open up asynchrony only during the `.next()` step and are unable to offer asynchrony between steps (eg by calling `.next()` twice and operating on the resulting Futures).
* **What are the sources for this story?**
    * Two years of experience with async streams in Nushell.
    * You can watch a [recording of the on-going experimentation](https://youtu.be/2AknX7canvw) as well.
    * User stories for the difficulty of using async streams were largely shared with the team on the project [discord](https://discord.gg/NtAbbGn).
    * We tried a number of different approaches including [using the `async_stream!`](https://crates.io/crates/async_stream) macro, [removing off the `async_stream!` macro](https://github.com/nushell/nushell/pull/1916) because of the error message cliff, and variations of the above.
* **Why did you choose Barbara to tell this story?**
    * Barbara is an experienced engineer who may come to async streams and async/await in general with a partially-incorrect set of baseline understanding. It may take her time to understand and see more clearly where her model was wrong because there are things similar to other experiences she's had. For example, Rust futures differ from C++ futures and do not offer the same style of asynchrony. Terms like "streams" sound like they may have more internal functionality, and it would be easy for an experienced developer to trip up with the wrong starting assumption.
* **How would this story have played out differently for the other characters?**
    * Alan may have come to a similar idea for an architecture, as async/await is popular in languages like JavaScript and C#. Once Alan attempted to use asynchrony between units of work, namely using async streams, this is where Alan may have failed. The amount of Rust one has to know to succeed here is quite high and includes understanding Arc, Pin, Streams, traits/adapters, the borrowchecker and dealing with different types of errors, and more.
    * Grace may have chosen a different core abstraction from the start. She has a good chance of thinking through how she'd like the data processing system to work. It's possible she would have found threads and channels a better fit. This would have had different trade-offs.
    * Niklaus may have also tried to go down the async stream path. The information available is mixed and hype around async/await is too strong. This makes it shine brighter than it should. Without experience with different systems languages to temper the direction, the most likely path would be to experiment with asynchrony and hope that "underneath the surface it does the right thing."

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
