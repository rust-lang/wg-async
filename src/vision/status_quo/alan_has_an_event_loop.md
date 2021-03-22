# 😱 Status quo stories: Alan has an external event loop and wants to use futures/streams

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

As a first Rust Project, Alan decides to program his own IRC Client.

Since it is Alans first Project in Rust, it is going to be a private one. He is going to use it on is Mac, so he decides to go with the cocoa crate to not have to learn any Framework specific quirks. This way Alan can get a feel of Rust itself.

### Alans hopes and dreams
Despite a learning curve, he managed to creating a first window and have some buttons and menus works. After the initialisation is done, the App hand over control to [CFRunLoop::Run](https://developer.apple.com/documentation/corefoundation/1542011-cfrunlooprun?language=occ).

Once Alan is happy whit his Mock UI, he wants to hook it up with some actual features. He is happy to learn that Rust has Features he already knows.
* Promises => Futures
* Observables => Streams.

Alan smiles, thinking he knows what and more importantly how to do this.

### First time dealing with runtimes

Unfortunately, coming from frameworks like Angular or Node.js, Alan is not used that he himself is responsible for driving the progress of Async/Streams. 

After reading up about Runtimes, his mental image of a runtime is something like: 

```rust
impl Runtime {
    fn run() {
        while !self.tasks.is_empty() {
            while let Some(task) = self.awoken_tasks.pop() {
                task.poll();
                //... remove finished task from 'tasks'
            }
        }
    }
}
```

Coming from Single-Threaded Angular development, Alan decides to limit his new App to Single-Threaded. He does not feel like learning about Send/Sync/Mutex as well as struggling with the borrow checker.

On top of that, his App is not doing any heavy calculation so he feels async should be enough to not block the main thread too bad and have a hanging UI.

### Fun time is over

Soon Alan realises that he cannot use any of those runtimes because they all take control of the thread and block. The same as the OS Event loop.

Alan spends quite some time to look through several runtime implementations. Ignoring most internal things, all he wants is a runtime that looks a bit like this:

```rust
impl Runtime {
    fn make_progress() {
        while let Some(task) = self.awoken_tasks.pop() {
            task.poll();
            //... remove finished task from 'tasks'
        }
    }
    fn run() {
        while !self.tasks.is_empty() {
            self.make_progress();
        }
    }
}
```

It could be soo easy. Unfortunately he does not find any such solution. Having already looked through quite a bit of low level documentation and runtime code, Alan thinks about implementing his own runtime...

...but only for a very short time. Soon after looking into it, he finds out that he has to deal with ```RawWakerVTable```, ```RawWaker```, ```Pointers```. Worst of all, he has to do that without the safety net of the rust compiler, because this stuff is ```unsafe```.

Reimplementing the OS Event Loop is also not an option he wants to take. See [here](https://developer.apple.com/documentation/appkit/nsapplication)
>Override run() if you want the app to manage the main event loop differently than it does by default. (This a critical and complex task, however, that you should only attempt with good reason).


### The cheap way out

Alan gives up and uses a runtime in a seperate thread from the UI. This means he has to deal with the additional burden of syncing and he has to give up the frictionless use of some of the patterns he is accustomed to by treating UI events as ```Stream<Item = UIEvent>```.

## 🤔 Frequently Asked Questions


* **What are the morals of the story?**
    * Even though you come from a language that has async support, does not mean you are used to selecting und driving a runtime.
    * It should be possible to integrate runtimes into existing Event loops.
* **What are the sources for this story?**
    * The authors own experience working on a GUI Framework (very early stage)
    * Blog post: [Integrating Qt events into Actix and Rust](https://www.rubdos.be/corona/qt/rust/tokio/actix/2020/05/23/actix-qt.html)
* **Why did you choose Alan to tell this story?**
    * The story deals about UI event loops, but the other characters could run into similar issues when trying to combine event loops from different systems/frameworks.
* **Is this Apple specific?**
    * No! You have the same issue with other OSs/Frameworks that don't already support Rust Async.
* **How would this story have played out differently for the other characters?**
    * Since this is a technical and not a skill or experience issue, this would play out similar for other Characters. Although someone with deep knowledge of those Event loops, like Grace, might be more willing to re-implement them.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
