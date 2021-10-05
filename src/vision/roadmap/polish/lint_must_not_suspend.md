# Lint must not suspend

## Impact

* Warnings when values which ought not to be live over an await are, well, live over an `await`.
    * Example: lock guards.

## Milestones

| Milestone                                | Status | Key Participants |
| ---                                      | ---    | ---              |
| Implemented the [RFC]                  | 🦀    | [Gus Wynn] |

[RFC]: https://rust-lang.github.io/rfcs/3014-must-not-suspend-lint.html
[Gus Wynn]: https://github.com/guswynn
