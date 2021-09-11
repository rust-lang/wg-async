# Boxable async fn

## Impact

* Able to easily cause some async functions, blocks, or closures to allocate their stack space lazilly when called (by 'boxing' it)
    * Combined with profiler or other tooling support, this can help to tune the size of futures
* Boxed async blocks allows particular *portions* of a function to be boxed, e.g. cold paths

## Milestones

| Milestone | State | Key participants |
| --- | --- | --- |
| Author [evaluation doc] | 💤  | |
| [Feature complete] implementation | 💤  | |

[evaluation doc]: ./roadmap/stages.html#evaluation
[stabilize]: https://lang-team.rust-lang.org/initiatives/process/stages/stabilized.html
[feature complete]: https://lang-team.rust-lang.org/initiatives/process/stages/feature_complete.html

## Design notes

Example might be to use a decorator:

```rust
#[boxed]
async fn foo() { }
```

This does not have to desugar to `-> Box<dyn Future<...>>`; it can instead desugar to `Box<impl Future>`, or perhaps a nominal type to permit recursion.

Another approach is the `box` keyword:

```rust
box async fn foo() { }
```

We can apply the keyword modifier to async blocks and closures:

```rust
fn foo() -> BoxFuture<Output = ()> {
    box async { ... }
}
```

```rust
async fn stuff(s: impl AsyncIterator) {
    s.map(box async |x| { ... })
}
```

This is useful for breaking up future types to make them more shallow.
