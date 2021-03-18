# ⚡ Projects: MonsterMesh (embedded sensors)

## What is this?

This is a sample project for use within the various ["status quo"] or ["shiny future"] stories.

["status quo"]: ../status_quo.md
["shiny future"]: ../shiny_future.md

## Description

"MonsterMesh" is a sensor mesh on microcontrollers using Rust. The nodes communicate wirelessly to relay their results. These sensors are built using very constrained and low power hardware without operating system, so the code is written in a `#[no_std]` environment and is very careful about available resources.

## 🤔 Frequently Asked Questions

* **What makes embedded projects like MonsterMesh different from others?**
    * Embedded developers need to write error-free applications outside of the comfort zone of an operating system. Rust helps to prevent many classes of programming errors at compile time which inspires confidence in the software quality and and cuts time intensive build-flash-test iterations.
    * Embedded developers needs good hardware abstraction. Frameworks in other languages do not provide the sophisticated memory mapped IO to safe type abstraction tooling which have been created by the Rust teams.
    * Embedded developers care about hard real time capabilities; the concept of "you only pay for what you use" is very important in embedded applications. The combination of the inherently asynchronous interrupt handling of microcontrollers with the Rust async building blocks are a perfect match to effortlessly create applications with hard realtime capabilities.
    * Embedded developers are particularly appreciative of strong tooling support. The availability of the full environment via `rustup` and the integration of the full toolchain with `cargo` and `build.rs` make her very happy because she can focus on what she does best instead of having regular fights with the environment.
* **Does MonsterMesh require a custom tailored runtime?**
    * Yes! The tradeoffs for an embedded application like MonsterMesh and a typical server are very different. Further, most server-grade frameworks are not `#[no_std]` compatible and far exceeded the available footprint on the sensor nodes.
* **How much of this project is likely to be built with open source components from crates.io?**
    * Having no operating system to provide abstractions to it, MonsterMesh will contain all the logic it needs to run. Much of this, especially around the hardware-software-interface is unlikely to be unique to MonsterMesh and will be sourced from crates.io. However, the further up the stack one goes, the more specialized the requirements will become.
* **How did you pick the name?**
    * So glad you asked! Please watch this [entertaining video](https://www.youtube.com/watch?v=vNuVifA7DSU).