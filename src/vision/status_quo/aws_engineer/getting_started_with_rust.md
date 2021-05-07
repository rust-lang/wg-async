# Status quo of an AWS engineer: Getting started with Rust

For his latest project, Alan is working on a new service, [DistriData]. Like many AWS services, they are trying to move on a tight deadline. 

The plan is to implement the new service in Rust. The service that they are rewriting was implemented in Java, but it was having difficulty with high tail latencies and other performance hiccups, and they would like to reduce resource usage by adopting Rust.

This service is being implemented in Rust. The start is a bit different than what he is used to. There's not much infrastructure. They still define their service interface using the same modeling language, but there is no tooling to generate a server from it.

[DistriData]: https://rust-lang.github.io/wg-async-foundations/vision/projects/DistriData.html

## IDE setup

Of course, the very first thing Alan does it to tweak his IDE setup. He's happy to learn that IntelliJ has support for Rust, since he is accustomed to the keybindings and it has great integration with Brazil, AWS's internal build system.

Still, as he plays around with Rust code, he realizes that the support is not nearly at the level of Java. Autocomplete often gets confused. For example, when there are two traits with the same name but coming from different crates, Intellij often picks the wrong one. It also has trouble with macros, which are very common in async code. Some of Alan's colleagues switch to VSCode, which is sometimes better but has many of the same problems; Alan decides to stick with IntelliJ.

## Building the first server

Alan asks around the company to learn more about how Async Rust works and he is told to start with the tokio tutorial and the Rust book. He also joins the company slack channel, where he can ask questions. The tokio tutorial is helpful and he is feeling relatively confident. 

## Missing types during Code review

One problem Alan finds has to do with AWS's internal tooling (although it would be the same in most places). When browsing Rust code in the IDE, there are lots of tips to help in understanding, such as tooltips showing the types of variables and the like. In code reviews, though, there is only the plain text. Rust's type inference is super useful and make the code compact, but it can be hard to tell what's going on when you just read the plain source.
