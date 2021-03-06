# 🙋‍♀️ Cast of characters

### Barbara: embedded developer doing networking

Barbara is building a sensor grid system using Rust. The sensors communicate wirelessly to relay their results, and Barbara is responsible for the networking component. These sensors are relatively simple devices with limited resources, so she is coding in a `#[no_std]` environment and is very careful about things like allocation.

[axes]: ../characters.md#axes

#### 🤔 Frequently Asked Questions

* How would you describe Barbara in terms of the [axes]?
    * *Programming language background:* C, C++
    * *Overall programming experience:* Advanced
    * *Async experience:* Writing custom state machines in C
    * *Target applications:* Embedded networking
    * *Role:* Team member
* What is most important to Barbara about async Rust? Why?
    * Barbara needs low-level control. She can't use typical tools.
    * She also cares about portability: few people work in her space, and she would like to be able to use as many tools and libraries as posible.
* What is least important to Barbara about async Rust? Why?
    * XXX
* What are key parts of Barbara's background or story that distinguishes them from the other characters?
    * Working in embedded.
