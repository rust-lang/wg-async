# 🙋‍♀️ Cast of characters

## Alan: the startup guy trying to stand-up a web stack quickly

| Status | Owner |
| --- | --- |
| ⚠️ Draft ⚠️ | nikomatsakis |

Alan is a "full stack" JavaScript developer. He's bounced around from startup to startup building things both in node.js and doing front-end work. Alan was recently hired at `fido.io`, a pioneering dog-food delivery business that uses a combination of deep learning and [IoT] to select the optimum dog food for your pet's health. Fido has chosen to build their networking stack in Rust. They were already using Rust for their deep learning code and they wanted to keep to a single language. For the time being, their needs are relatively simple. They want to stand up a server that serves up pages and connects to various back-end and cloud services.

[IoT]: https://en.wikipedia.org/wiki/Internet_of_things

<details><summary>🤔 <b>Frequently Asked Questions</b></summary>
<p>

* XXX
</p>
</details>

## Grace: the principal engineer hacking on T4, a new storage service

| Status | Owner |
| --- | --- |
| ⚠️ Draft ⚠️ | nikomatsakis |

Grace works on T4, a cloud storage service that is used in a lot of the world's largest web properties. She is a principal engineer who has been building high-performance networking systems in C for a number of years. T4 recently switched to Rust for the new component they are building. They are hoping it will help them avoid security vulnerabilities without compromising on performance. Huma has been using Rust for a while now and she's starting to feel fairly comfortable with it. The new T4 component is getting ready to go into production soon; she's hoping they'll be able to meet their performance goals.

<details><summary>🤔 <b>Frequently Asked Questions</b></summary>
<p>

* XXX
</p>
</details>

## Niklaus: the developer building generic Rust libraries and frameworks

| Status | Owner |
| --- | --- |
| ⚠️ Draft ⚠️ | nikomatsakis |

Niklaus is an open source developer building various generic libraries and frameworks. He is hoping to create successful open source projects that are widely used in all sorts of applications and which have a lively community around them. Niklaus is currently working on the following projects:

* **SLOW**, a Rust implementation of a fancy new communications protocol that is gaining in popularity.
* **Hyperactive**, a Rust web framework meant to make it easy to standup a web server and start serving requests. Probably exactly what Alan wants!

<details><summary>🤔 <b>Frequently Asked Questions</b></summary>
<p>

* XXX
</p>
</details>

## Barbara: embedded developer doing networking

| Status | Owner |
| --- | --- |
| ⚠️ Draft ⚠️ | nikomatsakis |

Barbara is building a sensor grid system using Rust. The sensors communicate wirelessly to relay their results, and Barbara is responsible for the networking component. These sensors are relatively simple devices with limited resources, so she is coding in a `#[no_std]` environment and is very careful about things like allocation.

<details><summary>🤔 <b>Frequently Asked Questions</b></summary>
<p>

* XXX
</p>
</details>

