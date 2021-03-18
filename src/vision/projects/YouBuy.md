# ⚡ Projects: YouBuy (Server Application)

## What is this?

This is a sample project for use within the various ["status quo"] or ["shiny future"] stories.

["status quo"]: ../status_quo.md
["shiny future"]: ../shiny_future.md

## Description

YouBuy is a growing ecommerce website that now has millions of users. The team behind YouBuy is struggling to keep up with traffic and keep server costs low. Having originally written YouBuy in a mix of Ruby on Rails and Node, the YouBuy team decides to rewrite many parts of their service in Rust which they've investigated and found to be performant while still allowing for high levels of abstraction they're used to.

## 🤔 Frequently Asked Questions

* **What makes YouBuy and other server applications different from others?**
* Many server applications are written in languages with garbage collectors. Many of the things that Rust forces users to care about are not first order concerns for those working on server applications (e.g., memory management, stack vs heap allocations, etc.). 
* Many server applications are written in languages without static type checking. The developers of YouBuy don't have much experience with statically typed languages and some of the developers early in their Rust learning journeys expressed frustration that they found it hard to get their programs to compile especially when using async constructs.
* **Does YouBuy require a custom tailored runtime?**
* YouBuy should be perfectly fine with a runtime from crates.io. In fact, their concern isn't at the runtime level but at the high-level server framework level.
