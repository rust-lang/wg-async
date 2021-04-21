# ✨ Shiny future stories: Alan uses async in traits

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

[Alan] is working on a project with [Barbara]. The project is currently written to use `reqwest` to manage its HTTP requests. The code looks like:

```rust
struct Context {
    http_client: Arc<reqwest::Client>,
    ...
}

impl Context {
    fn new(http_client: reqwest::Client) -> Self {
        Context {
            http_client: Arc::new(http_client),
        }
    }

    async fn do_the_thing(&self) {
        let some_request = /* make a request */;
        let response = self.http_client.perform(some_request).await;
        response.process();
    }
}
```

### Everything has to stay `Send`

The SDK implementation that Alan works on has some unit tests meant to ensure that its types remain `Send`:

```rust
fn assert_send<T: Send>(t: T) { }

#[test]
fn check_things_are_send() {
    let cx = Context::new(...);
    assert_send(cx);
    assert_send(cx.do_the_thing());
}
```

These tests ensure that:

* The main type `Context` is `Send`.
* Similarly, the function returned by `context.do_the_thing()` is also `Send`.
    * This requires that the future returned by `self.http_client.perform()` is `Send`.

### Introducing a trait with async functions

He wants the user to implement an async trait called `HttpClient` which has one method `perform(&self, request: Request) -> Response`. Alan writes the trait definition as follows:

```rust
trait HttpClient {
    async fn perform(&self, request: Request) -> Response;
}
```

Next he implements it for a few of the clients he is familiar with:

```rust
impl HttpClient for reqwest::Client {
    async fn perform(&self, request: Request) -> Response {
        ...
    }
}
```

### Trait objects

Next, Alan replaces the use of `reqwest::Client` with a `dyn HttpClient`:

```rust
struct Context {
    http_client: Arc<dyn HttpClient>,
}

impl Context {
    fn new(http_client: impl HttpClient + 'static) -> Self {
        Context {
            http_client: Arc::new(http_client),
        }
    }

    async fn do_the_thing(&self) {
        let some_request = /* make a request */;
        let response = self.http_client.perform(some_request).await;
        response.process();
    }
}
```

### Unit tests fail

After making this change, Alan notices that his unit tests no longer compile:

```
error[E0277]: `(dyn HttpClient + 'static)` cannot be sent between threads safely
  --> src/lib.rs:18:5
   |
13 | fn assert_send<T: Send>(t: T) { }
   |                   ---- required by this bound in `assert_send`
...
18 |     assert_send(c);
   |     ^^^^^^^^^^^ `(dyn HttpClient + 'static)` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `(dyn HttpClient + 'static)`
   = note: required because of the requirements on the impl of `Send` for `Arc<(dyn HttpClient + 'static)>`
   = note: required because it appears within the type `Context`
```

He knows that all the HTTP clients will be `Send`, so he modifies the `HttpClient` trait to include a `Send` supertrait:

```rust
trait HttpClient: Send {
    async fn perform(&self, request: Request) -> Response;
}
```

XXX this is not quite enough -- I have to figure out how to tell the part of the story that talks about the future that gets returned from `client.perform()`.

```rust
fn assert_send<T: Send>(t: T) { }

#[test]
fn check_things_are_send() {
    let cx = Context::new(...);
    assert_send(cx);
    assert_send(cx.do_the_thing());
}
```


### Are things still `Send`?

There are a few subtle things happening behind the scenes, but Alan isn't entirely aware of them:

* `Context: Send` is true because 

## 🤔 Frequently Asked Questions

### **What status quo story or stories are you retelling?**

* [Alan needs async in traits](../status_quo/alan_needs_async_in_traits.md)

### **What is [Alan] most excited about in this future? Is he disappointed by anything?**

He's not that excited; thinks work as expected.

### **What is [Grace] most excited about in this future? Is she disappointed by anything?**

As with Alan.

### **What is [Niklaus] most excited about in this future? Is he disappointed by anything?**

As with Alan.

### **What is [Barbara] most excited about in this future? Is she disappointed by anything?**

As with Alan.

### **What [projects] benefit the most from this future?**

All of them, most likely.

### **Are there any [projects] that are hindered by this future?**

Embedded projects like [MonsterMesh] are the most complex: `dyn` traits with `async fn` are tricky to implement without access to `Box`.

[MonsterMesh]: ../projects/MonsterMesh.md

### **What are the incremental steps towards realizing this shiny future?** (Optional)

* See the [Path to Async Functions in Traits](https://hackmd.io/5kCE2T6sTDijhqMx8kaikw)
* We also need to work out how `dyn` will work with `async fn` in traits 

### **Does realizing this future require cooperation between many projects?**

No, this is strictly connected to the core Rust language implementation.

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
