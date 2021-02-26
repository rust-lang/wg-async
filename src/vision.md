# 🔮 The vision

## What is this

This section lays out a general vision and set of goals for Async I/O in Rust. This document is being written in January of 2021. The core pieces of async-await are stable, but there remains a lot of work for it to feel like a "first class" feature of Rust. 

This section describes three things:

* The **design tenets**, or principles, that we are using to drive our work on Async I/O in Rust;
* Descriptions of how writing Async I/O code in Rust **feels today**;
* Descriptions of how we think writing Async I/O code in Rust **should feel eventually**.

## Status and schedule

**This document is currently being formed.** 
We expect to publish the first version on March 31.
Once that version is published, we intend to revisit the document once per quarter to keep it up to date.

| Phase | Date |
| --- | --- |
| **Drafting initial stories and proposals** | **until March 31** |
| 2021 Q1 revision published | March 31 |
| 2021 Q2 revision process begins | June 14 - June 30 |
| 2021 Q2 revision published | June 30 |
| 2021 Q3 revision process begins | Sep 13 - Sep 30 |
| 2021 Q3 revision published | Sep 30 |
| 2021 Q4 revision process begins | Nov 29 - Dec 10 |
| 2021 Q4 revision published | Dec 10 |

## Think big -- too big, if you have to

You'll notice that the ideas in this document are **maximalist and ambitious**. They stake out an opinionated position on how the ergonomics of Async I/O should feel. This position may not, in truth, be attainable, and for sure there will be changes along the way. Sometimes the realities of how computers actually work may prevent us from doing all that we'd like to. That's ok. This is a dream and a goal.

## The vision drives the work

The vision is not just idle speculation. It is the central document that we use to organize ourselves. When we think about our [roadmap](./roadmap.md) for any given year, it is always with the aim of moving us closer to the vision we lay out here. 

## This is a group effort

As the leads of the Async Foundation group, Niko and Tyler are driving and organizing this document. But writing it and shaping it is a group effort. If you think there is a part of the async experience that is not reflected here, we want to hear from you! Check out the [How to vision doc] for more details on how to contribute.

[How to vision doc]: ./vision/how_to_vision_doc.md
