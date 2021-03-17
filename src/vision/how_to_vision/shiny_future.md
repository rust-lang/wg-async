# ❓ How to vision: "Shiny future" stories

## 🛑 Not time for this yet 🛑

We're not ready for this yet! See the [how to vision](../how_to_vision.md) page for details on the phasing.

[character]: ../characters.md
[comment]: ./comment.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[domains]: ../domains.md

## Goals of a shiny future PR

Shiny future PRs "retell" the story from one or more status quo PRs. The story is now taking place 2-3 years in the future, when Async Rust has had the chance to make all sorts of improvements. **Shiny future stories are aspirational:** we don't have to know exactly how they will be achieved yet! (Of course, it never hurts to have a plan too.)

Like [status quo stories], each shiny future story is always presented from the POV of a particular [character]. They should be detailed. Sometimes this will mean you have to make stuff up, like method names or other details -- you can use the FAQ to spell out areas of particular uncertainty.

## The role of the FAQ

Every shiny future PR includes a FAQ. This FAQ should always include answers to some standard questions:

* What status quo story or stories are you retelling?
    * List out the PRs here. If there isn't a story that you're retelling, [write it](./status_quo.md)!
* What is [Alan] most excited about in this future? Is he disappointed by anything?
    * Think about Alan's top priority (performance) and the expectations he brings (ease of use, tooling, etc). How do they fare in this future?
* What is [Grace] most excited about in this future? Is she disappointed by anything?
    * Think about Grace's top priority (memory safety) and the expectations she brings (still able to use all the tricks she knows and loves). How do they fare in this future?
* What is [Niklaus] most excited about in this future? Is he disappointed by anything?
    * Think about Niklaus's top priority (accessibility) and the expectations he brings (strong community that will support him). How do they fare in this future?
* What is [Barbara] most excited about in this future? Is she disappointed by anything?
    * Think about Barbara's top priority (productivity, maintenance over time) and the expectations she brings (fits well with Rust). How do they fare in this future?
* If this is an alternative to another shiny future, which one, and what motivated you to write an alternative?
    * Cite the PR. Be specific, but focus on what you like about your version, not what you dislike about the other.
    * If this is not an alternative, you can skip this one. =)
* What [domains] benefit the most from this future?
* Are there any [domains] that are hindered by this future?

There are also some optional questions:

* What are the incremental steps towards realizing this shiny future?
    * Talk about the actual work we will do. You can link to [design docs](../design_docs.md) or even add new ones, as appropriate.
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

* What is the process to propose a shiny future story?
    * Just open a PR!
* What character should I use for my shiny future story?
    * Usually you would use the same character from the status quo story you are retelling.
    * If for some reason you chose a different character, add a FAQ to explain why.
* What do I do if there is no status quo story for my shiny future?
    * [Write the status quo story first!](./status_quo.md)
* How much detail should I give? How specific should I be?
    * Detailed is generally better, but only if those details are helpful for understanding the morals of your story.
    * Specific is generally better, since an abstract story doesn't feel as real.
* What do I do when I get to details that I don't know yet?
    * Take your best guess and add a FAQ explaining which details are still up in the air.
* What do I do if I don't know that my idea is technically feasible?
    * You don't have to know how your idea will work yet. You can add FAQs to try and clarify what parts you do know and what parts still need to be figured out.
* What do I do if somebody leaves a comment about how my idea will work and I don't know the answer?
    * Add it to the FAQ!
