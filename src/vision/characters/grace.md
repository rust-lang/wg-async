# 🙋‍♀️ Cast of characters

## Grace: the systems programming expert, new to Rust

### Variant A: Networking systems

Grace has been building high-performance networking systems in C and C++ for a number of years. She's accustomed to hacking lots of low-level details to coax the most performance she can from her network stack. She's also experienced her share of epic debugging sessions resulting from memory errors in C. She's intrigued by Rust: she likes the idea of getting the same control and performance she gets from C but with the productivity benefits she gets from memory safety. She's currently experimenting with introducing Rust into some of the systems she works on, and she's considering Rust for a few greenfield projects as well.

[axes]: ../characters.md#axes

### Variant B: Embedded

Grace is building a sensor mesh on microcontrollers using Rust. The nodes communicate wirelessly to relay their results, and Grace is responsible for the networking component. These sensors are built using very constrained and low power hardware without operating system, so she is coding in a `#[no_std]` environment and is very careful about available resources.

(Read more about [embedded applications](../applications/embedded.md).)

## 🤔 Frequently Asked Questions

* What does Grace want most from Async Rust?
    * Grace is most interested in memory safety. She is comfortable with C and C++ but she's also aware of the maintenance burden that arises from the lack of memory safety.
* What expectations does Grace bring from her current environment?
    * Grace expects to be able to be able to get the same performance she used to get from C or C++.
    * Grace is accustomed to various bits of low-level tooling, such as gdb or perf. It's nice if Rust works reasonably well with those tools, but she'd be happy to have access to better alternatives if they were available. She's happy using `cargo` instead of `make`, for example.