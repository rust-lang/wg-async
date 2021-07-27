# Boxable async fn

## Impact

* Able to easily cause some async functions, blocks, or closures to allocate their stack space lazilly when called (by 'boxing' it)
    * Combined with profiler or other tooling support, this can help to tune the size of futures
* Boxed async blocks allows particular *portions* of a function to be boxed, e.g. cold paths

## Design notes

Example might be to use a decorator:

```rust
#[boxed]
async fn foo() { }
```

This does not have to desugar to `-> Box<dyn Future<...>>`; it can instead desugar to `Box<impl Future>`, or perhaps a nominal type to permit recursion.

