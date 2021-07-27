# Type alias impl trait

## Impact

* Able to use "unnameable types" in a variety of positions, such as function return types, struct fields, the value of an associated type, and have the compiler infer its value for you.
    * "unnameable types" refers to closures, futures, iterators, and any other type that is either impossible or tedious to write in full.
