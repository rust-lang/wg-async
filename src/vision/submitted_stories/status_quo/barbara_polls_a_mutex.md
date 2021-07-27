## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## Barbara polls a Mutex

### Brief summary

[Barbara] is implementing an interpreter for a scripting language. This
language has implicit asynchronicity, so all values in the language can
potentially be futures underneath.

Barbara wants to store a namespace which maps variable names to their values. She
chooses to use a `HashMap` and finds the `async_lock` crate provides an async
mutex, which she can use for concurrency. She determines she'll need a lock
around the namespace itself to protect concurrent modification.

For the entries in her map, Barbara decides to implement a two-variant enum. One
variant indicates that there is no implicit asynchronicity to resolve and the
value is stored directly here. The other variant indicates that this value is
being computed asynchronously and polling will be required to resolve it.
Because an asynchronous task might want to change one of these entries from the
asynchronous variant to the ready variant, she'll need to wrap the entries in
an `Arc` and a `Mutex` to allow an asynchronous task to update them.

Barbara wants to be able to derive a future from the entries in her namespace
that will allow her to wait until the entry becomes ready and read the value.
She decides to implement the `Future` trait directly. She's done this before
for a few simple cases, and is somewhat comfortable with the idea, but she runs
into significant trouble trying to deal with the mutex in the body of her poll
function. Here are her attempts:

```rust=
use async_lock::Mutex;

enum Value {
    Int(i32),
}

enum NSEntry {
    Ready(Value),
    Waiting(Vec<Waker>),
}

type Namespace = Mutex<String, Arc<Mutex<NSEntry>>>;

// Attempt 1: This compiles!!
struct NSValueFuture(Arc<Mutex<NSEntry>>);
impl Future for NSValueFuture {
    type Output = Value;
    pub fn poll(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<Self::Output> {
        let entry = match self.0.lock().poll() {
            Poll::Ready(ent) => ent,

            // When this returns, it will drop the future created by lock(),
            // which drops our position in the lock's queue.
            // You could never wake up.
            // Get starved under contention. / Destroy fairness properties of lock.
            Poll::Pending => return Poll::Pending,
        };

        ...
    }
}

// Attempt 2
struct NSValueFuture {
    ent: Arc<Mutex<NSEntry>>,
    lock_fut: Option<MutexGuard<'_, NSEntry>>,
}
impl Future for NSValueFuture {
    type Output = Value;
    pub fn poll(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<Self::Output> {
        if self.lock_fut.is_none() {
            self.lock_fut = Some(self.ent.lock()),
        }
        // match self.lock_fut.unwrap().poll(cx)
        // Pulled out pin-project, got confused, decided to just use unsafe.
        match unsafe { Pin::new_unchecked(&mut self).lock_fut.unwrap() }.poll(cx) {
            ...
        }
        // ??? lifetime for MutexLockFuture ???
        // try async-std, async-lock
    }
}

// Realize `lock_arc()` is a thing
// Realize you need `BoxFuture` to await it, since you can't name the type

// Working code:
struct NsValueFuture {
    target: Arc<Mutex<NsValue>>,
    lock_fut: Option<BoxFuture<'static, MutexGuardArc<NsValue>>>,
}

impl Future for NsValueFuture {
    type Output = Value;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.lock_fut.is_none() {
            let target = Arc::clone(&self.target);
            let lock = async move { target.lock_arc().await }.boxed();
            self.lock_fut = Some(lock)
        }

        if let Poll::Ready(mut value) = self.lock_fut.as_mut().unwrap().as_mut().poll(cx) {
            self.lock_fut = None;
            match &mut *value {
                NsValue::Ready(x) => {
                    Poll::Ready(x.clone())
                }
                NsValue::Waiting(w) => {
                    w.push(cx.waker().clone());
                    Poll::Pending
                }
            }
        } else {
            Poll::Pending
        }
    }
}
```

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
* Trying to compose futures manually without an enclosing async block/function
is extremely difficult and may even be dangerous.

### **What are the sources for this story?**
*Talk about what the story is based on, ideally with links to blog posts, tweets, or other evidence.*

### **Why did you choose *Barbara* to tell this story?**
* It's possible to be fairly comfortable with Rust and even some of the
internals of async and still be stopped in your tracks by this issue.

### **How would this story have played out differently for the other characters?**
*In some cases, there are problems that only occur for people from specific backgrounds, or which play out differently. This question can be used to highlight that.*

[status quo stories]: ../status_quo.md
[Barbara]: ../../characters/barbara.md
[htvsq]: ../../how_to_vision/status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
