# Error messages for most confusing scenarios

## Impact

* Errors not only show that there is a problem, they help the user to fix it and to learn more about Rust (possibly directing the user to other documentation).

## Design notes

Of course there are an infinite number of improvements one could make. The point of this deliverable is to target the *most common* situations and confusions people see in practice. The final list is still being enumerated:

* Confusing error: Immutable reference to future is not a future [rust-lang/rust#87211](https://github.com/rust-lang/rust/issues/87211)