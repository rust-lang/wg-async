# wg-async-foundations
Working group dedicated to improving the foundations of async I/O in Rust

**Leads**: [@tmandry] and [@nikomatsakis]

[@tmandry]: https://github.com/tmandry
[@nikomatsakis]: https://github.com/nikomatsakis

## Getting involved

Our meetings take place at [11:30 AM PST](https://everytimezone.com/s/8c679f10) every Thursday in our [Zulip stream][zulip]. Feel free to stop by then (or any time!) to introduce yourself.

**If you'd like something to work on, check our [mentored issues][E-mentor] for places to get started.** Feel free to claim one of these by adding a comment with the text `@rustbot claim`.

You can also take a look at our [ongoing work][project board] to get a sense of what we're up to, and to look for more unclaimed issues you could tackle.

[E-mentor]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AAsyncAwait-Triaged+label%3AE-mentor+is%3Aopen&type=Issues
[project board]: https://github.com/orgs/rust-lang/projects/2

## What is the goal of this working group?

This working group is focused around implementation/design of the “foundations” for Async I/O. This includes the async-await language feature but also the core Future trait and adapters.

### Current focus

The **current focus** of the group is polishing the async-await
feature and working on the async book. 

### Overall roadmap and progress

- ✅ Stabilize the `Future` trait
- ✅ Deliver a "minimal viable product" (MVP) introducing `async fn` inherent functions and async blocks.
- ⚒️ Polish the async-await feature, improving diagnostics, spurious errors, and other corner cases.
- ⚒️ Write the [async book](https://github.com/rust-lang/async-book), which introduces how the core language features in support of Async I/O and teaches you how to use them.

### Future areas

Possible future areas for consideration include:

- Stabilize other core traits in std, such as `AsyncRead`
- Support async fn in traits
- Support async closures (and possibly an `AsyncFn` trait)
- Support consuming async streams conveniently (e.g., `for await` loops or some similar thing)
- Support authoring async streams conveniently via async generators
- Support async drop 

However, we've decided to largely defer this sort of work until we've
finished off more of the polish work on async-await.

## Triage meetings

In our weekly triage meetings, we take new issues assigned [`A-async-await`] and categorize them. The process is:

- Review the [project board], from right to left:
  - Look at what got **Done**, and celebrate! :tada:
  - Review **In progress** issues to check we are making progress and there is a clear path to finishing (otherwise, move to the appropriate column)
  - Review **Blocked** issues to see if there is anything we can do to unblock
  - Review **Claimed** issues to see if they are in progress, and if the assigned person still intends to work on it
  - Review **To do** issues and assign to anyone who wants to work on something
- Review [uncategorized issues]
  - Mark `P-low`, `P-medium`, or `P-high`
  - Add `P-high` and _assigned_ `E-needs-mentor` issues to the [project board]
  - Mark `AsyncAwait-triaged`
- If there's still a shortage of **To do** issues, review the list of [`P-medium`] or [`P-low`] issues for candidates

### Mentoring

If an issue is a good candidate for mentoring, mark `E-needs-mentor` and try to find a mentor.

Mentors assigned to issues should write up mentoring instructions. **Often, this is just a couple lines pointing to the relevant code.** Mentorship doesn't require intimate knowledge of the compiler, just some familiarity and a willingness to look around for the right code.

After writing instructions, mentors should un-assign themselves, add `E-mentor`, and remove `E-needs-mentor`. On the project board, if a mentor is assigned to an issue, it should go to the **Claimed** column until mentoring instructions are provided. After that, it should go to **To do** until someone has volunteered to work on it.

[`A-async-await`]: https://github.com/rust-lang/rust/labels/A-async-await
[uncategorized issues]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AA-async-await+is%3Aopen+-label%3AAsyncAwait-Triaged&type=Issues
[`P-high`]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AAsyncAwait-Triaged+label%3AP-high+is%3Aopen&type=Issues
[`P-medium`]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AAsyncAwait-Triaged+label%3AP-medium+is%3Aopen&type=Issues
[`P-low`]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AAsyncAwait-Triaged+label%3AP-low+is%3Aopen&type=Issues

## Links

- [wg-async-foundations Zulip][zulip]
- [Project board][project board]

[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/187312-wg-async-foundations

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
