# 😱 Status quo stories: Alan wants to migrate a web server to Rust


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

## The story

### Is Rust ready for the web?

Alan has been following the [arewewebyet site](https://www.arewewebyet.org/) for quite some time. He is a Typescript full-stack developer and follows the project in order to know when it would be sensible to migrate the backend of a web application he's responsible for. Alan loves Rust and has used it for some tasks that didn't quite need async routines. Since `arewewebyet` is [an official Rust language project](https://github.com/rust-lang/arewewebyet), he trusts their reviews of several web frameworks, tools, libraries, etc.

Alan was thrilled during the 2020 Xmas holiday. It turns out that at that time [Rust was declared to be web ready](https://github.com/rust-lang/arewewebyet/pull/309)! Alan takes this is a sign that not only is Rust great for web servers, but also a confirmation that async features have matured and stabilised. For, how can a language be web ready and not fully support asynchronous tasks? 

Alan's point of reference are the Golang and Javascript languages. They were both created for web servers and clients. They also support async/await natively. At the same time, Alan is not aware of the complexities that these languages are "hiding" from him.


### Picking a web server is ok
Golang native http server is nice but, as a Typescript developer, Alan is also used to dealing with "Javascript fatigue". Javascript developers often use this term to refer to a fast-pace framework ecosystem, where every so often there is the "new" thing everybody else is migrating to. Similarly, Javascript engineers are used to having to pick from a myriad of options within the vast npm ecosystem. And so, the lack of a web sever in Rust's standard library didn't surprise him. The amount of options didn't overwhelm him either. 

The [arewewebyet site](https://www.arewewebyet.org/) mentions four good web servers. Alan picks Tide because the interfaces and the emphasis on middleware reminds him of Nodejs' Express framework.


### The first endpoint
Alan sets up all the boilerplate and is ready to write the first endpoint. He picks `PUT /support-ticket` because it barely has any logic in it. When a request arrives, the handler only makes a request to Zendesk to create a support ticket. The handler is stateless and has no middleware.

The [arewewebyet site](https://www.arewewebyet.org/) doesn't recommend a specific http client, so Alan searches for one in crates.io. He picks reqwest simply because it's the most popular.

Alan combines the knowledge he has from programming in synchronous Rust and asynchronous Javascript to come up with a few lines that should work. If the compiler is happy, then so is he!

### First problem: incompatible runtimes

The first problem he runs into is very similar to the one described in [the compiler trust story](alan_started_trusting_the_rust_compiler_but_then_async.md#fractured-futures-fractured-trust): `thread 'main' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime`.

In short, Alan has problems because Tide is based on `std-async` and reqwest on the latest version of `tokio`. This is a real pain for Alan as he has now to change either the http client or the server so that they use the same runtime.

He decides to switch to Actix web.

### Second problem: incompatible versions of the same runtime

Alan migrates to Actix web and again the compiler seems to be happy. To his surprise, the same problem happens again. The program panics with the message as before: `there is no reactor running, must be called from the context of a Tokio 1.x runtime`. He is utterly puzzled as Actix web is based on Tokio just like reqwest. Didn't he just fix problem number 1?

It turns out that the issue is that Alan's using v0.11.2 of reqwest, which uses tokio v1, and v3.3.2 of actix-web, which uses tokio v0.3.

The solution to this problem is then to dig into all the versions of `reqwest` until he finds one which uses the same version of tokio.

### Can Alan sell the Rust migration to his boss?

This experience has made Alan think twice about whether Rust is indeed web ready. On the one hand, there are very good libraries for web servers, ORMs, parsers, session management, etc. On the other, Alan is fearful that in 2/3/6 months time he has to develop new features with libraries that already exist but turn out to be incompatible with the runtime chosen at the beginning of the project.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* Rust's ecosystem has a lot of great components that may individually be ready for the web, but combining them is still a fraught proposition. In a typical web server project, dependencies that use async features form an intricate web which is hard to decipher for both new and seasoned Rust developers. Alan picked Tide and reqwest, only to realise later that they are not compatible. How many more situations like this will he face? Can Alan be confident that it won't happen again? New users especially are not accustomed to having to think about what "runtime" they are using, since there is usually not a choice in the matter.
* The situation is so complex that it's not enough knowing that all dependencies use the same runtime. They all have to actually be compatible with the same runtime **and** version. Newer versions of reqwest are incompatible with the latest stable version of actix web (verified at the time of writing)
* Developers that need a stable environment may be fearful of the complexity that comes with managing async dependencies in Rust. For example, if reqwest had a security or bug fix in one of the latest versions that's not backported to older ones, Alan would not be able to upgrade because actix web is holding him back. He has in fact to wait until **ALL** dependencies are using the same runtime to apply fixes and upgrades.

### **What are the sources for this story?**
Personal experience of the author.

### **Why did you choose Alan to tell this story?**
As a web developer in GC languages, Alan writes async code every day. A language without stable async features is not an option.

### **How would this story have played out differently for the other characters?**
Learning what async means and what it entails in a codebase is usually hard enough. Niklaus would struggle to learn all that while at the same time dealing with the many gotchas that can happen when building a project with a lot of dependencies.

Barbara may be more tolerant with the setup since she probably knows the rationale behind keeping Rust's standard library lean and the need for external async runtimes.

### **How would this story have played out differently if Alan came from another GC'd language?**
Like the trust story, it would be very close, since all other languages (that I know of) provide async runtimes out of the box and it's not something the programmer needs to concern themselves with.
