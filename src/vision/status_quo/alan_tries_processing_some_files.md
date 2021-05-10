# 😱 Status quo stories: Alan tries processing some files

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

Alan is new to Rust. He wants to build a program that recurses over all the files in a directory (and its subdirectories), reads each file, and produces some fingerprint of the file.

Since so much blocking I/O is involved, he chooses async in order to process many files concurrently.

### Recursion
Alan starts by writing a recursive function that can call some operation on each regular file in a directory and recurse on each subdirectory.
```rust
async fn process_directory<'a, F, P, T>(path: PathBuf, processor: &'a P) -> Vec<F>
where
    P: Fn(DirEntry) -> F,
    F: Future<Output = T>,
{
    ReadDirStream::new(read_dir(path).await.unwrap())
        .filter_map(|x| async {
            let dir_entry = x.unwrap();
            let ft = dir_entry.file_type().await.unwrap();
            if ft.is_file() {
                Some(vec![processor(dir_entry)])
            } else if ft.is_dir() {
                Some(process_directory(dir_entry.path(), processor).await)
            } else {
                None
            }
        })
        .collect::<Vec<Vec<F>>>()
        .await
        .into_iter()
        .flatten()
        .collect()
}
```

The first paper cut comes when the compiler complains:
```
error[E0733]: recursion in an `async fn` requires boxing
  --> src/main.rs:23:77
   |
23 | async fn process_directory<'a, F, P, T>(path: PathBuf, processor: &'a P) -> Vec<F>
   |                                                                             ^^^^^^ recursive `async fn`
   |
   = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`

...
For more information about an error, try `rustc --explain E0733`.
```

From the explainer, Alan learns that he cannot use the `async` sugaring, and needs to use a Boxed Pin in his function signature:
```
fn process_directory<'a, F, P, T>(
    path: PathBuf,
    processor: &'static P,
) -> Pin<Box<dyn Future<Output = Vec<F>>>>
```

New to Rust, Alan still doesn't really understand what `Pin` does, so he reads [the docs](https://doc.rust-lang.org/std/pin/index.html), sees that it marks which objects are *"guaranteed not to move"*, and wonders why the compiler couldn't determine this automatically since he read so much about how the borrow checker can already detect _moves_ versus borrows.

He's also not entirely sure why the returned `Future` needs to be `Boxed`. The suggested explainer helps a bit:
```
The `Box<...>` ensures that the result is of known size, and the pin is
required to keep it in the same place in memory.
```
But Alan figures that the size of `Future<Output = T>` should be determined by the type `T`. It's not like he's implementing a custom struct that is `Future`; he's returning a `Vec<T>` inside the standard `async move {}`. Alan wishes there was a way to express "*Hey I'm returning a Future created by `async move`, whose `Output` attribute has a known size, so the resulting Future should have a known size too!*"

But Alan does what the compiler tells him to do and adds some extra stuff to his function, which now looks like:

```rust
fn process_directory<'a, F, P, T>(
    path: PathBuf,
    processor: &'static P,
) -> Pin<Box<dyn Future<Output = Vec<F>> + 'a>>
where
    P: Fn(DirEntry) -> F,
    F: Future<Output = T>,
{
    Box::pin(async move {
        ReadDirStream::new(read_dir(path).await.unwrap())
            .filter_map(|x| async {
                let dir_entry = x.unwrap();
                let ft = dir_entry.file_type().await.unwrap();
                if ft.is_file() {
                    Some(vec![processor(dir_entry)])
                } else if ft.is_dir() {
                    Some(process_directory(dir_entry.path(), processor).await)
                } else {
                    None
                }
            })
            .collect::<Vec<Vec<F>>>()
            .await
            .into_iter()
            .flatten()
            .collect()
    })
}
```

### Rate Limiting
Alan knows that `process_directory` may be called on directories with many thousands of files or subdirectories, and is wary of exhausting file descriptor limits. Since he can't find much documentation about how to keep the number of async tasks in check - Tokio's docs suggest we can [spawn millions of tasks](https://tokio.rs/tokio/tutorial/spawning), but don't offer advice on how to manage tasks with expensive side effects - he decides he needs to build a simple rate limiter.

Alan's rate limiter will wrap some `Future<Output =T>`, acquire a semaphore, and then await the Future, returning the same type `T`:
```rust
async fn rate_limit<F, T>(fut: F, sem: &Semaphore) -> T
where
    F: Future<Output = T>,
{
    let _permit = sem.acquire().await;
    fut.await
}
```

Since the `async fn foo<T>() -> T` syntax desugars to `fn foo<T>() -> Future<Output = T>`, and since `fut.await` returns `T`, Alan assumes that the above is equivalent to:
```rust
fn rate_limit<F, T>(fut: F, sem: &Semaphore) -> F
where
    F: Future<Output = T>,
{
    ...
}
```

So he plugs this new `rate_limit` logic into `process_directory`:

```rust
use futures::future::join_all;                 
use futures::stream::StreamExt;                
use futures::Future;                           
use std::path::PathBuf;                        
use std::pin::Pin;                                    
use tokio::fs::{read_dir, DirEntry};     
use tokio::sync::Semaphore;                    
use tokio_stream::wrappers::ReadDirStream;     

async fn rate_limit<F, T>(fut: F, sem: &Semaphore) -> T
where
    F: Future<Output = T>,
{
    let _permit = sem.acquire().await;
    fut.await
}

fn process_directory<'a, F, P, T>(
    path: PathBuf,
    processor: &'a P,
    sem: &'static Semaphore,
) -> Pin<Box<dyn Future<Output = Vec<F>> + 'a>>
where
    P: Fn(DirEntry) -> F,
    F: Future<Output = T>,
{
    Box::pin(async move {
        ReadDirStream::new(read_dir(path).await.unwrap())
            .filter_map(|x| async {
                let dir_entry = x.unwrap();
                let ft = dir_entry.file_type().await.unwrap();
                if ft.is_file() {
                    Some(vec![rate_limit(processor(dir_entry), sem)])
                } else if ft.is_dir() {
                    Some(process_directory(dir_entry.path(), processor, sem).await)
                } else {
                    None
                }
            })
            .collect::<Vec<Vec<F>>>()
            .await
            .into_iter()
            .flatten()
            .collect()
    })
}

async fn expensive(de: DirEntry) -> usize {
    // assume this function spawns a task that does heavy I/O on the file
    de.file_name().len()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let sem = Semaphore::new(10);
    let path = PathBuf::from("/tmp/foo");
    let results = join_all(process_directory(path, &expensive, &sem).await);
    dbg!(results.await);
}
```

And is met with a new complaint from the compiler:
```
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:34:24
   |
18 |    fn process_directory<'a, F, P, T>(
   |                             - this type parameter
...
32 |  /                 if ft.is_file() {
33 |  |                     Some(vec![rate_limit(processor(dir_entry), sem)])
   |  |                     ------------------------------------------------- expected because of this
34 |  |                 } else if ft.is_dir() {
   |  |________________________^
35 | ||                     Some(process_directory(dir_entry.path(), processor, sem).await)
36 | ||                 } else {
37 | ||                     None
38 | ||                 }
   | ||                 ^
   | ||_________________|
   | |__________________`if` and `else` have incompatible types
   |                    expected opaque type, found type parameter `F`
   |
   = note: expected type `Option<Vec<impl futures::Future>>`
              found enum `Option<Vec<F>>`
   = help: type parameters must be constrained to match other types
   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
```

Alan is confused. In line 33, `rate_limit` returns `Future<Output = usize>`, so why is this an opaque `Future`? So far as he can tell, the `Option<Vec<impl futures::Future<Output = usize>` returned on line 33 is the same type as the `Option<Vec<F>>` where `F: Future<Output = usize>` returned on line 35.

So he strips the problem down to only a few lines of code, and still he cannot figure out why the compiler complains:
```rust
use futures::{future::pending, Future};

async fn passthru<F, T>(fut: F) -> T
where
    F: Future<Output = T>,
{
    fut.await
}

fn main() {
    let func = pending::<u8>;
    match true {
        true => passthru(func()),
        false => func(),
    };
}
```

To which the compiler nevertheless replies:
```
error[E0308]: `match` arms have incompatible types
  --> src/main.rs:14:18
   |
12 | /     match true {
13 | |         true => passthru(func()),
   | |                 ---------------- this is found to be of type `impl futures::Future`
14 | |         false => func(),
   | |                  ^^^^^^ expected opaque type, found struct `futures::future::Pending`
15 | |     };
   | |_____- `match` arms have incompatible types
   |
   = note: expected type `impl futures::Future`
            found struct `futures::future::Pending<u8>`
```

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
- The manual desugaring required for `async` recursion erases some of the "magic" of `async`.  
- Some programmers may never implement custom types that are `Future`, instead using standard constructs like `async` blocks to produce them. In these cases, the programmer might assume the returned `Future`s should have concrete types with known sizes, which would allow them to work directly with the returned types rather than have to deal with the complexities of trait objects, `Box`-ing, and opaque type comparisons.
- `Pin` documentation focuses on data that can or cannot "move" in memory. To someone new to Rust, it might be easy to confuse this concept with "move" semantics in the context of ownership.

### **What are the sources for this story?**
I describe my own experience while working on my first Rust project.

### **Why did you choose *NAME* to tell this story?**
I chose Alan to tell this story because I envision him comping from Python. I mostly work in `asyncio` Python by day, which means my exposure to async is shaped by what I'd expect from a language without traits, and one where heap wrangling and memory addressing is abstracted away.

### **How would this story have played out differently for the other characters?**
I'm not sure, but I'd assume:
- **Grace** would not get tripped up on the need for `Box::pin`
- **Niklaus** might share the confusion expressed above
- **Barbara** might wish we could use `async` sugaring in recursive functions.
