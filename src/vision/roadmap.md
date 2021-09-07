# Roadmap

What follows is a list of *high-level goals*, like "async fn everywhere", that capture some part of the improved user experience. Each goal has associated *initiatives*, which are particular streams of work within that goal. Each goal and each initiative have an associated owner -- in some cases multiple owners -- who are the people responsible for ensuring that the goal/initiative is making progress. If you click on a goal/initiative, you will get a high-level description of its *impact*. That is, how the experience of using async Rust is going to change as a result of this work.

We categorize the goals and initiatives into four states:

| State | Meaning |
| --- | --- |
| ✅ | Done. |
| 🦀  | In progress: Work is ongoing! |
| ✋ | Help wanted: Seeking an [owner] to pursue this! Talk to the [wg leads] if you are interested. |
| 💤 | Paused: we are waiting to work on this until some other stuff gets done. |

[owner]: ./how_to_vision/owners.md

## Impact and milesetones

Clicking on active initiatives also shows a list of *milestones*. These milestones (things like "write an [evaluation doc]") indicate the planned work ahead of us. We meet every 2 weeks to assess our progress on these milestones and to update the list as needed.

[evaluation doc]: ./roadmap/stages.html#evaluation
[stabilize]: https://lang-team.rust-lang.org/initiatives/process/stages/stabilized.html
[feature complete]: https://lang-team.rust-lang.org/initiatives/process/stages/feature_complete.html

## Overview

| Deliverable | State | Progress | [Owner] |
| --- | --- | --- | --- |
| 🔻 [Async fn everywhere] | 🦀  | ▰▰▱▱▱▱ | [tmandry] |
| &nbsp;&nbsp;↳ [Type Alias Impl Trait] | 🦀  | ▰▰▰▰▰▱ | [oli-obk] |
| &nbsp;&nbsp;↳ [Generic Associated Types] | 🦀  | ▰▰▰▰▰▱ | [jackh726] |
| &nbsp;&nbsp;↳ [Fundamentals] | 🦀  | ▰▰▱▱▱▱ | [tmandry] |
| &nbsp;&nbsp;↳ [Boxable async functions] | 💤  | ▰▱▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Async main and tests] | 💤 | ▰▱▱▱▱▱ | |
| 🔻 [Scoped spawn and reliable cancellation] | 💤 | ▰▱▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Capability] | 💤 | ▰▱▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Scope API] | 💤 | ▰▱▱▱▱▱ | |
| 🔻 [Async iteration] | 🦀  | ▰▰▱▱▱▱ | [estebank] |
| &nbsp;&nbsp;↳ [Async iteration trait] | 💤 | ▰▰▰▱▱▱ | |
| &nbsp;&nbsp;↳ [Generator syntax] | 💤 | ▰▰▱▱▱▱ | |
| 🔻 [Portable across runtimes] | 💤 | ▰▰▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Read/write traits] | 💤 | ▰▰▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Timer traits] | 💤 | ▰▱▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Spawn traits] | 💤 | ▰▱▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Runtime trait] | 💤 | ▰▱▱▱▱▱ | |
| 🔻 [Polish] | 🦀  | ▰▰▰▱▱▱ | [eholk] |
| &nbsp;&nbsp;↳ [Error messages] | 💤 | ▰▰▰▱▱▱ | |
| &nbsp;&nbsp;↳ [Must not suspend lint] | 💤 | ▰▰▰▰▱▱ | |
| &nbsp;&nbsp;↳ [Blocking function lint] | 💤 | ▰▰▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Lint against large copies] | 💤 | ▰▰▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Cleaner async stacktraces] | 💤 | ▰▱▱▱▱▱ | |
| 🔻 [Tooling] | 🦀  | ▰▰▱▱▱▱ | [pnkfelix] |
| &nbsp;&nbsp;↳ [Tokio console] | 🦀  | ▰▰▰▰▱▱ | [eliza weisman] |
| &nbsp;&nbsp;↳ [Crashdump debugging] | 🦀  | ▰▰▱▱▱▱ | [michaelwoerister] |
| 🔻 [Documentation] | 🦀  | ▰▰▱▱▱▱ | |
| &nbsp;&nbsp;↳ [Async book] | 💤 | ▰▰▱▱▱▱ | |
| 🔻 [Testing] | 💤 | ▰▱▱▱▱▱ |  |
| &nbsp;&nbsp;↳ tbd | 💤 | ▰▱▱▱▱▱ |
| 🔻 [Threadsafe portability] | 💤 | ▰▱▱▱▱▱ |  |
| &nbsp;&nbsp;↳ tbd | 💤 | ▰▱▱▱▱▱ |
| 🔻 [Async overloading] | 💤 | ▰▱▱▱▱▱ |  |
| &nbsp;&nbsp;↳ tbd | 💤 | ▰▱▱▱▱▱ |

[Async fn everywhere]: ./roadmap/async_fn.md
[fundamentals]: ./roadmap/async_fn/async_fn_fundamentals.md
[Async closures]: ./roadmap/async_fn/async_fn_fundamentals/async_closures.md
[Boxable async functions]: ./roadmap/async_fn/boxable.md
[Async main and tests]: ./roadmap/async_fn/async_main_and_tests.md
[Scoped spawn and reliable cancellation]: ./roadmap/scopes.md
[Capability]: ./roadmap/scopes/capability.md
[Scope API]: ./roadmap/scopes/scope_api.md
[Async iteration]: ./roadmap/async_iter.md
[Async iteration trait]: ./roadmap/async_iter/traits.md
[Generator syntax]: ./roadmap/async_iter/generators.md
[Portable across runtimes]: ./roadmap/portable.md
[Read/write traits]: ./roadmap/portable/read_write.md
[Timer traits]: ./roadmap/portable/timers.md
[Spawn traits]: ./roadmap/portable/spawn.md
[Runtime trait]: ./roadmap/portable/runtime.md
[polish]: ./roadmap/polish.md
[Error messages]: ./roadmap/polish/error_messages.md
[Blocking function lint]: ./roadmap/polish/lint_blocking_fns.md
[Must not suspend lint]: ./roadmap/polish/lint_must_not_suspend.md
[Cleaner async stacktraces]: ./roadmap/polish/stacktraces.md
[Lint against large copies]: ./roadmap/polish/lint_large_copies.md
[Tooling]: ./roadmap/tooling.md
[Tokio console]: https://github.com/tokio-rs/console
[Crashdump debugging]: ./roadmap/tooling/crashdump.md
[Documentation]: ./roadmap/documentation.md
[Async book]: ./roadmap/documentation/async_book.md
[Testing]: ./roadmap/testing.md
[Threadsafe portability]: ./roadmap/threadsafe_portability.md
[Async overloading]: ./roadmap/async_overloading.md
[Generic Associated Types]: https://github.com/nikomatsakis/generic-associated-types-initiative/
[Type Alias Impl Trait]: https://github.com/nikomatsakis/impl-trait-initiative/

[nikomatsakis]: https://github.com/nikomatsakis
[tmandry]: https://github.com/tmandry
[estebank]: https://github.com/estebank
[michaelwoerister]: https://github.com/michaelwoerister
[eholk]: https://github.com/eholk
[pnkfelix]: https://github.com/pnkfelix
[eliza weisman]: https://github.com/hawkw
[jackh726]: https://github.com/jackh726
[oli-obk]: https://github.com/oli-obk

[wg leads]: ../welcome.md#leads
