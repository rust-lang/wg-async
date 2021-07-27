# 🔮 The vision

[cc]: ./vision/characters.md
[Grace]: ./vision/characters/grace.md
[Alan]: ./vision/characters/alan.md
[gba]: ./vision/characters/grace.md#variant-a-networking-systems
[gba]: ./vision/characters/grace.md#variant-b-embedded
[sq]: ./vision/status_quo.md
[gsq]: ./vision/status_quo/gracy_deploys_her_service.md
[cok]: https://en.wikipedia.org/wiki/Curse_of_knowledge
[sf]: ./vision/shiny_future.md
[roadmap]: ./vision/roadmap.md

## What is this

We believe Rust can become one of the most popular choices for building distributed systems, ranging from embedded devices to foundational cloud services. Whatever they're using it for, we want all developers to love using Async Rust. For that to happen, we need to move Async Rust beyond the "MVP" state it's in today and make it accessible to everyone.

This document is a collaborative effort to build a shared vision for Async Rust. **Our goal is to engage the entire community in a collective act of the imagination:** how can we make the end-to-end experience of using Async I/O not only a pragmatic choice, but a *joyful* one?

## 🚧 Under construction! Help needed! 🚧

The first version of this document is not yet complete, but it's getting very close! We are in the process of finalizing the set of ["status quo"](./vision/status_quo.md) and ["shiny future"](./vision/shiny_future.md) stories and the details of the [proposed roadmap](./vision/roadmap.md). The current content however is believed to be relatively final, at this point we are elaborating and improving it.

## Where we are and where we are going

The "vision document" starts with a [cast of characters][cc]. Each character is tied to a particular Rust value (e.g., performance, productivity, etc) determined by their background; this background also informs the expectations they bring when using Rust. [Grace], for example, wants to keep the same level of performance she currently get with C, but with the productivity benefits of memory safety. [Alan], meanwhile, is hoping Rust will give him higher performance without losing the safety and ergonomics that he enjoys with garbage collected languages. 

For each character, we write ["status quo" stories][sq] that describe the challenges they face as they try to achieve their goals (and typically fail in dramatic fashion!), **These stories are not fiction.** They are an amalgamation of the real experiences of people using Async Rust, as reported to us by interviews, blog posts, and tweets. Writing these stories helps us gauge the cumulative impact of the various papercuts and challenges that one encounters when using Async Rust.

The ultimate goal of the vision doc, of course, is not just to tell us where we are now, but where we are going and how we will get there. For this, we include ["shiny future" stories][sf] that tell us how those same characters will fare in a few years time, when we've had a chance to improve the Async Rust experience.

## The vision drives the work

The vision is not just idle speculation. It is the central document that we use to organize ourselves. When we think about our [roadmap](./vision/roadmap.md) for any given year, it is always with the aim of moving us closer to the vision we lay out here.

## Involving the whole community

The async vision document provides a forum where the Async Rust community can plan a great overall experience for Async Rust users. Async Rust was intentionally designed not to have a "one size fits all" mindset, and we don't want to change that. Our goal is to build a shared vision for the end-to-end experience while retaining the loosely coupled, exploration-oriented ecosystem we have built.
