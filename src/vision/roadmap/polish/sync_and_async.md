# Sync and async behave the same

## Impact

**Async code should not be surprising.** In general, if you surround a block of
synchronous code with `async` or mark a sync `fn` as `async`, nothing
unexpected should happen.

* The code should evaluate to the same value after awaiting.
* Any compilation errors should be essentially the same, modulo details around implicit futures in the return type.

## Milestones

| Milestone                                | Status | Key Participants |
| ---                                      | ---    | ---              |
| Define "behave the same"                 | 💤     | [yoshuawuyts] |
| Create testing to ensure same behavior   | 💤     | [yoshuawuyts] |

[yoshuawuyts]: https://github.com/yoshuawuyts

## Details

Ideally, there should not be a lot of work needed specifically to achieve this goal.
Instead, the primary aim is to define principles that can inform the design in other places.
That said, automated testing to verify we have achieved these principles may take significant effort.

There are several ways we can look at what it means to behave the same.
One way is from a language and semantics standpoint, while another is from a library and ecosystem standpoint.
We will look at each of these in more detail, and then lay out some ideas for automated testing.

### Language and Semantics

Roughly what we want here is that code that differs only in its syncness should do the same thing.
Of course, this is not strictly possible because a sync and async program are fundamentally different programs.
We still want something approximating this.
Below are several principles that try to make this more precise.
For each one, we are talking about a synchronous and asynchronous version of a piece of code where the synchronous version is basically the async version with all the `async` and `.await` keywords removed.
Or conversely, we can view the async version as the sync version where all `fn`s have been replaced with `async fn` and all calls have `.await` added.
Note that this assumes there are no manually implemented futures.
This is an intentionally restrictive subset to focus on the core language semantics.
In the Library and Ecosystem section, we will discuss replacing standard library functionality with async equivalents to make this comparison more interesting.

1. **Equality**: if the sync version and the async version both produce a value, then the values are the same.
2. **Effects**: the same set of observable effects happen in both the sync and async version, and the effects happen in the same order, at least where order is specified. Effects here includes things such as writing to a file (although realistically the async version should use the async version of the File I/O API), observable memory reads or writes.
3. **Termination**: either both the sync and async version terminate (or can be polled to completion in the async case), or both do not terminate. Note that this is a special case of **Effects**.
4. **Panic**: the sync version panics if and only if the async version panics. Note that this is a special case of **Effects**.
5. **Types***: if the sync version of a function returns type `T` then the async version returns type `Future<Output = T>` and vice-versa. Functions or closures passed as parameters would undergo a similar transformation.
6. **Compilation***: either both the sync and async version compile successfully, or they both produce equivalent compiler errors on the same line.

The first four principles are probably not terrible hard to achieve.
The last two, marked with an asterisk, may not be completely possible or even desirable in all cases.

For types, there is a fundamental difference in the async code because `.await` points expose types that would be purely internal in the sync version.
One impact of this is that the auto traits may not be the same between the two.
We might be able to get this property in one direction though.
For example, adding a `.await` might make the future not `Send`, but removing a `.await` will probably not remove any auto traits.
See the following code for more detail:

```rust
fn sync_foo() {
    let t = NonSend { ... };
    bar(); // `sync_foo` is `Send` with or without this line.
}

async fn async_foo() {
    let t = NonSend { ... };
    bar().await; // With this line, the future returned by `async_foo` is `!Send`
                 // because NonSend is `!Send` and is alive across the `.await`
                 // point. Without this line, the future returned by `async_foo`
                 // is `Send`.
}
```

The key difference between the sync version and the async version here is that the suspension introduced by the `.await` point reveals internal details of `async_foo` that are not observable in the `sync_foo` case.

Compilation is closely related to the types goal because if async causes the types to change then this could introduce or remove compilation errors.
Additionally, we will probably have some async-only diagnostics, such as the [`must_not_suspend` lint][must_not_suspend].

### Library and Ecosystem

At a high level, the library and ecosystem goals are about having comparable capabilities available in libraries for both sync and async code.
For example, mutexes in an async context need integration with the runtime, so the standard synchronous mutex is not generally suitable for async code, although there are cases where a sync mutex makes sense [[1]], [[2]].
For this reason, most async runtimes provide some form of `AsyncMutex`.

[1]: https://ryhl.io/blog/async-what-is-blocking/
[2]: https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch20.html#the-group-table-synchronous-mutexes

Note that one way to achieve a comparable sync and async library ecosystem may be through [Async Overloading].

One way to approach this is to generalize the mostly mechanical transformation we described above to also include translating library calls, and then define what properties we would want to be preserved during the translation.
We would assume for synchronous blocking APIs, such as File I/O, the `Read` and `Write` traits, etc., we have corresponding async File I/O APIs, `AsyncRead` and `AsyncWrite` traits, etc.
The [async-std] project showed that most of the Rust standard library can be pretty directly translated into async code, other than cases where there were missing language features such as [async drop], [async traits], and [async closures].


[async-std]: https://async.rs/
[async-traits]: https://rust-lang.github.io/async-fundamentals-initiative/roadmap.html
[async drop]: https://rust-lang.github.io/async-fundamentals-initiative/roadmap/async_drop.html
[async-closures]: https://rust-lang.github.io/async-fundamentals-initiative/roadmap/async_closures.html


🛠️ This area is still underdeveloped, so if you would like to help this would be a great place to pitch in!

#### Open, Related Questions

- Should `async::iter::once` take `Future<Output = T>` or `T`?
  - Similarly for `async::iter::empty`
  - And `async::iter::repeat` (one future to completion and yield return value repeatedly)
  - `async::iter::repeat_with` would almost certainly want to take an async closure

### Automated Testing

🛠️ This area is still underdeveloped, so if you would like to help this would be a great place to pitch in!

[Async Overloading]: ../async_overloading.md
[must_not_suspend]: ./lint_must_not_suspend.md
