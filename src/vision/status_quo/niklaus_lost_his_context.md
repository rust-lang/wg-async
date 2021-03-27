# 😱 Status quo stories: Niklaus lost his context!

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Niklaus heard about a project to reimplement a deprecated browser plugin using Rust and WASM. This old technology had the ability to load resources over HTTP; so it makes sense to try and implement that functionality using the Fetch API. He looks up the documentation of `web_sys` and realizes that he needs to take a thing called a `Future` out of the call to `fetch`, do some things to it in Rust, and then send the future off to `spawn_local` as such:

```rust
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

fn load_image(src: &url) {
    spawn_local(async move {
        let url = src.to_string();
        let request = make_request(url);
                      //just pretend it's here
        window().unwrap().fetch_with_request(&request).await;
        log::error!("It worked");
    });
}
```

Niklaus adds calls to `load_image` where appropriate, sees the message pop up in the console, and figures it's time to now actually do something to that loaded image. At this point, it's important to note that many parts of this project pass around "contexts" - bundles of borrowed state that various parts of the reimplementation are free to alter. (This is necessary because this plugin had multiple scripting runtimes in it.) In synchronous code, this was perfectly fine. However, when we start integrating the function into the player...

```rust
use crate::{Index, Context}
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use crate::parse_png;

fn load_image(src: &url, inside_of: Index<Node>, context: &mut Context<'_>) {
    spawn_local(async move {
        let url = src.to_string();
        let request = //still elided
        let data = window().unwrap().fetch_with_request(&request).await.unwrap().etc.etc.etc;
        let image = parse_png(data, context);
        context.display_tree.add_child_node(inside_of, image);
    });
}
```

Well, that stopped compiling. After looking through far too many compiler error messages, Niklaus realizes that the problem is the context itself. You see, whenever you close over some borrowed state, your closure's lifetime is bounded by that borrow. This is reasonable for closures, as there are plenty of contexts where you would pass such a closure and have it called immediately. However, the whole point of futures is that they are *not* called immediately - you hold them inside of a thing called an "executor", which typically needs futures that own their contents.

At this point, Niklaus is looking at either rewriting a good chunk of the program to not use contexts, or rewriting his own code to sidestep the problem. He chooses the latter, by holding a reference to the thing that produces the context and then updating it after the async portion of the task has concluded:

```use crate::{Index, Player}
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use crate::parse_png;

fn load_image(src: &url, inside_of: Index<Node>, player: Arc<Mutex<Player>>) {
    spawn_local(async move {
        let url = src.to_string();
        let request = //still elided
        let data = window().unwrap().fetch_with_request(&request).await.unwrap().etc.etc.etc;

        player.lock().unwrap().update(|context| {
            let image = parse_png(data, context);
            context.display_tree.add_child_node(inside_of, image);
        });
    });
}
```

It works, but it causes problems. Async and contextual code can't really coexist; so async code always has to live at the periphery of the program. More complicated async behaviors, such as reading the image in chunks and progressively decoding it, requires a delicate dance of await followed by update and so on. Niklaus tells Alan about this and Alan says it has something to do with colored functions, something he isn't quite familiar about.

## 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * Borrowing means that Rust has three function colors, not two:
      * Sync, which can both own and borrow it's parameters
      * Owned Async, which can only own parameters
      * Borrowed Async, which can both own and borrow parameters, but gains a lifetime
    * Non-`'static` Futures are of limited use as lifetimes are tied to the sync stack. I think you can use `block_on` with them but that's about it.
    * Borrowed Async functions would be useful if there was a way to have a future whose borrows were scoped to the moment they are `poll`ed.
* **What are the sources for this story?**
    * This is personal experience. Specifically, I had to do [almost exactly this dance][RuffleAsync] in order to get fetch to work in Ruffle.
    * I have omitted a detail from this story: in Ruffle, we use a GC library (`gc_arena`) that imposes a special lifetime on all GC references. This is how the GC library upholds it's memory safety invariants, but it's also what forces us to pass around contexts, and once you have that, it's natural to start putting even non-GC data into it. It also means we can't hold anything from the GC in the Future as we cannot derive it's `Collect` trait on an anonymous type.
* **Why did you choose Niklaus to tell this story?**
    * It was the character that fit the best. Lifetimes on closures is already non-obvious to new Rust programmers and using them in the context of Futures.
    * I suppose if I wasn't talking about an issue I had on a reimplementation of Flash Player, I might have chosen Grace.
    * I personally don't identify with Niklaus, I'm probably half Grace, half Alan.
* **How would this story have played out differently for the other characters?**
    * Alan had similar struggles to Niklaus, but interpreted them from a different angle (e.g. "colored functions" rather than "fighting the borrow checker")
    * Grace might also have been surprised, but ultimately understands the limitation
    * Barbara already knew about Futures and 'static

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
[RuffleAsync]: https://github.com/ruffle-rs/ruffle/blob/master/core/src/loader.rs
