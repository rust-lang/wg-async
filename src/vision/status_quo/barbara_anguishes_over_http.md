# 😱 Status quo stories: Barbara Anguishes Over HTTP

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect people's experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Barbara is starting a new project, working together with Alan. They want to write a Rust library and as part of it they will need to make a few HTTP calls to various web services. While HTTP is part of the responsibilities of the library it is by no means the only thing the library will need to do.

As they are pair programming, they get the part of the library where HTTP will be involved and Alan asks Barbara, "OK, how do I make an HTTP request?".

As an experienced async Rust developer Barbara has been dreading this question from the start of the project. She's tempted to ask "How long do you have?", but she quickly gathers herself and starts to outline the various considerations. She starts with a relatively simple question: "Should we use an HTTP library with a sync interface or an async interface?".

Alan, who comes from a JavaScript background, remembers the transition from callbacks to async/await in that language. He assumes Rust is merely making its transition to async/await, and it will eventually be the always preferred choice. He hesitates and asks Barbara: "Isn't async/await always better?". Barbara, who can think of many scenarios where a blocking, sync interface would likely be better, weighs whether going done the rabbit-hole of async vs sync is the right way to spend their time. She decides instead to try to directly get at the question of whether they should use async for this particular project. She knows that bridging sync and async can be difficult, and so there's another question they need to answer first: "Are we going to expose a sync or an async interface to the users of our library?".

Alan, still confused about when using a sync interface is the right choice, replies as confident as he can: "Everybody wants to use async these days. Let's do that!". He braces for Barbara's answer as he's not even sure what he said is actually true.

Barbara replies, "If we expose an async API then we need to decide which async HTTP implementation we will use". As she finishes saying this, Barbara feels slightly uneasy. She knows that it is possible to use a sync HTTP library and expose it through an async API, but she fears totally confusing Alan and so decides to not mention this fact.

Barbara looks over at Alan and sees a blank stare on his face. She repeats the question: "So, which async HTTP implementation should we use?". Alan responds with the only thing that comes to his mind: "which one is the best?" to which Barbara responds "Well, it depends on which async runtime you're using". 

Alan, feeling utterly dejected and hoping that the considerations will soon end tries a new route out of this conversation: "Can we allow the user of the library to decide?". 

Barbara thinks to herself, "Oh boy, we could provide a trait that abstracts over the HTTP request and response and allow the user to provide the implementation for whatever HTTP library they want... BUT, if we ever need any additional functionality that an async runtime needs to expose - like async locks or async timers - we might be forced to pick an actual runtime implementation on behalf of the user... Perhaps, we can put the most popular runtime implementations behind feature flags and let the user chose that way... BUT what if we want to allow plugging in of different runtimes?"

Alan, having watched Barbara stare off into the distance for what felt like a half-hour, feels bad for his colleague. All he can think to himself is how Rust is so much more complicated that C#.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
    * What is a very mundane and simple decision in many other languages, picking an HTTP library, requires users to contemplate *many* different considerations.
    * There is no practical way to choose an HTTP library that will serve most of the ecosystem. Sync/Async, competing runtimes, etc. - someone will always be left out.
    * HTTP is a small implementation detail of this library, but it is a HUGE decision that will ultimately be the biggest factor in who can adopt their library.

### **What are the sources for this story?**
Based on the author's personal experience of taking newcomers to Rust through the decision making process of picking an HTTP implementation for a library.

### **Why did you choose [Barbara][] to tell this story?**
Barbara knows all the considerations and their consequences. A less experienced Rust developer might just make a choice even if that choice isn't the right one for them.

[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
