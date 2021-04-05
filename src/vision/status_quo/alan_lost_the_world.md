# 😱 Status quo stories: Alan lost the world!

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Alan heard about a project to reimplement a deprecated browser plugin using Rust and WASM. This old technology had the ability to load resources over HTTP; so it makes sense to try and implement that functionality using the Fetch API. Alan looks up the documentation of `web_sys` and realizes they need to...

1. Call one of the [`fetch` methods][WasmFetch], which returns a [`Promise`](https://docs.rs/js-sys/0.3.50/js_sys/struct.Promise.html)
2. [Convert the `Promise`][WasmJsFuture] into a Rust thing called a `Future`
3. `await` the `Future` in an `async` function
4. Do whatever they want with the resulting data

```rust,ignore
use web_sys::{Request, window};

fn make_request(src: &url) -> Request {
    // Pretend this contains all of the complicated code necessary to
    // initialize a Fetch API request from Rust
}

async fn load_image(src: String) {
    let request = make_request(&url);
    window().unwrap().fetch_with_request(&request).await;
    log::error!("It worked");
}
```

Alan adds calls to `load_image` where appropriate. They realize that nothing is happening, so they look through more documentation and find a thing called [`spawn_local`][WasmSpawn]. Once they pass the result of `load_image` into that function, they see their log message pop up in the console, and figure it's time to actually do something to that loaded image data.

At this point, Alan wants to put the downloaded image onto the screen, which in this project means putting it into a `Node` of the current `World`. A `World` is a bundle of global state that's passed around as things are loaded, rendered, and scripts are executed. It looks like this:

```rust,ignore

/// All of the player's global state.
pub struct World<'a> {
    /// A list of all display Nodes.
    nodes: &'a mut Vec<Node>,

    /// The last known mouse position.
    mouse_pos &'a mut (u16, u16),

    // ...
}
```

In synchronous code, this was perfectly fine. Alan figures it'll be fine in async code, too. So Alan adds the world as a function parameter and everything else needed to parse an image and add it to our list of nodes:

```rust,ignore
async fn load_image(src: String, inside_of: usize, world: &mut World<'_>) {
    let request = make_request(&url);
    let data = window().unwrap().fetch_with_request(&request).await.unwrap().etc.etc.etc;
    let image = parse_png(data, context);

    let new_node_index = world.nodes.len();
    if let Some(parent) = world.nodes.get(inside_of) {
        parent.set_child(new_node_index);
    }
    world.nodes.push(image.into());
}
```

Bang! Suddently, the project stops compiling, giving errors like...

```ignore
error[E0597]: `world` does not live long enough
  --> src/motionscript/globals/loader.rs:21:43
```

Hmm, okay, that's kind of odd. We can pass a `World` to a regular function just fine - why do we have a problem here? Alan glances over at `loader.rs`...

```rust,ignore
fn attach_image_from_net(world: &mut World<'_>, args: &[Value]) -> Result<Value, Error> {
    let this = args.get(0).coerce_to_object()?;
    let url = args.get(1).coerce_to_string()?;

    spawn_local(load_image(url, this.as_node().ok_or("Not a node!")?, world))
}
```

Hmm, the error is in that last line. `spawn_local` is a thing Alan had to put into everything that called `load_image`, otherwise his async code never actually did anything. But why is this a problem? Alan can borrow a `World`, or anything else for that matter, inside of async code; and it should get it's own lifetime like everything else, right?

Alan has a hunch that this `spawn_local` thing might be causing a problem, so Alan reads the documentation. The function signature seems particuarly suspicious:

```rust,ignore
pub fn spawn_local<F>(future: F) 
where
    F: Future<Output = ()> + 'static
```

So, `spawn_local` only works with futures that return nothing - so far, so good - and are `'static`. Uh-oh. What does that last bit mean? Alan asks Barbara, who responds that it's the lifetime of the whole program. Yeah, but... the async function is part of the program, no? Why wouldn't it have the `'static` lifetime? Does that mean all functions that borrow values aren't `'static`, or just the async ones?

Barbara explains that when you borrow a value in a closure, the closure doesn't gain the lifetime of that borrow. Instead, the borrow comes with it's own lifetime, separate from the closure's. The only time a closure can have a non-`'static` lifetime is if one or more of its borrows is *not* provided by it's caller, like so:

```rust,ignore
fn benchmark_sort() -> usize {
    let mut num_times_called = 0;
    let test_values = vec![1,3,5,31,2,-13,10,16];

    test_values.sort_by(|a, b| {
        a.cmp(b)
        num_times_called += 1;
    });

    num_times_called
}
```

The closure passed to `sort_by` has to copy or borrow anything not passed into it. In this case, that would be the `num_times_called` variable. Since we want to modify the variable, it has to be borrowed. Hence, the closure has the lifetime of that borrow, not the whole program, because it can't be called anytime - only when `num_times_called` is a valid thing to read or write.

Async functions, it turns out, *act like closures that don't take parameters*! They *have to*, because all `Future`s have to implement the same trait method `poll`:

```rust,ignore
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

When you call an async function, all of it's parameters are copied or borrowed into the `Future` that it returns. Since we need to borrow the `World`, the `Future` has the lifetime of `&'a mut World`, not of `'static`.

Barbara suggests changing all of the async function's parameters to be owned types. Alan asks Grace, who architected this project. Grace recommends holding a reference to the `Plugin` that owns the `World`, and then borrowing it whenver you need the `World`. That ultimately looks like the following:

```rust,ignore
async fn load_image(src: String, inside_of: usize, player: Arc<Mutex<Player>>) {
    let request = make_request(&url);
    let data = window().unwrap().fetch_with_request(&request).await.unwrap().etc.etc.etc;
    let image = parse_png(data, context);

    player.lock().unwrap().update(|world| {
        let new_node_index = world.nodes.len();
        if let Some(parent) = world.nodes.get(inside_of) {
            parent.set_child(new_node_index);
        }
        world.nodes.push(image.into());
    });
}
```

It works, well enough that Alan is able to finish his changes and PR them into the project. However, Alan wonders if this could be syntactically cleaner, somehow. Right now, async and update code have to be separated - if we need to do something with a `World`, then `await` something else, that requires jumping in and out of this `update` thing. It's a good thing that we only really *have* to be async in these loaders, but it's also a shame that we practically *can't* mix `async` code and `World`s.

## 🤔 Frequently Asked Questions

* **What are the morals of the story?**
    * Async functions capture all of their parameters for the entire duration of the function. This allows them to hold borrows of those parameters across await points.
      * When the parameter represents any kind of "global environment", such as the `World` in this story, it may be useful for that parameter not to be captured by the future but rather supplied anew after each await point.
    * Non-`'static` Futures are of limited use to developers, as lifetimes are tied to the sync stack. The execution time of most asynchronous operations does not come with an associated lifetime that an executor could use.
      * It is possible to use borrowed futures with `block_on` style executors, as they necessarily extend all lifetimes to the end of the Future. This is because they turn asynchronous operations back into synchronous ones.
      * Most practical executors want to release the current stack, and thus all of it's associated lifetimes. They need `'static` futures.
    * Async programming introduces more complexity to Rust than it does, say, JavaScript. The complexity of async is [sometimes explained in terms of 'color'][WhatColor], where functions of one 'color' can only call those of another under certain conditions, and developers have to keep track of what is sync and what is async. Due to Rust's borrowing rules, we actually have three 'colors', not the two of other languages with async I/O:
      * Sync, or 'blue' in the original metaphor. This color of function can both own and borrow it's parameters. If made into the form of a closure, it may have a lifetime if it borrows something from the current stack.
      * Owned Async, or 'red' in the original metaphor. This color of function can only own parameters, by copying them into itself at call time.
      * Borrowed Async. If an async function borrows at least one parameter, it gains a lifetime, and must fully resolve itself before the lifetime of it's parameters expires.
* **What are the sources for this story?**
    * This is personal experience. Specifically, I had to do [almost exactly this dance][RuffleAsync] in order to get fetch to work in Ruffle.
    * I have omitted a detail from this story: in Ruffle, we use a GC library (`gc_arena`) that imposes a special lifetime on all GC references. This is how the GC library upholds it's memory safety invariants, but it's also what forces us to pass around contexts, and once you have that, it's natural to start putting even non-GC data into it. It also means we can't hold anything from the GC in the Future as we cannot derive it's `Collect` trait on an anonymous type.
* **Why did you choose Alan to tell this story?**
    * Lifetimes on closures is already non-obvious to new Rust programmers and using them in the context of Futures is particularly unintuitive.
* **How would this story have played out differently for the other characters?**
    * Niklaus probably had a similar struggle as Alan.
    * Grace would have felt constrained by the `async` syntax preventing some kind of workaround for this problem.
    * Barbara already knew about Futures and 'static and carefully organizes their programs accordingly.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
[RuffleAsync]: https://github.com/ruffle-rs/ruffle/blob/master/core/src/loader.rs
[WasmFetch]: https://docs.rs/web-sys/0.3.50/web_sys/struct.Window.html#method.fetch_with_request
[WasmJsFuture]: https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/struct.JsFuture.html
[WasmSpawn]: https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html
[WhatColor]: https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/
