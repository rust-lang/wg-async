# ❓ How to vision

## How you can help

| When | What |
| --- | --- |
| ✅ **Now** till 2021-04-30 | Improve the [sample projects][hvp] |
| ✅ **Now** till 2021-04-30 | Propose new ["status quo" stories][hvsq] or [comment] on existing PRs |
| 🛑 Starting 2021-04-02 | Propose new ["shiny future" stories][hvsf] or [comment] on existing PRs |
| 🛑 Starting 2021-04-30 | Vote for the [awards] on the status quo and shiny future stories! |

## The big picture

The process we are using to write the vision doc encourages active collaboration and "positive sum" thinking. It starts with a brainstorming period, during which we aim to collect as many "status quo" and "shiny future" stories as we can. 

This brainstorming period runs for six weeks, until the end of April. For the first two weeks (until 2021-04-02), we are collecting ["status quo" stories][hvsq] only. After that, we will accept both ["status quo"][hvsq] and ["shiny future" stories][hvsf] until the end of the brainstorming period. Finally, to cap off the brainstorming period, we will select winners for [awards] like "Most Humorous Story" or "Most Supportive Contributor". 

Once the brainstorming period is complete, the working group leads will begin work on assembling the various stories and shiny futures into a coherent draft. This draft will be reviewed by the community and the Rust teams and adjusted based on feedback.

### Brainstorming

The brainstorming period runs until 2021-04-30:

* Folks open "status quo" and (starting 2021-04-02) "shiny future" story PRs against the [wg-async-foundations repo][repo].
    * [Templates and instructions for status quo stories can be found here.][hvsq] 
    * You can also browse the [open "status quo" issues] for ideas!
    * [Templates and instructions for shiny future stories can be found here.][hvsf]
* We collectively [comment] on these PRs, helping to improve them and make them more complete.
* Unless they contain factual inaccuracies, the aim is to merge **all** PRs opened in the brainstorming period.
    * We also expect people to sometimes extend other stories with additional details or FAQs.
* At the end of the brainstorming period, we will vote and give [awards] for things like "most amusing". (We'd like suggestions on the best categories!)

#### The more the merrier!

**During this brainstorming period, we want to focus on getting as many ideas as we can.** Having multiple "shiny futures" that address the same problem is a feature, not a bug, as it will let us mix-and-match later to try and find the best overall plan. Comments and questions will be used as a [tool for improving understanding or sharpening proposals.][comment] Presenting alternative ideas is done by [writing an alternative story][alt].

[alt]: https://nikomatsakis.github.io/wg-async-foundations/vision/how_to_vision/comment.html#you-might-just-want-to-write-your-own-story

#### Reviewing contributions 

To merge a story or project PR, any member of the working group can open a topic on Zulip and propose it be merged. Ideally there will be no outstanding concerns. If a second member of the working group approves, the PR can then be merged.

### Harmonizing

At this point, the [wg leads] will write the draft vision document, drawing on the status quo and shiny future stories that were submitted.
Like an RFC, this draft vision doc will be opened for comment and improved based on the resulting feedback.
When the [wg leads] feel it is ready, it will be taken to the [lang] and [libs] teams for approval (and other Rust teams as appropriate).

[lang]: https://www.rust-lang.org/governance/teams/lang
[libs]: https://www.rust-lang.org/governance/teams/library

### Living document

This meant to be a **living document**. We plan to revisit it regularly to track our progress and update it based on what we've learned in the meantime. Note that the shiny future stories in particular are going to involve a fair bit of uncertainty, so we expect them to change as we go.
 
[hvsq]: ./how_to_vision/status_quo.md
[hvsf]: ./how_to_vision/shiny_future.md
[Vote]: ./how_to_vision/awards.md
[Vote]: ./how_to_vision/awards.md#Vote
[comment]: ./how_to_vision/comment.md
[awards]: ./how_to_vision/awards.md
[wg leads]: ../welcome.md#leads
[hvp]: ./how_to_vision/projects.md
[repo]: https://github.com/rust-lang/wg-async-foundations
[open "status quo" issues]: https://github.com/rust-lang/wg-async-foundations/labels/status-quo-story-ideas

## Wait, did somebody say awards?

Yes! We are planning to give [awards] in various categories for folks who write [status quo](./how_to_vision/status_quo.md) and [shiny future](./how_to_vision/shiny_future.md) PRs. The precise categories are TBD. Check out the [awards] page for more details.
