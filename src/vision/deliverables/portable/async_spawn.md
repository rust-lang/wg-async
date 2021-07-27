# Async spawn, spawn-blocking

## Impact

* Able to write libraries or applications that use a trait to spawn async or blocking tasks without referring to a particular runtime
* Able to use the trait in a dyn-safe fashion