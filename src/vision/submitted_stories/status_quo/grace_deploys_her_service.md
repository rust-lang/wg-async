# 😱 Status quo stories: Grace deploys her service and hits obstacles

[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md

[Grace deploys her service and is able to fix problems]: ./shiny_future.md#grace-deploys-her-service-and-is-able-to-fix-problems

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today.

## The story

When examining her service metrics, [Grace] notices tail latencies in the P99 that exceed their target. She identifies GC in the routing layer as the culprit. Grace follows industry trends and is already aware of Rust and its ecosystem at a high level. She decides to investigate rewriting the routing service in Rust.

To meet throughput requirements, Grace has already decided to use a thread-per-core model and minimize cross-thread communication. She explores available ecosystem options and finds no option that gets her exactly what she is looking for out of the box. However, she can use Tokio with minimal configuration to achieve her architecture.

A few months of frantic hacking follow.

<img src="https://media.giphy.com/media/ule4vhcY1xEKQ/giphy.gif" alt="montage of cats typing" width=200></img>
 
Soon enough, she and her team have a proof of concept working. They run some local stress tests and notice that 5% of requests hang and fail to respond. The client eventually times out. She cannot reproduce this problem when running one-off requests locally. It only shows up when sending above 200 requests-per-second. 

She realizes that she doesn't have any tooling to give her insight into what's going on. She starts to add lots of logging, attempting to tie log entries to specific connections. Using an operating system tool, she can identify the socket addresses for the hung connections, so she also includes the socket addresses in each log message. She then filters the logs to find entries associated with hung connections. Of course, the logs only tell her what the connection managed to do successfully; they don't tell her why it stopped -- so she keeps going back to add more logging until she can narrow down the exact call that hangs.

Eventually, she identifies that the last log message is right before authenticating the request. An existing C library performs authentication, integrated with the routing service using a custom future implementation. She eventually finds a bug in the implementation that resulted in occasional lost wake-ups.

She fixes the bug. The service is now working as expected and meeting Grace's performance goals.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* When coming from a background of network engineering, users will bring their own design choices around architecture.
    * Examples: [seastar](http://seastar.io/) and [Glommio](https://www.datadoghq.com/blog/engineering/introducing-glommio/)
* There is a lack of debugging tools for async.
* Writing futures by hand is error prone.

### **What are the sources for this story?**
This is based on the experiences of helping a tokio user to diagnose a bug in their code.

### **Why did you choose Grace to tell this story?**
* The actual user who experienced this problem fit the profile of Grace.
* The story is focused on the experience of people aiming to use workflows they are familiar with from C in a Rust setting.

### **How would this story have played out differently for the other characters?**
Alan or Niklaus may well have had a much harder time diagnosing the problem due to not having as much of a background in systems programming. For example, they may not have known about the system tool that allowed them to find the list of dangling connections.

### **Could Grace have used another runtime to achieve the same objectives?**
* Maybe! But in this instance the people this story is based on were using tokio, so that's the one we wrote into the story.
* (If folks want to expand this answer with details of how to achieve similar goals on other runtimes that would be welcome!)

