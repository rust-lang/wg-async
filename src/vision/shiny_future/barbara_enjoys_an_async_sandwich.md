# ✨ Shiny future stories: Barbara enjoys her async-sync-async sandwich :sandwich: 

:::warning
Alternative titles:
- Barbara enjoys her async-sync-async sandwich :sandwich: 
- Barbara recursively blocks
- Barbara blocks and blocks and blocks
:::


## 🚧 Warning: Draft status 🚧

This is a draft "shiny future" story submitted as part of the brainstorming period. It is derived from what actual Rust users wish async Rust should be, and is meant to deal with some of the challenges that Async Rust programmers face today.

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as peoples needs and desires for async Rust may differ greatly, shiny future stories [cannot be wrong]. At worst they are only useful for a small set of people or their problems might be better solved with alternative solutions). Alternatively, you may wish to [add your own shiny vision story][htvsq]!

## The story

Barbara wants to customize a permissions lookup when accepting requests. The library defines a `trait PermitRequest`, to allow the user to define their own rules. Nice!

```rust
trait PermitRequest {}
```

She starts small, to get her feet wet.

```rust
struct Always;

impl PermitRequest for Always {
    fn permit(&self, _: &Request) -> bool {
        true
    }
}
```

All requests are permitted! Simple, but now to actually to implement the permissions logic.

One of the basic rules Barbara has is to check the request for the existence of a header, but the function is written as `async`, since Barbara figured it might need to be eventually.

```rust
async fn req_has_header(req: &Request) -> bool {
    req.headers().contains_key("open-sesame")
}
```

When Barbara goes to implement the `PermitRequest` trait, she realizes a problem: the trait did not think permissions would require an async lookup, so its method is not `async`. Barbara tries the easiest thing first, hoping that she can just block on the future.

```rust
struct HasHeader;

impl PermitRequest for HasHeader {
    fn permit(&self, req: &Request) -> bool {
        task::block_on(req_has_header(req))
    }
}
```

When Barbara goes to run the code, it works! Even though she was already running an async runtime at the top level, trying to block on *this* task didn't panic or deadlock. This is because the runtime optimistically hoped the future would be available without needing to go to sleep, and so when it found the currently running runtime, it re-used it to run the future.

The compiler *does* emit a warning, thanks to a blocking lint (link to shiny future when written). It let Barbara know this could have performance problems, but she accepts the trade offs and just slaps a `#[allow(async_blocking)]` attribute in there.


Barbara, now energized that things are looking good, writes up the other permission strategy for her application. It needs to fetch some configuration from another server based on a request header, and to keep it snappy, she limits it with a timeout.

```rust
struct FetchConfig;

impl PermitRequest for FetchConfig {
    fn permit(&self, req: &Request) -> bool {
        let token = req.headers().get("authorization");
        
        #[allow(async_blocking)]
        task::block_on(async {
            select! {
                resp = fetch::get(CONFIG_SERVER).param("token", token) => {
                    resp.status() == 200
                },
                _ = time::sleep(2.seconds()) => {
                    false
                }
            }
        })
    }
}
```

This time, there's no compiler warning, since Barbara was ready for that. And running the code, it works as expected. The runtime was able to reuse the IO and timer drivers, and not need to disrupt other tasks.

However, the runtime chose to emit a runtime log at the warning level, informing her that while it was able to make the code work, it *could* have degraded behavior if the same parent async code were waiting on this and another async block, such as via `join!`. In the first case, since the async code was ready immediately, no actual harm could have happened. But this time, since it had to block the task waiting on a timer and IO, the log was emitted.

Thanks to the runtime warning, Barbara does some checking that the surround code won't be affected, and once sure, is satisfied that it was easier than she thought to make an async-sync-async sandwich.


## 🤔 Frequently Asked Questions

### What status quo stories are you retelling?

While this story isn't an exact re-telling of an existing status quo, it covers the morals of a couple:

- [Barbara bridges sync and async](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/barbara_bridges_sync_and_async.html)
- [A comment about async in `ResolveServerCerts`](https://github.com/rust-lang/wg-async-foundations/pull/164#issuecomment-824028298)

### What are the key attributes of this shiny future?

- `block_on` tries to be forgiving and optimistic of nested usage.
    - It does a best effort to "just work".
- But at the same time, it provides information to the user that it might not always work out.
    - A compiletime lint warns about the problem in general.
        - This prods a user to *try* to use `.await` instead of `block_on` if they can.
    - A runtime log warns when the usage could have reacted badly with other code.
        - This gives the user some more information if a specific combination degrades their application.

### What is the "most shiny" about this future? 

It significantly increases the areas where `block_on` "just works", which should improve *productivity*.

### What are some of the potential pitfalls about this future?

- While this shiny future tries to be more forgiving when nesting `block_on`, the author couldn't think of a way to completely remove the potential dangers therein.
- By making it *easier* to nest `block_on`, it might increase the times a user writes code that degrades in performance.
    - Some runtimes would purposefully panic early to try to encourage uses to pick a different design that wouldn't degrade.
    - However, by keeping the warnings, hopefully users can evaluate the risks themselves.

*Thing about Rust's core "value propositions": performance, safety and correctness, productivity. Are any of them negatively impacted? Are there specific application areas that are impacted negatively? You might find the sample [projects] helpful in this regard, or perhaps looking at the goals of each [character].*

### Did anything surprise you when writing this story? Did the story go any place unexpected?

No.

### What are some variations of this story that you considered, or that you think might be fun to write? Have any variations of this story already been written?

A variation would be an even more optimistic future, where we are able to come up with a technique to completely remove all possible bad behaviors with nested `block_on`. The author wasn't able to think of how, and it seems like the result would be similar to just being able to `.await` in every context, possibly implicitly.

### What are some of the things we'll have to figure out to realize this future? What projects besides Rust itself are involved, if any? (Optional)

- A runtime would need to be modified to be able to lookup through a thread-local or similar whether a runtime instance is already running.
- A runtime would need some sort of `block_in_place` mechanism.
- We could make a heuristic to guess when `block_in_place` would be dangerous.
    - If the runtime knows the task's waker has been cloned since the last time it was woken, then *probably* the task is doing something like `join!` or `select!`.
    - Then we could emit a warning like "nested block_on may cause problems when used in combination with `join!` or `select!`"
    - The heuristic wouldn't work if the nested block_on were part of the *first* call of a `join!`/`select!`.
    - Maybe a warning regardless is a good idea.
    - Or a lint, that a user can `#[allow(nested_block_on)]`, at their own peril.
- This story uses a generic `task::block_on`, to not name any specific runtime. It doesn't specifically assume that this could work cross-runtimes, but maybe a shinier future would assume it could?
- This story referes to a lint in a proposed different shiny future, which is not yet written.



[character]: ../characters.md
[comment]: ./comment.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[projects]: ../projects.md
[htvsq]: ../how_to_vision/shiny_future.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade

