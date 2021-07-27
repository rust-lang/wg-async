# Threadsafe portability

## Impact

* Able to write code that can be easily made `Send` or not `Send`
    * The resulting code is able to switch between helper types, like `Rc` and `Arc`, appropriately.