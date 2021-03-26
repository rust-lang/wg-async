# 😱 Status quo stories: Template

*This is a template for adding new "status quo" stories. To propose a new status quo PR, do the following:*

* *Create a new file in the [`status_quo`] directory named something like `Alan_tries_to_foo.md` or `Grace_does_bar.md`, and start from [the raw source from this template]. You can replace all the italicized stuff. :)*
* *Do not add a link to your story to the [`SUMMARY.md`] file; we'll do it after merging, otherwise there will be too many conflicts.*

*For more detailed instructions, see the [How To Vision: Status Quo] page!*

*If you're looking for ideas of what to write about, take a look at the [open issues]. You can also [open an issue of your own] to throw out an idea for others.*

[How To Vision: Status Quo]: ../how_to_vision/status_quo.md
[the raw source from this template]: https://raw.githubusercontent.com/rust-lang/wg-async-foundations/master/src/vision/status_quo/template.md
[`status_quo`]: https://github.com/rust-lang/wg-async-foundations/tree/master/src/vision/status_quo
[`SUMMARY.md`]: https://github.com/rust-lang/wg-async-foundations/blob/master/src/SUMMARY.md
[open issues]: https://github.com/rust-lang/wg-async-foundations/issues?q=is%3Aopen+is%3Aissue+label%3Astatus-quo-story-ideas
[open an issue of your own]: https://github.com/rust-lang/wg-async-foundations/issues/new?assignees=&labels=good+first+issue%2C+help+wanted%2C+status-quo-story-ideas&template=-status-quo--story-issue.md&title=


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

*Write your story here! Feel free to add subsections, citations, links, code examples, whatever you think is best.*

## 🤔 Frequently Asked Questions

[YouBuy](../projects/YouBuy.md) is written using an async web framework that predates the stabilization of async function syntax. When Alan joins the company, it is using async functions for its business logic, but can't use them for request handlers because the framework doesn't support it yet. It requires the handler's return value to be `Box<dyn Future<...>>`. Rather than switching YouBuy to a different web framework, Alan decides to contribute to the web framework himself.

After a bit of a slog, he manages to make the web framework capable of using an `async fn` as an http request handler. He does this by making a wrapper function that boxes up the `impl Future`.

It's still not fantastically ergonomic though. Because the web framework predates async function syntax, it requires you to take ownership of the request context (`fn handler(state: State)`) and return it alongside your response in the success/error cases (`Ok((state, response))` or `Err((state, error))`). This means that Alan can't use the `?` operator inside his http request handlers. Alan knows answer is to make another wrapper function so that the handler can take an `&mut` reference to `State` for the lifetime of the future, but Alan can't work out how to express this. He submits his pull-request upstream as-is, but it nags on his mind that he has been defeated.

Shortly afterwards, someone raises a bug about `?`, and a few other web framework contributors try to get it to work, but they also get stuck. When Alan tries it, the compiler diagnostics keep sending him around in circles. He can work out how to express the lifetimes for a function that returns a `Box<dyn Future + 'a>` but not an `impl Future` because of how where clauses are expressed. Alan longs to be able to say "this function takes an async function as a callback" (`fn register_handler(handler: impl async Fn(state: &mut State) -> Result<Response, Error>)`) and have Rust elide the lifetimes for him, like how they are elided for async functions.

A month later, one of the contributors finds a forum comment by Barbara explaining how to express what the Alan is after (using higher-order lifetimes and a helper trait). They implement this and merge it.

When Alan sees another open source project struggling with the same issue, he notices that Barbara has helped them out as well.


* **What are the morals of the story?**
    * Callback-based APIs with async callbacks are a bit fiddly, because of the `impl Future` return type, but not insurmountable.
    * Callback-based APIs with async callbacks that borrow their arguments are almost impossible to write without help.
* **What are the sources for this story?**
    * This is from the author's [own experience](https://github.com/rust-lang/wg-async-foundations/issues/78#issuecomment-808193936).
* **Why did you choose Alan/YouBuy to tell this story?**
    * Callback-based apis are a super-common way to interact with web frameworks. I'm not sure how common they are in other fields.
* **How would this story have played out differently for the other characters?**
    * I suspect that even many Barbara-shaped developers would struggle with this problem.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
