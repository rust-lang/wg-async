# 🙋‍♀️ Cast of characters

### Barbara: embedded developer doing networking

Barbara is building a sensor mesh on microcontrollers using Rust. The nodes communicate wirelessly to relay their results, and Barbara is responsible for the networking component. These sensors are built using very constrained and low power hardware without operating system, so she is coding in a `#[no_std]` environment and is very careful about available resources.

[axes]: ../characters.md#axes

#### 🤔 Frequently Asked Questions

* How would you describe Barbara in terms of the [axes]?
    * *Programming language background:* C, C++
    * *Overall programming experience:* Advanced
    * *Async experience:* Writing custom state machines and interrupt service routines in C
    * *Target applications:* Sensor mesh running on a bare metal microcontroller
    * *Role:* Team member
* What is most important to Barbara about async Rust? Why?
    * She needs to be able to write error free applications outside of the comfort zone of an operating system. Rust helps to prevent many classes of programming errors at compile time which inspires confidence in the software quality and and cuts time intensive build-flash-test iterations.
    * Barbara needs good hardware abstraction. Frameworks in other languages do not provide the sophistacted memory mapped IO to safe type abstraction tooling which have been created by the Rust teams.
    * She also cares about hard real time capabilities: The concept of "you only pay for what you use" is very important for her, the combination of the inherently asynchronous interrupt handling of microcontrollers with the Rust async building blocks are a perfect match to effortlessly create applications with hard realtime capabilities.
    * Barbara is also very happy about exceptional tooling. The availibity of the full environment via `rustup` and the integration of the full toolchain with `cargo` and `build.rs` make her very happy because she can focus on what she does best instead of having regular fights with the environemnt.
* What is least important to Barbara about async Rust? Why?
    * Server grade async frameworks because they're neither `#[no_std]`, nor would they have the required minimum footprint to fit into minimal resources.
* What are key parts of Barbara's background or story that distinguishes them from the other characters?
    * Experience with microcontrollers and the vastly different challenges which come with working on constrained devices very close to the hardware with only a few thing layers of abstractions between the physical world and the application.
