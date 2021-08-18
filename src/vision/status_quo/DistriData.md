# 😱 Status quo: DistriData

This is the story of [Alan, Barbara, Grace, and Niklaus][ABG and N] (ABG and N, hereafter) as they work on [DistriData]. It shows the various problems they hit in getting started and how they work around them today.

## What DistriData does

* The core functionality of DistriData is to take incoming write requests and replicate them to a several backend nodes. For reads, it performs a similar "inverse mapping", grabbing the data from the servers onto which it was replicated.

## Setting up their environments

* Alan and Grace are starting a new service, a rewrite of [DistriData] into Rust.
* Alan has mostly worked with services implemented in Java. Grace has done work on C++ services. They're both excited to learn Rust, they've heard a lot of good things about it.
* Alan starts using IntelliJ to write Rust code. It works reasonably well, but the support is not what Alan is used to from Java. It gets confused about things like multiple traits with the same name and so forth.
* Grace meanwhile decides to try out VSCode. She installs the RLS plugin but find that it's quite slow. Looking on the internet, she see mentions of rust-analyzer, which seems to work much better. "Why isn't this the default?", she wonders.

## Learning about async Rust

* They search google for "async rust book" and encounter the async Rust book. It's got a promising intro but quickly dives off into details they don't understand at all.
* They start searching message boards and get a lot of conflicting advice. It seems like there's a few different runtimes to pick from -- Alan is a bit confused by this, but Grace explains a bit about how Async I/O means that a lot of services typically provided by the kernel now live in user space. "I guess Rust doesn't have a standard runtime."
* After some discussion, they tinker a bit with both tokio and async-std. They ultimately decide to run with tokio because other people in the company are already using it and it seems to be the most widely used. It has a really nice tutorial (so does async-std). "It'd be nice if the official async book were like this tutorial", they think.
* XXX maybe at this point we could weave in some of the stories about asking Barbara and she doens't know what to recommend

## Exploring the ecosystem

* Having picked tokio, Alan and Grace thought they were all set, but they soon learn there are many more choices to be made. 
They discover that there are many versions of things like read traits, locks, streams. In many cases, tokio offers a version, but there are others to pick from.
* In general the Rust ecosystem seems to include a lot of choices, even beyond async, and it's often hard to evaluate between them. For example, Alan spent some time evaluating crates that do md5 hashing, for example, and found tons of choices. He does some quick performance testing and finds huge differences: openssl seems to be the fastest, so he takes that, but he is worried he may have missed some crates.
* Grace meanwhile is exploring http. She wants to stand-up a simple "echo server" in http. She realizes that tokio doesn't seem to include an http server, but they find references to the hyper library and they go with that.

## Getting error handling right is tricky

* XXX Adapt [this story](../submitted_stories/status_quo/aws_engineer/juggling_error_handling.md)

## Trying to parallelize a loop

* XXX Adapt [this story](../submitted_stories/status_quo/aws_engineer/failure_to_parallelize.md)

## Deadlock from nested awaits

* XXX Adapt [this story](../submitted_stories/status_quo/aws_engineer/solving_a_deadlock.md)

## Slowdown from missing waker

## Implementing a stream

## Packets arriving quickly lead to surprising problems

* XXX Adapt stories from Fuchsia engineers about cancellation, select, etc
* Talk about eventually arriving at standard patterns ..?

[ABG and N]: ../characters.md
[DistriData]: ../projects/DistriData.md