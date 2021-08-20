# Lint must not suspend

## Impact

* Warnings when values which ought not to be live over an await are, well, live over an `await`.
    * Example: lock guards.
