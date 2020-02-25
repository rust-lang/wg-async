# wg-async-foundations
Working group dedicated to improving the foundations of Async I/O in Rust

## Getting involved

Our meetings take place at [6:00 PM UTC](https://everytimezone.com/s/3b45ddfe) every Tuesday in our [Zulip stream][zulip]. Feel free to stop by then (or any time!) to introduce yourself.

**If you'd like something to work on, check our [mentored issues][E-mentor] for places to get started.** Feel free to claim one of these by commenting `@rustbot claim`.

You can also take a look at our [ongoing work][project board] to get a sense of what we're up to, and to look for more unclaimed issues you could tackle.

[E-mentor]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AAsyncAwait-Triaged+label%3AE-mentor+is%3Aopen&type=Issues
[project board]: https://github.com/orgs/rust-lang/projects/2

## What is the goal of this working group?

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

- Review [uncategorized issues]
  - Mark `P-low`, `P-medium`, or `P-high`
  - If an issue is a good candidate for mentoring, mark `E-needs-mentor` and try to find a mentor
  - Add `P-high` and _assigned_ `E-needs-mentor` issues to the [project board]
  - Mark `AsyncAwait-triaged`
- Review the [project board]
  - Move `E-needs-mentor` issues in **To do** to **Mentor assigned**
  - Move **Mentor assigned** issues with `E-mentor` to **To do**
  - Review **In progress** issues to check we are making progress (move to **Blocked** or **Done** when appropriate)
  - Review **To do** issues and assign to anyone who wants to work on something
- If there's a shortage of **To do** issues, review the list of [`P-medium`] or [`P-low`] issues for candidates

### Mentoring

Mentors assigned to issues should write up mentoring instructions. Often, this is just a couple lines pointing to the relevant code. After that they should un-assign themselves, add E-mentor, and remove E-needs-mentor.

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
