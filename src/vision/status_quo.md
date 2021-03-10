# 😱 Status quo stories

## What is this

The "status quo" stories document the experience of using Async Rust today. Each story narrates the challenges encountered by [one of our characters][cc] as they try (and typically fail in dramatic fashion) to achieve their goals.

[cc]: ./characters.md

Writing "status quo" stories helps us compensate for the [curse of knowledge][cok] -- in particular, the folks working on Async Rust tend to be experts in Async Rust. We've gotten used to the [workarounds] required to be productive, and we know the little tips and tricks that can get you out of a jam. 

Writing the "status quo" stories gives us a concrete way to put ourselves in [our characters'][cc] shoes. They help us to estimate the cumulative impact all the paper cuts can have on someone still learning their way around. This gives us the data we need to prioritize.

[cok]: https://en.wikipedia.org/wiki/Curse_of_knowledge
[workarounds]: https://rust-lang.github.io/async-book/07_workarounds/01_chapter.html

### Based on a true story

These stories are not made up. They are always based on real-life experiences of actual people. Each story contains a "Frequently Asked Questions" section, and that will include notes the sources used to create the story. In some cases, it may link to notes or summaries in the [conversations] section, though that is not required. The "Frequently Asked Questions" section also contains a summary of what the "morals" of the story are (i.e., what are the key takeaways), along with answers to questions that people have raised along the way.

[conversations]: ../conversations.md

### The stories provide data we use to prioritize, not a prioritization itself

**Just because a user story is represented here doesn't mean we're going to be able to fix it right now.** Some of these user stories will indicate more severe problems than others. As we consider the stories, we'll select some subset to try and address; that choice is reflected in the [roadmap].

[roadmap]: ./roadmap.md

### Help wanted!

This is not a static document! There are lots of ways you can help to expand it! Take a look at the [How to Vision Doc](./how_to_vision_doc.md) for more details.

[Alan]: ./characters.md#alan-the-startup-guy-trying-to-stand-up-a-web-stack-quickly
[Grace]: ./characters.md#grace-the-principal-engineer-hacking-on-a-data-storage-service
[Niklaus]: ./characters.md#niklaus-the-developer-building-generic-rust-libraries-and-frameworks
[Barbara]: ./characters.md#barbara-embedded-developer-doing-networking
[Volunteer wanted]: ./how_to_vision_doc.md#how-can-i-finish-or-add-a-section-to-the-document

## Alan has trouble getting started

| Character | Shiny Future |
| --- | --- |
| [Alan] | [Alan has a blast standing up a web server](./shiny_future.md#alan-has-a-blast-standing-up-a-web-server) |

Outline:

* At first, [Alan] can't find docs about how to do async stuff
* Alan finds learning Rust challenging enough, learning async at the same time is even harder ([cite](https://twitter.com/richardsabow/status/1345815115745140736))
* Alan has trouble picking a runtime, it's kind of overwhelming -- [what if he picks the wrong one?](https://twitter.com/EchoRior/status/1359965313979346944)
* Alan is confused because adding async/await doesn't seem to be getting him concurrency
* Alan tries to connect to a database but the docs aren't very comprehensive
* Alan doesn't like the web framework he's using, but the one he likes requires a different runtime, and switching between them is a real pain
* Alan quits and becomes a [Bob Ross] groupy

[Bob Ross]: https://www.pbs.org/show/best-joy-painting/

#### 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * Many people cite that picking a runtime is a stressful choice.
    * People want to be able to 'mix and match' the runtime, web framework, and other libraries and have things just work.
    * Information on async-await is hard to find and not yet well integrated into standard Rust learning materials.
    * Rust's async model is different from other languages (JavaScript, C#) and that can be confusing.
    * Learning async Rust is strictly more work than learning sync Rust, which is already harder than we'd like.
    * The async book as written gives way too many low-level details way too early.
* **Are you just making this stuff up?**
    * Not at all. This is based on a number of sources, including tweets but also personal conversations.
    * Some sources can be found in the conversations page:
        * [2021-02-12 Twitter thread](../conversations/2021-02-12-Twitter-Thread.md)
* **Why doesn't Alan go to the tutorial for some specific runtime?**
    * It's true that there are a number of excellent tutorials out there, like the one from [tokio](https://tokio.rs/tokio/tutorial) and [async-std](https://book.async.rs/tutorial/index.html). Unfortunately, Alan doesn't know about those runtimes yet! Alan wants to learn Rust, so that's where he started.

## Alan finds the experience of writing async code is a lot more painful than sync code

| Character | Shiny Future |
| --- | --- |
| [Alan] | not yet written |

Outline:

* [Alan] gets inscrutable backtraces
* Alan tries to write a recursive function and gets a weird error
* Alan gets confusing diagnostics because of incompatible versions of the futures crate
* Alan gets a confusing compiler error about a variable being live across an await point that doesn’t seem to be live
* Bobs gets an error message that exceeds the memory limits in his terminal program, crashes his computer, and forces him to reboot

#### 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * (Explain your key points here)
* **Are you just making this stuff up?**
    * (Cite important sources here; feel free to add files into the conversations folder for more details)

## Grace tries to write a stream and pin breaks her mind

| Character | Shiny Future |
| --- | --- |
| [Grace] | [Grace discovers the joy of generators](./shiny_future.html#grace-discovers-the-joy-of-generators) |

Outline:

* We need to make this better -- pick some specific problem Grace was trying to solve
* She winds up implementing a stream and has to use pin and it's hard
* we don’t currently provide any language support for doing this
* so you have to work with pin
* it is very confusing and nobody really understands it
* she doesn’t get parallel like she wanted
* she switches to a stream of futures 
* it kind of works but it’s a pain to maintain and she doesn’t like fixing bugs in that code
    * she transfers to another team and somebody else deletes the file, starts over, and repeats her experience

#### 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * (Explain your key points here)
* **Are you just making this stuff up?**
    * (Cite important sources here; feel free to add files into the conversations folder for more details)

## Grace deploys her service and hits obstacles

| Character | Shiny Future |
| --- | --- |
| [Grace] | [Grace deploys her service and is able to fix problems] |

[Grace deploys her service and is able to fix problems]: ./shiny_future.md#grace-deploys-her-service-and-is-able-to-fix-problems

When examining her service metrics, Grace notices tail latencies in the P99 that exceed their target. She identifies GC in the routing layer as the culprit. Grace follows industry trends and is already aware of Rust and its ecosystem at a high level. She decides to investigate rewriting the routing service in Rust.

To meet throughput requirements, Grace has already decided to use a thread-per-core model and minimize cross-thread communication. She explores available ecosystem options and finds no option that gets her exactly what she is looking for out of the box. However, she can use Tokio with minimal configuration to achieve her architecture.

A few months of frantic hacking follow.

<img src="https://media.giphy.com/media/ule4vhcY1xEKQ/giphy.gif" alt="montage of cats typing" width=200></img>
 
Soon enough, she and her team have a proof of concept working. They run some local stress tests and notice that 5% of requests hang and fail to respond. The client eventually times out. She cannot reproduce this problem when running one-off requests locally. It only shows up when sending above 200 requests-per-second. 

She realizes that she doesn't have any tooling to give her insight into what's going on. She starts to add lots of logging, attempting to tie log entries to specific connections. Using an operating system tool, she can identify the socket addresses for the hung connections, so she also includes the socket addresses in each log message. She then filters the logs to find entries associated with hung connections. Of course, the logs only tell her what the connection managed to do successfully; they don't tell her why it stopped -- so she keeps going back to add more logging until she can narrow down the exact call that hangs.

Eventually, she identifies that the last log message is right before authenticating the request. An existing C library performs authentication, integrated with the routing service using a custom future implementation. She eventually finds a bug in the implementation that resulted in occasional lost wake-ups.

She fixes the bug. The service is now working as expected and meeting Grace's performance goals.

#### 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * When coming from a background of network engineering, users will bring their own design choices around architecture.
        * Examples: [seastar](http://seastar.io/) and [Glommio](https://www.datadoghq.com/blog/engineering/introducing-glommio/)
    * There is a lack of debugging tools for async.
    * Writing futures by hand is error prone.
* **Are you just making this stuff up?**
    * This is based on user conversations.

## Grace wants to use io-uring

| Character | Shiny Future |
| --- | --- |
| [Grace] | not yet written |

Outline:

* there are existing crates, which to use?
    * [ringbahn](https://github.com/ringbahn/ringbahn)
    * [glommio](https://github.com/DataDog/glommio)
    * [rio](https://crates.io/crates/rio) -- "but users should beware that use-after-free bugs are still possible without unsafe when using rio."
    * [tokio-rs/io-uring](https://github.com/tokio-rs/io-uring)
    * [quininer/ritsu](https://github.com/quininer/ritsu)
    * ...and probably more
* the AsyncRead trait is not a perfect fit

#### 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * (Explain your key points here)
* **Are you just making this stuff up?**
    * (Cite important sources here; feel free to add files into the conversations folder for more details)

## Niklaus goes to write SLOW but it’s annoying

| Character | Shiny Future |
| --- | --- | 
| [Niklaus] | not yet written |

Outline:

* async fns in traits maybe come into this story?
* to make it work across many runtimes he has to juggle a lot of feature gates 
* the AsyncRead trait is in the futures crate, but he can’t tell how stable that crate is — can he rely on it in his public interface? Can he call that 1.0?
    * futures crate makes a breaking change, is he screwed?
* he has to copy and paste a bunch of random utilities that everyone seems to use but which aren’t defined in a shared crate that has a stable version
    * xxx some of them are in futures??
* maybe he wants to target WASM?

#### 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * (Explain your key points here)
* **Are you just making this stuff up?**
    * (Cite important sources here; feel free to add files into the conversations folder for more details)


## Niklaus builds his web framework

| Character | Shiny Future |
| --- | --- |
| [Niklaus] | not yet written |

Outline:

* He is frustrated by lack of async trait support
* He uses the `async-trait` procedural macro but it feels weird to have that in a public, stable API.
* As with the SLOW library, he would to make it independent of a specific runtime, but that's very difficult

#### 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * (Explain your key points here)
* **Are you just making this stuff up?**
    * (Cite important sources here; feel free to add files into the conversations folder for more details)

## Barbara tries to use the embedded runtime

| Character | Shiny Future |
| --- | --- |
| [Barbara] | not yet written |

Outline:

* She doesn’t want to use allocation and `dyn Trait` but async fns in traits are not available on stable rust
* She is not wanting to use nightly

#### 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * (Explain your key points here)
* **Are you just making this stuff up?**
    * (Cite important sources here; feel free to add files into the conversations folder for more details)

