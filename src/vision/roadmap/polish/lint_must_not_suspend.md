# Lint must not suspend

## Impact

* Warnings when values which ought not to be live over an await are, well, live over an `await`.
    * Example: lock guards.

## Milestones

| Milestone                                | Status | Key Participants |
| ---                                      | ---    | ---              |
| Implemented the [RFC]                  | ✅     | [Gus Wynn] |
| [Improve drop range tracking]          | 🦀     | [Eric Holk] |
| Stabilize the lint                     | 💤     |[Gus Wynn] |

[RFC]: https://rust-lang.github.io/rfcs/3014-must-not-suspend-lint.html
[Improve drop range tracking]: https://github.com/rust-lang/rust/pull/91032
[Gus Wynn]: https://github.com/guswynn
[Eric Holk]: https://github.com/eholk
