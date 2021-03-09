# ⚡ Applications: Embedded devices

## Brief description

Threading and communication on embedded systems that often lack basics like an operating system or threading support. Communication with device drivers and managing callbacks.

## 🤔 Frequently Asked Questions

* What are some of the "special requirements" for embedded applications?
    * Embedded developers need to write error-free applications outside of the comfort zone of an operating system. Rust helps to prevent many classes of programming errors at compile time which inspires confidence in the software quality and and cuts time intensive build-flash-test iterations.
    * Embedded developers needs good hardware abstraction. Frameworks in other languages do not provide the sophistacted memory mapped IO to safe type abstraction tooling which have been created by the Rust teams.
    * Embedded developers care about hard real time capabilities; the concept of "you only pay for what you use" is very important in embedded applications. The combination of the inherently asynchronous interrupt handling of microcontrollers with the Rust async building blocks are a perfect match to effortlessly create applications with hard realtime capabilities.
    * Embedded developers are particularly appreciative of strong tooling support. The availibity of the full environment via `rustup` and the integration of the full toolchain with `cargo` and `build.rs` make her very happy because she can focus on what she does best instead of having regular fights with the environment.
* Do embedded applications require custom tailored runtimes?
    * Likely yes! The tradeoffs for an embedded application and a typical server are very different. Further, most server-grade frameworks are not `#[no_std]` compatible and far exceeded the available footprint on an embedded device.