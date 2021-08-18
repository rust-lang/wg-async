# Problems

The following sections break out the biggest problems that we see.  These problems are grouped into categories, but they also show up in the various narratives.

| Category | Problem |
| --- | --- |
| [**Too many ways to do it**](./problems/tmwdi.md) | 
| | Have to make complex choices up front |
| | Libraries are locked to particular runtimes |
| | Combining runtimes can lead to panics or poor performance |
| | No standard read, write trait |
| | No standard async iteration trait |
| | No standard way to spawn tasks |
| | No standard way to spawn blocking tasks |
| | No standard way to get timers |
| | No standard way to access files, network sockets |
| | Scattered or missing documentation |
| [**Footguns**](./problems/footguns.md) |
| | Unexpected cancellation |
| | Nested awaits |
| | Async-sync-async sandwich |
| | Complexity cliff for low-level APIs |
| [**Missing language features**](./problems/language_features.md) |
| | Async fn in traits |
| | Async closures |
| | Async destructors |
| | Async main |
| | Async tests |
| | Ability to guarantee destructors or progress |
| | Ability to spawn tasks that access borrowed data |
| | Ability to be generic over thread-safety |
| [**Poor tooling support**](./problems/tooling.md) |
| | Complex stacktraces |
| | Inconsistent IDE support |
| | Debuggers, profilers operate at wrong level of abstraction |
| | Hard to debug stuck tasks or other runtime failures |
| | Async runtimes don't offer runtime metrics |
| [**Writing tests is hard**](./problems/testing.md) |
| | No standard or easy way to mock network access |
| | No way to test concurrent schedules |

