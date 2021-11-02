# Polish

## Impact

* Users can predict and understand why the compiler raises error messages. Errors are aligned with an experienced user's intuition about how Rust works.
* Error messages identify common misconceptions, suggest solutions, and are generally on par with sync Rust.
  * Errors not only show that there is a problem, they help the user to fix it and to learn more about Rust (possibly directing the user to other documentation).
  * The compiler may suggest crates from the ecosystem to help solve problems when appropriate.
* Lints guide the user away from common errors and help them both to get started with async Rust and to maintain async Rust programs over time.
* Rust's async implementation is high quality and reflects an attention to detail.
  * No internal compiler errors
  * Compiler analysis and code generation passes are precise and not unnecessarily conservative.
  * Integration with low-level tooling and the like is high-quality.
  * The generated code from the compiler is high quality and performant.

## 🛠️ How to Help

The goal of a highly polished async experience in Rust has many details and touches many aspects of the project, including both the async area in particular and the Rust project in general.
This means there are lots of ways to get involved!

The weekly [triage meeting] primarily focuses on polish issues, so that is a great place to get to know people already working on the project and find out what people are actively working on.
We meet over Zulip, so feel free to just lurk, or chime in if you want to.
See the [triage meeting] page for details about when the meeting happens and how to join.

Even outside of regularly scheduled meetings, you are welcome to hang out in the Async Working Group's [Zulip stream].
There are usually a few people active there who are happy to discuss async-related topics.

If you are looking for a specific area to help, there are several places where we track work.

* The [Initiatives](#initiatives) list down below.
* The Async Work Group [Project Board]. The "On Deck" column is a good place to start looking.
* Issues on the [wg-async-foundations repo]. These tend to relate to project organization and longer term objectives.
* Issues on the [Rust repo]. Specifically, issues tagged [AsyncAwait-Polish], [A-async-await]. Issues that are also tagged with E-mentor will have mentoring instructions, which are usually pointers to specific points in the code where changes will be needed to fix the issue.

Finally, a great way to contribute is to point out any rough edges you come across with writing async Rust.
This can be done either through issues on the [Rust repo], or by starting a topic on our [Zulip stream].
Examples of rough edges that we are interested in include confusing error messages or places where Rust behaved in a way you found surprising or counter-intuitive.
Knowing about these issues helps to ensure we are fixing the right things.

[A-async-await]: https://github.com/rust-lang/rust/labels/A-async-await
[AsyncAwait-Polish]: https://github.com/rust-lang/rust/labels/AsyncAwait-Polish
[Project Board]: https://github.com/orgs/rust-lang/projects/2
[Rust repo]: https://github.com/rust-lang/rust/issues
[Triage meeting]: ../../triage.md
[wg-async-foundations repo]: https://github.com/rust-lang/wg-async-foundations/issues
[Zulip stream]: https://rust-lang.zulipchat.com/#narrow/stream/187312-wg-async-foundations

## Initiatives

| Initiative                                 | State | Key participants |
| ---                                        | ---   | --- |
| [Error messages]                           | 💤    | |
| Lint: [Must not suspend]                   | 🦀    | [Gus Wynn] |
| Lint: [Blocking in async context]          | 💤    | |
| Lint: [Large copies], large generators     | 💤    | |
| [Cleaner async stacktraces]                | 💤    | |
| [Precise generator captures]               | 🦀    | [eholk] |
| [Sync and async behave the same]           | 💤    | |

[eholk]: https://github.com/eholk/
[Lang team]: https://www.rust-lang.org/governance/teams/lang
[Blocking in async context]: ./polish/lint_blocking_fns.md
[Large copies]: ./polish/lint_large_copies.md
[Must not suspend]: ./polish/lint_must_not_suspend.md
[RFC]: https://rust-lang.github.io/rfcs/3014-must-not-suspend-lint.html
[Precise generator captures]: ./polish/precise_generator_captures.md
[Gus Wynn]: https://github.com/guswynn
[Error messages]: ./polish/error_messages.md
[Cleaner async stacktraces]: ./polish/stacktraces.md
[Sync and async behave the same]: ./polish/sync_and_async.md
