# Sync and async behave the same

## Impact

**Async code should not be surprising.** In general, if you surround a block of
synchronous code with `async` or mark a sync `fn` as `async`, nothing
unexpected should happen.

- The code should evaluate to the same value after awaiting.
- Any compilation errors should be essentially the same, modulo details around implicit futures in the return type.

## Milestones

| Milestone                                | Status | Key Participants |
| ---                                      | ---    | ---              |
| Define "behave the same"                 | 💤     |  |
| Create testing to ensure same behavior   | 💤     |  |
