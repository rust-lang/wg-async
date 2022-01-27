# ❓ How to vision: "Shiny future" stories

We want all Async Rust users and their hopes and dreams for what Async Rust should be in the future to be reflected in the async vision doc, so please help us by writing 'shiny future' stories about what you would like async Rust to look like! **Remember: we are in a brainstorming period.** Please feel free to leave comments in an effort to help someone improve their PRs, but if you would prefer a different approach, you are better off writing your own story. (In fact, you should write your own story even if you like their approach but just have a few alternatives that are worth thinking over.)

[character]: ../characters.md
[comment]: ./comment.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[projects]: ../projects.md

## TL;DR

Just want to get started? Here are quick instructions to get you going:

* **To write your own story:**
    * Create a PR based on the ["shiny future" template][template].
    * Do not add your file to [`SUMMARY.md`] -- that will create conflicts, we'll do it manually after merging.

## How to open a PR

If you have an idea you'd like to write about, please [open a PR using this template][template] and adding a new file into [the `shiny_future` directory][sfd]. Do not add your file to [`SUMMARY.md`], that will create conflicts. We'll do it after merging.

## Goals of a shiny future PR

Shiny future PRs "retell" the story from one or more status quo PRs. The story is now taking place 2-3 years in the future, when Async Rust has had the chance to make all sorts of improvements. **Shiny future stories are aspirational:** we don't have to know exactly how they will be achieved yet! (Of course, it never hurts to have a plan too.)

Like [status quo stories], each shiny future story is always presented from the POV of a particular [character]. They should be detailed. Sometimes this will mean you have to make stuff up, like method names or other details -- you can use the FAQ to spell out areas of particular uncertainty.

## The role of the FAQ

Every shiny future PR includes a FAQ. This FAQ should always include answers to some standard questions:

* What status quo story or stories are you retelling?
    * Link to the status quo stories here. If there isn't a story that you're retelling, [write it](./status_quo.md)!
* What is [Alan] most excited about in this future? Is he disappointed by anything?
    * Think about Alan's top priority (performance) and the expectations he brings (ease of use, tooling, etc). How do they fare in this future?
* What is [Grace] most excited about in this future? Is she disappointed by anything?
    * Think about Grace's top priority (memory safety) and the expectations she brings (still able to use all the tricks she knows and loves). How do they fare in this future?
* What is [Niklaus] most excited about in this future? Is he disappointed by anything?
    * Think about Niklaus's top priority (accessibility) and the expectations he brings (strong community that will support him). How do they fare in this future?
* What is [Barbara] most excited about in this future? Is she disappointed by anything?
    * Think about Barbara's top priority (productivity, maintenance over time) and the expectations she brings (fits well with Rust). How do they fare in this future?
* If this is an alternative to another shiny future, which one, and what motivated you to write an alternative?
    * Cite the story. Be specific, but focus on what you like about your version, not what you dislike about the other.
    * If this is not an alternative, you can skip this one. =)
* What [projects] benefit the most from this future?
* Are there any [projects] that are hindered by this future?

There are also some optional questions:

* What are the incremental steps towards realizing this shiny future?
    * Talk about the actual work we will do. You can link to [design docs](../../design_docs.md) or even add new ones, as appropriate.
    * You don't have to have the whole path figured out yet!
* Does realizing this future require cooperation between many projects?
    * For example, if you are describing an interface in libstd that runtimes will have to implement, talk about that.

You can feel free to add whatever other FAQs seem appropriate. You should also expect to grow the FAQ in response to questions that come up on the PR.

## The review process

When you opan a status quo PR, people will start to [comment] on it. These comments should always be constructive. They usually have the form of asking "in this future, what does NAME do when X happens?" or asking you to elaborate on other potential problems that might arise. Ideally, you should respond to every comment in one of two ways:

* Adjust the story with more details or to correct factual errors.
* Add something to the story's FAQ to explain the confusion.
    * If the question is already covered by a FAQ, you can just refer the commenter to that.

The goal is that, at the end of the review process, the status quo story has a lot more details that address the major questions people had.

## 🤔 Frequently Asked Questions

### What is the process to propose a shiny future story?
* Just open a PR [using this template][template].
* Do not add your file to [`SUMMARY.md`], that will create conflicts. We'll do it after merging.

### What character should I use for my shiny future story?
* Usually you would use the same character from the status quo story you are retelling.
* If for some reason you chose a different character, add a FAQ to explain why.

### What do I do if there is no status quo story for my shiny future?
[Write the status quo story first!](./status_quo.md)

#### What happens when there are multiple "shiny future" stories about the same thing?

During this brainstorming period, we want to focus on getting as many ideas as we can. Having multiple "shiny futures" that address the same problem is a feature, not a bug, as it will let us mix-and-match later to try and find the best overall plan.

### How much detail should I give? How specific should I be?
* Detailed is generally better, but only if those details are helpful for understanding the morals of your story.
* Specific is generally better, since an abstract story doesn't feel as real.

#### What is the "scope" of a shiny future story? Can I tell shiny future stories that involve ecosystem projects?

All the stories in the vision doc are meant to cover the full "end to end" experience of using async Rust. That means that sometimes they will take about things that are really part of projects that are outside of the Rust org. For example, we might write a shiny future that involves how the standard library has published standard traits for core concepts and those concepts have been adopted by libraries throughout the ecosystem. There is a FAQ that asks you to talk about what kinds of coordinate between projects will be required to realize this vision.

### What do I do when I get to details that I don't know yet?
Take your best guess and add a FAQ explaining which details are still up in the air.

### Do we have to know exactly how we will achieve the "shiny future"?

You don't have to know how your idea will work yet. We will eventually have to figure out the precise designs, but at this point we're more interested in talking about the experience we aim to create. That said, if you do have plans for how to achieve your shiny future, you can also include [design docs] in the PR, or add FAQ that specify what you have in mind (and perhaps what you have to figure out still).

### What do I do if somebody leaves a comment about how my idea will work and I don't know the answer?
Add it to the FAQ!

### What if we write a "shiny future" story but it turns out to be impossible to implement?

Glad you asked! The vision document is a living document, and we intend to revisit it regularly. This is important because it turns out that predicting the future is hard. We fully expect that some aspects of the "shiny future" stories we write are going to be wrong, sometimes very wrong. We will be regularly returning to the vision document to check how things are going and adjust our trajectory appropriately.

[template]: https://github.com/rust-lang/wg-async/tree/master/src/vision/shiny_future/template.md
[sfd]: https://github.com/rust-lang/wg-async/tree/master/src/vision/shiny_future
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async/blob/master/src/SUMMARY.md
