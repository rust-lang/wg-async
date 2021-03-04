# 🙋‍♀️ Cast of characters

## What is this?

These characters represent the various kinds of people that we are targeting as we design Async Rust. Each character has a specific background and set of experiences which affects what expectations they bring to Async Rust and what kinds of challenges they are able to overcome.

Hopefully you see yourself, at least somewhat, in one of these characters. If not, we'd like to hear about it, because there may be a missing character!

## The characters

### Alan: the full stack developer

| Status | Owner |
| --- | --- |
| ⚠️ Draft ⚠️ | ? |

Alan is a "full stack" developer who has been solving problems for customers in many companies. He's used node.js, Ruby-on-Rails, Django, and a couple other frameworks besides. Alan was recently hired at `fido.io`, a pioneering dog-food delivery business that uses a combination of deep learning and [IoT] to select the optimum dog food for your pet's health. Fido has chosen to build their networking stack in Rust. They were already using Rust for their deep learning code and they wanted to keep to a single language. For the time being, their needs are relatively simple. They want to stand up a server that serves up pages and connects to various back-end and cloud services.

[IoT]: https://en.wikipedia.org/wiki/Internet_of_things

#### 🤔 Frequently Asked Questions

* What is most important to Alan about async Rust? Why?
    * Alan prioritizes flexibility and ergonomics. He wants to be able to stand up a site quickly and be able to easily extend it to meet his future needs.
* What is least important to Alan about async Rust? Why?
    * Alan is not overly concerned about performance. The site has to be "fast enough" but they're not handling a ton of network traffic at the moment.
* What are key parts of Alan's background or story that distinguishes them from the other characters?
    * Alan's background is in dynamic languages like JavaScript, Ruby, and Python. Many of these languages have an async-await feature, but it works differently from Rust in some particulars.

### Grace: the principal engineer hacking on a data storage service

| Status | Owner |
| --- | --- |
| ⚠️ Draft ⚠️ | ? |

Grace is a principal engineer who has been building high-performance networking systems in Java and C++ for a number of years. She currently works on a distributed data storage service that is used in a lot of the world's largest web properties. This service is implemented in Java, with certain key components written in C++. Grace is currently working on introducing Rust into the system.

#### 🤔 Frequently Asked Questions

* What is most important to Grace about async Rust? Why?
    * Grace prioritizes performance, correctness, and reliability.
* What is least important to Grace about async Rust? Why?
    * XXX this is tricky
* What are key parts of Grace's background or story that distinguishes them from the other characters?
    * Grace has been working on network services for years in different languages. She's fairly familiar with existing tools and is often able to cobble together a fix to overcome obstacles.
    * At the same time, Grace is also accustomed to a certain suite of tools being available. She's used to monitoring her Java services 

### Niklaus: the developer building generic Rust libraries and frameworks

| Status | Owner |
| --- | --- |
| ⚠️ Draft ⚠️ | seanmonstar |

Niklaus is an open source developer building various generic libraries and frameworks. He is hoping to create successful open source projects that are widely used in all sorts of applications and which have a lively community around them. Niklaus is currently working on the following projects:

* **SLOW**, a Rust implementation of a fancy new communications protocol that is gaining in popularity.
* **Hyperactive**, a Rust web framework meant to make it easy to standup a web server and start serving requests. Probably exactly what Alan wants!

#### 🤔 Frequently Asked Questions

* What is most important to Niklaus about async Rust? Why?
    * Niklaus prioritizes being able to write things across runtimes and avoiding limiting the set of users that can consume his libraries.
    * In working on SLOW, he cares about high performance, because he knows many people won't use his library unless it can keep up.
    * For Hyperactive, he cares a lot about convenience.
* What is least important to Niklaus about async Rust? Why?
    * XXX
* What are key parts of Niklaus's background or story that distinguishes them from the other characters?
    * Niklaus is a hobbyist. He only has slices of time to do development. He wants to use those bits of time profitably and he wants to enjoy it.


### Barbara: embedded developer doing networking

| Status | Owner |
| --- | --- |
| ⚠️ Draft ⚠️ | ? |

Barbara is building a sensor grid system using Rust. The sensors communicate wirelessly to relay their results, and Barbara is responsible for the networking component. These sensors are relatively simple devices with limited resources, so she is coding in a `#[no_std]` environment and is very careful about things like allocation.

#### 🤔 Frequently Asked Questions

* What is most important to Barbara about async Rust? Why?
    * Barbara needs low-level control. She can't use typical tools.
    * She also cares about portability: few people work in her space, and she would like to be able to use as many tools and libraries as posible.
* What is least important to Barbara about async Rust? Why?
    * XXX
* What are key parts of Barbara's background or story that distinguishes them from the other characters?
    * Working in embedded.
