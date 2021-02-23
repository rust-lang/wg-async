# ⚡  Generator syntax

* It would be useful to be able to write a function to return an iterator or (in the async context) a generator
* The basic shape might be (modulo bikeshedding) `gen fn` that contains `yield`
* Some question marks:
    * How general of a mechanism do we want?
        * Just target iterators and streams, or shoot for something more general?
* Some of the question marks that arise if you go beyond iterators and streams:
    * Return values that are not unit
    * Have yield return a value that is passed by the caller of `next` ("resume args")