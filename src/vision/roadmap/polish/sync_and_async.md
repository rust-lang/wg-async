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

Roughly want we want here is that code that differs only in its syncness should do the same thing.
Of course, this is not strictly possible because a sync and async program are fundamentally different program.
We still want something approximating this.
Below are several principles that try to make this more precise.
For each one, we are talking about a synchronous and asynchronous version of a piece of code where the synchronous version is basically the async version with all the `async` and `.await` keywords removed.
Note that this assumes there are no manually implemented futures.

1. **Equality**: if the sync version and the async version both produce a value, then the values are the same.
2. **Termination**: either both the sync and async version terminate (or can be polled to completion in the async case), or both do not terminate.
3. **Panic**: the sync version panics if and only if the async version panics.
4. **Types***: if the sync version has type `T` then the async version has type `Future<Output = T>` and vice-versa.
5. **Compilation***: either both the sync and async version compile successfully, or they both produce equivalent compiler errors on the same line.

The first three principles are probably not terrible hard to achieve.
The last two, marked with an asterisk, may not be completely possible or even desirable in all cases.

For types, there is a fundamental difference in the async code because `.await` points expose types that would be purely internal in the sync version.
One impact of this is that the auto traits may not be the same between the two.
We might be able to get this property in one direction though.
For example, adding a `.await` might make future not `Send`, but removing a `.await` will probably not remove any auto traits.

Compilation is closely related to the types goal because if async causes the types to change then this could introduce or remove compilation errors.
Additionally, we will probably have some async-only diagnostics, such as the [`must_not_suspend` lint][must_not_suspend].

### Library and Ecosystem

At a high level, the library and ecosystem goals are about having comparable capabilities available in libraries for both sync and async code.
For example, mutexes in an async context need integration with the runtime, so the standard synchronous mutex is not generally suitable for async code.
For this reason, most async runtimes provide some form of `AsyncMutex`.

Note that one way to achieve this may be through [Async Overloading].

🛠️ This area is still underdeveloped, so if you would like to help this would be a great place to pitch in!

### Automated Testing

🛠️ This area is still underdeveloped, so if you would like to help this would be a great place to pitch in!

[Async Overloading]: ../async_overloading.md
[must_not_suspend]: ./lint_must_not_suspend.md
