# 😱 Status quo stories: Barbara makes their first foray into async

## Barbara's first big project in Rust: a journey marred by doubt

It's Barbara's last year at their university and for their master's thesis, they have chosen to create a distributed database.
They have chosen to use their favorite language, Rust, because Rust is a suitable language for low latency applications that they have found very pleasant to work in.
Their project presents quite a challenge since they have only written some small algorithms in Rust, and it's also their first foray into creating a big distributed system.

### Deciding to use Async
Up until now, Barbara has followed the development of Async from afar by reading the occasional Boats blog post, and celebrating the release announcements with the rest of the happy community.
Due to never having worked with async in other languages, and not having had a project suitable for async experimentation, their understanding of async and its ecosystem remained superficial.
However, since they have heard that async is suitable for fast networked applications, they decide to try using async for their distributed database.
After all, a fast networked application is exactly what they are trying to make.

To further solidify the decision of using async, Barbara goes looking for some information and opinions on async in Rust. Doubts created by reading some tweets about how most people should be using threads instead of async for simplicity reasons are quickly washed away by helpful conversations on the Rust discord.

### Learning about Async
Still enarmored with the first edition of the Rust book, they decide to go looking for an updated version, hoping that it will teach them async in the same manner that it taught them so much about the language and design patterns for Rust. Disappointed, they find no mention of async in the book, aside from a note that it exists as a keyword.

Not to be deterred, they go looking further, and start looking for similarly great documentation about async.
After stumbling upon the async book, their disappointment is briefly replaced with relief as the async book does a good job at solidifying what they have already learned in various blog posts about async, why one would use it and even a bit about how it all works under the hood.
They skim over the parts that seem a bit too in-depth for now like pinning, as they're looking to quickly get their hands dirty.
Chapter 8: The Async Ecosystem teaches them what they already picked up on through blog posts and contentious tweets: the choice of the runtime has large implications on what libraries they can use.

### The wrong time for big decisions
Barbara's dreams to quickly get their hands dirty with async Rust are shattered as they discover that they first need to make a big choice: what executor to use. Having had quite a bit of exposure to the conversations surrounding the incompatible ecosystems, Barbara is perhaps a bit more paranoid about making the wrong choice than the average newcomer.
This feels like a big decision to them, as it would influence the libraries they could use and switching to a different ecosystem would be all but impossible after a while. Since they would like to choose what libraries they use before having to choose an executor, Barbara feels like the decision-making is turned on its head. 

Their paranoia about choosing the right ecosystem is eased after a few days of research, and some more conversations on the Rust subreddit, after which they discover that most of the RPC libraries they might want to use are situated within the most popular Tokio ecosystem anyways. Tokio also has a brief tutorial, which teaches them some basic concepts within Tokio and talks a bit more about async in general.

### Woes of a newcomer to async
Being reasonably confident in their choice of ecosystem, Barbara starts building their distributed system.
After a while, they want to introduce another networking library of which the api isn't async. Luckily Barbara picked up on that blocking was not allowed in async (or at least not in any of the currently existing executors), through reading some blog posts about async. More reddit discussions point them towards spawn\_blocking in Tokio, and even rayon. But they're none the wiser about how to apply these paradigms in a neat manner.

Previously the design patterns learned in other languages, combined with the patterns taught in the book, were usually sufficient to come to reasonably neat designs.
But neither their previous experience, nor the async book nor the Tokio tutorial were of much use when trying to neatly incorporate blocking code into their previously fully async project.

### Confused ever after
To this day the lack of a blessed approach leaves Barbara unsure about the choices they've made so far and misconceptions they might still have, evermore wondering if the original tweets they read about most how most people should just stick to threads were right all along.

## 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * When entering Rust's async world without previous async experience, and no benchmarks for what good async design patters look like, getting started with async can be a bit overwhelming.
    * Other languages which only have a single ecosystem seem to have a much better story for beginners since there's no fear of lock in, or ecosystem fomo about making the wrong choices early on.
    * This lack of documentation on design patterns, and solid guidance about the async ecosystem for unopiniated newcomers is partially made up for by Rust's community which often provides educated opinions on the design and technical choices one should make. Because of this getting started in async favors those who know where to find answers about Rust: blogs, Discord, Reddit, etc.
* **What are the sources for their story?**
    * This is based on the author's personal experience
* **What documentation did the character read during this story?**
    * Various blog posts of withoutboats
    * A blog post which spurred a lot of discussion about blocking in async: https://async.rs/blog/stop-worrying-about-blocking-the-new-async-std-runtime/
    * A nice blog post about blocking in Tokio, which still doesn't have any nice design patterns: https://ryhl.io/blog/async-what-is-blocking/
    * An example of design patterns being discussed for sync Rust in the book: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html#trade-offs-of-the-state-pattern
    * Perhaps I should've read a bit more of Niko's blogs and his async interviews.
* **Why did you choose Barbara to tell their story?**
    * Like the author of this story, Barbara had previous experience with Rust, and was new to Async. Knowing where to find the community also played a significant part in this story.
* **How would their story have played out differently for the other characters?**
    * Characters with previous async experience would probably have had a better experience getting started with async in Rust since they might know what design patterns to apply to async code.
    On the other hand, since Rust's async story is noticeably different from other languages, having async experience in other languages might even be harmful by requiring the user to unlearn certain habits. I don't know if this is actually the case since I don't have any experience with async in other languages.
    * Characters which are less in touch with Rust's community than Barbara might have had a much worse time, since just skimming over the documentation might leave some lost, and unaware of common pitfalls. On the other hand, not having learned a lot about async through blog posts and other materials, might compel someone to read the documentation more thoroughly.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
