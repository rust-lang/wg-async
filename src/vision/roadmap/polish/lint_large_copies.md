# Lint large copies

## Impact

* Identify when large types are being copied and issue a warning. This is particularly useful for large futures, but applies to other Rust types as well.

## Milestones

| Milestone                                | Status | Key Participants |
| ---                                      | ---    | ---              |
| [Lang team] initiative proposal        | 💤     |  |
| Implemented                            | 💤     |  |

## Design notes

This is already implemented in experimental form. We would also need easy and effective ways to reduce the size of a future, though, such as [deliv_boxable](../async_fn/boxable.md).

[Lang team]: https://www.rust-lang.org/governance/teams/lang
