# 😱 Status quo stories: Barbara builds an async executor


## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

## The story

Barbara wants to set priorities to the tasks spawned to the executor. However, she finds no existing async executor provides such a feature
She would be more than happy to enhance an existing executor and even intends to do so at some point. At the same time, Barbara understand that the process of getting
changes merged officially into an executor can be long, and for good reason.

Due to pressure and deadlines at work she needs a first version to be working as soon as possible. She then decides to build her own async executor.

First, Barbara found [crossbeam-deque](https://crates.io/crates/crossbeam-deque) provides work-stealing deques of good quality. She decides to use it to build task schedulers. She plans for each working thread to have a loop which repeatedly gets a task from the deque and polls it.

But wait, what should we put into those queues to represent each "task"?

At first, Barbara thought it must contain the `Future` itself and the additional priority which was used by the scheduler. So she first wrote:

```rust
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    priority: u8
}
```

And the working thread loop should run something like:

```rust
pub fn poll_task(task: Task) {
    let waker = todo!();
    let mut cx = Context::from_waker(&waker);
    task.future.as_mut().poll(&mut cx);
}
```

"How do I create a waker?" Barbara asked herself. Quickly, she found the `Wake` trait. Seeing the `wake` method takes an `Arc<Self>`, she realized the task in the scheduler should be stored in an `Arc`. After some thought, she realizes it makes sense because both the deque in the scheduler and the waker may hold a reference to the task.

To implement `Wake`, the `Task` should contain the sender of the scheduler. She changed the code to something like this:

```rust
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    scheduler: SchedulerSender,
    priority: u8,
}

unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.scheduler.send(self.clone());
    }
}

pub fn poll_task(task: Arc<Task>) {
    let waker = Waker::from(task.clone());
    let mut cx = Context::from_waker(&waker);
    task.future.as_mut().poll(&mut cx);
//  ^^^^^^^^^^^ cannot borrow as mutable
}
```

The code still needed some change because the `future` in the  `Arc<Task>` became immutable. 

"Okay. I can guarantee `Task` is created from a `Pin<Box<Future>>`, and I think the same future won't be polled concurrently in two threads. So let me bypass the safety checks." Barbara changed the future to a raw pointer and confidently used some `unsafe` blocks to make it compile.

```rust
pub struct Task {
    future: *mut (dyn Future<Output = ()> + Send + 'static),
    ...
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

pub fn poll_task(task: Arc<Task>) {
    ...
    unsafe {
        Pin::new_unchecked(&mut *task.future).poll(&mut cx);
    }
}
```

Luckily, a colleague of Barbara noticed something wrong. The `wake` method could be called multiple times so multiple copies of the task could exist in the scheduler. The scheduler might not work correctly because of this. What's worse, a more severe problem was that multiple threads might get copies of the same task from the scheduler and cause a race in polling the future.

Barbara soon got a idea to solve it. She added a state field to the `Task`. By carefully maintaining the state of the task, she could guarantee there are no duplicate tasks in the scheduler and no race can happen when polling the future.

```rust
const NOTIFIED: u64 = 1;
const IDLE: u64 = 2;
const POLLING: u64 = 3;
const COMPLETED: u64 = 4;

pub struct Task {
    ...
    state: AtomicU64,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        let mut state = self.state.load(Relaxed);
        loop {
            match state {
                // To prevent a task from appearing in the scheduler twice, only send the task
                // to the scheduler if the task is not notified nor being polling. 
                IDLE => match self
                    .state
                    .compare_exchange_weak(IDLE, NOTIFIED, AcqRel, Acquire)
                {
                    Ok(_) => self.scheduler.send(self.clone()),
                    Err(s) => state = s,
                },
                POLLING => match self
                    .state
                    .compare_exchange_weak(POLLING, NOTIFIED, AcqRel, Acquire)
                {
                    Ok(_) => break,
                    Err(s) => state = s,
                },
                _ => break,
            }
        }
    }
}

pub fn poll_task(task: Arc<Task>) {
    let waker = Waker::from(task.clone());
    let mut cx = Context::from_waker(&waker);
    loop {
        // We needn't read the task state here because the waker prevents the task from
        // appearing in the scheduler twice. The state must be NOTIFIED now.
        task.state.store(POLLING, Release);
        if let Poll::Ready(()) = unsafe { Pin::new_unchecked(&mut *task.future).poll(&mut cx) } {
            task.state.store(COMPLETED, Release);
        }
        match task.state.compare_exchange(POLLING, IDLE, AcqRel, Acquire) {
            Ok(_) => break,
            Err(NOTIFIED) => continue,
            _ => unreachable!(),
        }
    }
}
```

Barbara finished her initial implementation of the async executor. Despite there were a lot more possible optimizations, Barbara already felt it is a bit complex. She was also confused about why she needed to care so much about polling and waking while her initial requirement was just adding additional information to the task for customizing scheduling.

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**
  * It is difficult to customize any of the current async executors (to my knowledge). To have any bit of special requirement forces building an async executor from scratch.
  * It is also not easy to build an async executor. It needs quite some exploration and is error-prone. [`async-task`](https://github.com/smol-rs/async-task) is a good attempt to simplify the process but it could not satisfy all kinds of needs of customizing the executor (it does not give you the chance to extend the task itself).
### **What are the sources for this story?**
  * The story was from my own experience about writing a new thread pool supporting futures: https://github.com/tikv/yatp.
  * People may feel strange about why we want to set priorities for tasks. Currently, the futures in the thread pool are like user-space threads. They are mostly CPU intensive. But I think people doing async I/O may have the same problem.
### **Why did you choose Barbara to tell this story?**
  * At the time of the story, I had written Rust for years but I was new to the concepts for async/await like `Pin` and `Waker`.
### **How would this story have played out differently for the other characters?**
  * People with less experience in Rust may be less likely to build their own executor. If they try, I think the story is probably similar.

[character]: ../../characters.md
[status quo stories]: ../status_quo.md
[Alan]: ../../characters/alan.md
[Grace]: ../../characters/grace.md
[Niklaus]: ../../characters/niklaus.md
[Barbara]: ../../characters/barbara.md
[htvsq]: ../status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
