# Async fn fundamentals

## Impact

* Able to write `async fn` in traits and trait impls
    * Able to easily declare that `T: Trait + Send` where "every async fn in `Trait` returns a `Send` future"
    * Traits that use `async fn` can still be [dyn safe](./async_fn_fundamentals/dyn_async_trait.md) though some tuning may be required
    * Async functions in traits desugar to [impl Trait in traits]
* Able to write ["async fn drop"][async drop] to declare that the destructor may await
* Support for [async closures]

## Milestones

| Milestone | State | Key participants |
| --- | --- | --- |
| Author [evaluation doc] for [static async trait] | 🦀 | [tmandry]
| Author [evaluation doc] for [dyn async trait]  | 🦀 | [tmandry]
| Author [evaluation doc] for [async drop] | 🦀 | [tmandry]
| Author [evaluation doc] for [impl Trait in traits]  | 💤 |
| [Stabilize] [type alias impl trait] | 💤  |
| [Stabilize] [generic associated types]  | 💤 |
| Author RFC for async fn in traits  | 💤 |
| Author [evaluation doc] for [async closures]  | 💤 |
| Author RFC for async fn in traits  | 💤 |
| [Feature complete] for async fn in traits | 💤 |
| [Feature complete] for [impl Trait in traits] | 💤 |
| [Feature complete] for [async drop] | 💤 |
| [Feature complete] for [async closures] | 💤 |

[nikomatsakis]: https://github.com/nikomatsakis/
[oli-obk]: https://github.com/oli-obk/
[jackh726]: https://github.com/jackh726/
[tmandry]: https://github.com/tmandry/
[basics]: ./async_fn_fundamentals/basics.md
[async drop]: ./async_fn_fundamentals/async_fn_fundamentals.md
[async closures]: ./async_fn_fundamentals/async_closures.md
[impl Trait in traits]: ./async_fn_fundamentals/impl_trait_in_traits.md
[type alias impl trait]: ./async_fn_fundamentals/tait.md
[generic associated types]: ./async_fn_fundamentals/gats.md
[static async trait]: ./async_fn_fundamentals/static_async_trait.md
[dyn async trait]: ./async_fn_fundamentals/dyn_async_trait.md

[evaluation doc]: ./roadmap/stages.html#evaluation
[stabilize]: https://lang-team.rust-lang.org/initiatives/process/stages/stabilized.html
[feature complete]: https://lang-team.rust-lang.org/initiatives/process/stages/feature_complete.html
