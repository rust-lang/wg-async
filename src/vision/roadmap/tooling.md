# Tooling

## Impact

* Tooling that gives insight into the state of async runtimes
    * How many tasks are running and what is their state
    * What are tasks blocked on and why?
    * Where is memory allocated?
    * Where is CPU time spent?
    * Flamegraph of where things spend their time
    * Perf-style profile of where things spend their time
* Tooling should also allow you to limit profiles to a particular request or to requests that meet particular criteria (e.g., coming from a particular source)
* Tooling should detect common hazards and identify them, suggesting fixes
    * Tasks that clone a `Waker` but don't trigger it
    * Tasks that don't respond to a request to cancellation for a long time
    * Outlier tasks that sleep for a very long time without being awoken
* Tooling should permit "always on" profiling that can be used in production
* Tooling can provide profile-based feedback:
    * Where to "heap-allocate" futures
    * Poll functions that execute for a long time without yielding
    * Imbalanced workloads across cores
* Tooling can be either customized or integrated into existing tools like perf, gdb, lldb, etc, as appropriate