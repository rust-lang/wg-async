# Roadmap

What follows is a list of deliverables and their current status. Each deliverable goes through several stages -- ranging from "experimentation" (people in the greater Rust community authoring crates) to "stabilization" (something official is decided). For each deliverable, we indicate how far along in the process we feel that it has reached, and indicate whether it is something we are actively working on, something which is paused for the time being, or something that is blocked on something else. For things that we are actively working on, we give dates as to our estimated time of completion.

## Key

These emojis are used to represent the state of a deliverable:

| Symbol | Meaning |
| --- | --- |
| ✅ | Done |
| 🛠️ | Work is actively in progress |
| 🤔 | Planning to do this, have an owner in mind |
| ✋ | Planning to do this, need to find an owner |
| ⏸️ | Paused; no active blockers, but we are not pursuing it now for reasons of overall bandwidth |
| 🛑 | Blocked on some other deliverable, can't presently make progress |
| N/A | Inapplicable |

Each deliverable name is also a link to a description of what it entails. In many cases, the emojis link to more information, completed documents, and so forth. We also include estimated completion dates.

## Async fn everywhere

[Read more.](./deliverables/async_fn.md)

| Deliverable | [Owner] | [Exper.][stage] | [Eval.][stage] | [RFC][stage] | [Impl][stage] | [Stable][stage] |
| --- | --- | --- | --- | --- | --- | --- |
| [Async fn in traits](./deliverables/async_fn/async_fn_in_traits.md) | lang | ✅ | ⏸️ |
| [Impl trait in traits](./deliverables/async_fn/impl_trait_in_traits.md) | lang | ✅ | ⏸️ |
| [Type Alias Impl Trait](./deliverables/async_fn/tait.md) | lang | ✅ | ✅ | ✅ | [Sep][taitgat] | EOY |
| [Generic Associated Types](./deliverables/async_fn/gats.md) | lang | ✅ | ✅ | ✅ | [Sep][taitgat] | EOY |
| [Dyn async trait](./deliverables/async_fn/dyn_async_trait.md) | lang | ✅ | ⏸️ |
| [Dyn trait](./deliverables/async_fn/dyn_trait.md) | lang | ✅ | ⏸️ |
| [Inline async fn support](./deliverables/async_fn/inline_async_fn.md) | lang | ✅ | ⏸️ |
| [Async closures](./deliverables/async_fn/async_closures.md) | lang | ✅ | 🛑 |
| [Async drop](./deliverables/async_fn/async_drop.md) | lang | ✅ | 🛑 |
| [Async tests](./deliverables/async_fn/async_tests.md) | lang | ✅ | 🛑 |
| [Recursive async fn](./deliverables/async_fn/recursive.md) | lang | ✅ | 🛑 |
| [Boxable async fn](./deliverables/async_fn/boxable.md) | lang | ✅ | 🛑 |

## Easy acccess to borrowed data, reliable cancellation

[Read more.](./deliverables/borrowed_data_and_cancellation.md) 

| Deliverable | [Owner] | [Exper.][stage] | [Eval.][stage] | [RFC][stage] | [Impl][stage] | [Stable][stage] |
| --- | --- | --- | --- | --- | --- | --- |
| [Capability](./deliverables/borrowed_data_and_cancellation/capability.md) | lang, libs | ✅ | Sep |
| [Scope API](./deliverables/borrowed_data_and_cancellation/scope_api.md) | libs | 🛠️ | 

## Flexible async iteration

[Read more.](./deliverables/async_iter.md)

| Deliverable | [Owner] | [Exper.][stage] | [Eval.][stage] | [RFC][stage] | [Impl][stage] | [Stable][stage] |
| --- | --- | --- | --- | --- | --- | --- |
| [Async iteration traits](./deliverables/async_iter/traits.md) | libs | 🤔 |
| [Generators](./deliverables/async_iter/generators.md) | lang | ✅ | 🤔 |
| [Async iteration trait](./deliverables/portable/async_iter.md) | libs | 🤔 |

## Portable across runtimes, easy to switch

[Read more.](./deliverables/portable.md)

| Deliverable | [Owner] | [Exper.][stage] | [Eval.][stage] | [RFC][stage] | [Impl][stage] | [Stable][stage] |
| --- | --- | --- | --- | --- | --- | --- |
| [Async read, write trait](./deliverables/portable/async_read_write.md) | libs | ✅ | 🤔 |
| [Async spawn, spawn-blocking trait](./deliverables/portable/async_spawn.md) | libs | ✅ | ⏸️ |
| [Async timer trait](./deliverables/portable/async_timer.md) | libs |  ✅ | ⏸️ |
| [Portable async functionality in stdlib](./deliverables/portable/stdlib.md) | libs | 🤔 |

## Polish

[Read more.](./deliverables/polish.md)

| Deliverable | [Owner] | [Exper.][stage] | [Eval.][stage] | [RFC][stage] | [Impl][stage] | [Stable][stage] |
| --- | --- | --- | --- | --- | --- | --- |
| [must_not_suspend lint](./deliverables/polish/lint_must_not_suspend.md) | lang | ✅ | ✅ | [✅](https://github.com/rust-lang/rfcs/blob/master/text/3014-must-not-suspend-lint.md) | [🤔](https://github.com/rust-lang/rust/issues/83310) |
| [Lint against calling blocking functions from async fn](./deliverables/polish/lint_blocking_fns.md) | lang | ✅ | ✅ | ✋ |
| [Lint against large copies](./deliverables/polish/lint_large_copies.md) | lang | ✅ | ✅ | ✋ | ✅ |
| [Error messages for the most confusing scenarios](./deliverables/polish/error_messages.md) | compiler | N/A | N/A | N/A | now |
| [Stacktraces](./deliverables/polish/stacktraces.md) | lang | ✅ | ✅ | ✋ | ✅ |

## Tooling

[Read more.](./deliverables/tooling.md)

Tooling is presently in the [experimentation stage][stage], meaning that we are encouraging people to build things in the ecosystem that we can link here! If you have ideas that require support from the compiler or language, please raise them.

| Deliverable | [Owner] | [Exper.][stage] | [Eval.][stage] | [RFC][stage] | [Impl][stage] | [Stable][stage] |
| --- | --- | --- | --- | --- | --- | --- |
| [tokio-console] | tokio | N/A | N/A | N/A | 🛠️ | N/A

## Testing

[Read more.](./deliverables/testing.md)

Testing is presently in the [experimentation stage][stage], meaning that we are encouraging people to build things in the ecosystem that we can link here! If you have ideas that require support from the compiler or language, please raise them.

| Deliverable | [Owner] | [Exper.][stage] | [Eval.][stage] | [RFC][stage] | [Impl][stage] | [Stable][stage] |
| --- | --- | --- | --- | --- | --- | --- |
| [loom] | tokio | N/A | N/A | N/A | 🛠️ | N/A

## Documentation

[Read more.](./deliverables/documentation.md)

| Deliverable | [Owner] | [Exper.][stage] | [Eval.][stage] | [RFC][stage] | [Impl][stage] | [Stable][stage] |
| --- | --- | --- | --- | --- | --- | --- |
| [Async book](./deliverables/documentation/async_book.md) | lang, libs | ✅ | N/A | N/A | 🤔 | 

## Slightly past the horizon

The follow deliverables are slightly past the current horizon, but we are starting to noodle on what they might mean, and trying to leave space to pursue these directions in the future:

* [Threadsafe portability](./deliverables/threadsafe_portability.md)
* [Async overloading](./deliverables/async_overloading.md)

[stage]: ./roadmap/stages.md
[owner]: ./roadmap/owner.md
[taitgat]: https://github.com/rust-lang/wg-traits/projects/4
[tokio-console]: https://github.com/tokio-rs/console
[loom]: https://github.com/tokio-rs/loom
