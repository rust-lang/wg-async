# ⚡ Projects: DistriData (Generic Infrastructure)

## What is this?

This is a sample project for use within the various ["status quo"] or ["shiny future"] stories.

["status quo"]: ../status_quo.md
["shiny future"]: ../shiny_future.md

## Description

DistriData is the latest in containerized, micro-service distributed database technology. Developed completely in the open as part of Cloud Native Computing Foundation, this utility is now deployed in a large portion of networked server applications across the entire industry. Since it's so widely used, DistriData has to balance flexibility with having sensible defaults.

## 🤔 Frequently Asked Questions

* **What makes DistriData different from others?**
    * This project is meant to be used in many different ways in many different projects, and is not unique to any one application.
    * Many of those using this project will not even need or want to know that it's written in Rust.
* **Does DistriData require a custom tailored runtime?**
    * DistriData's concerns are at a higher level than the runtime. A fast, reliable, and resource conscious general purpose runtime will serve DistriData's needs.
* **How much of this project is likely to be built with open source components from crates.io?**
    * Yes, while DistriData receives many contributions, it's important to the team that when possible they utilize existing technologies that developers are already familiar with to ensure that contributing to the project is easy.
* **What is of most concern to this project?**
    * It needs to be resource conscious, fast, reliable, but above all else it needs to be easy to run, monitor, and maintain.
* **What is of least concern to this project?**
    * While DistriData is resource conscious, it's not resource *starved*. There's no need to make life difficult to save on a memory allocation here or there.
