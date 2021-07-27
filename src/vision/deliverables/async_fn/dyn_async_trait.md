# Dyn async trait

## Impact

* Traits that contain [`async fn`](./async_fn_in_traits.md) or [impl trait in traits](./impl_trait_in_traits.md) can still be dyn safe
* Costs like boxing of futures are limited to code that uses `dyn Trait` and not to all users of the trait
* Reasonable defaults around things like `Send + Sync` and what kind of boxing is used
* Ability to customize those defaults for individual traits or on a crate-wide or module-wide basis

## Requires

* [dyn trait](./dyn_trait.md)

## Design notes

* Permit a trait `TheTrait` containing  `async fn` or [impl trait in traits](./impl_trait_in_traits.md) to be used with `dyn TheTrait`, at least if other criteria are met.
* Do not require annoying annotations.
* Permit the user to select, for `TheTrait`, how the futures will be boxed or otherwise represented, which would permit us to use `Box` or potentially other types like `SmallBox` etc. 
* User should also be able to control whether the resulting futures are assumed to be send.

### Older notes


The most basic desugaring of async fn in traits will make the trait not dyn-safe. "Inline" async fn in traits is one way to circumvent that, but it's not suitable for all traits that must be dyn-safe. There are other efficient options:

- Return a `Box<dyn Async<...>>` -- but then we must decide if it will be `Send`, right? And we'd like to only do that when using the trait as a `dyn Trait`. Plus it is not compatible with no-std (it is compatible with alloc).
  - This comes down to needing some form of opt-in.

This concern applies equally to other "`-> impl Trait` in trait" scenarios.

We have looked at revising how "dyn traits" are handled more generally in the lang team on a number of occasions, but [this meeting](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2020-01-13-dyn-trait-and-coherence.md) seems particularly relevant. In that meeting we were discussing some soundness challenges with the existing dyn trait setup and discussing how some of the directions we might go enabled folks to write their _own_ `impl Trait for dyn Trait` impls, thus defining for themselves how the mapping from Trait to dyn Trait. This seems like a key piece of the solution.

One viable route might be:

- Traits using `async fn` are not, by default, dyn safe.
- You can declare how you want it to be dyn safe:
  - `#[repr(inline)]`
  - or `#[derive(dyn_async_boxed)]` or some such
    - to take an `#[async_trait]`-style approach
  - It would be nice if users can declare their own styles. For example, Matthias247 pointed out that the `Box` used to allocate can be reused in between calls for increased efficiency.
- It would also be nice if there's an easy, decent default -- maybe you don't even _have_ to opt-in to it if you are not in `no_std` land.

## Frequently asked questions

### What are the limitations around allocation and no-std code?

"It's complicated". A lot of no-std code does have an allocator (it depends on alloc), though it may require fallible allocation, or permit allocation of fixed quantities (e.g., only at startup, or so long as it can be known to be O(1)).
