# 😱 Status quo

## What is this?

The "status quo" section contains a number of [narratives](#the-narratives) that illustrate the experience of using Async Rust today to achieve various kinds of tasks. These narratives highlight the [biggest problems](./status_quo/problems.md) that we see in Async Rust today and help to bring those problems to life.

## The narratives

The following narratives describe the experience of using Async Rust to build a number of different kinds of projects. These stories are fiction, but they are meant to be accurate representations of people's experiences; they are derived from the [various status quo stories that were submitted while drafting the vision doc](../submitted_stories/status_quo.md) along with informal conversations and other sources. 

- [Authoring the DistriData service](./status_quo/DistriData.md)
    - Implementing a web service that stores and replicates data on behalf of its clients.
- [Creating a library for the SLOW protocol](./status_quo/slow.md)
    - Creating a network protocol as a library on crates.io meant to be used across all kinds of servers.
- [Developing MonsterMesh, a system of embedded sensors](./status_quo/monster_mesh.md)
    - Using Async Rust on tiny nodes that run without an operating system, memory allocator, or other such niceties.
- [Implementing a base64 decoding library](./status_quo/base64.md)
    - Creating a reusable library meant to be used in both embedded and non-embedded environments.
