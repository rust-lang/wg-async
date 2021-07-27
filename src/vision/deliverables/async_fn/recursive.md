# Recursive async fn

## Impact

* Able to write recursive async functions that do not require dynamic dispatch

## Requires

* [Boxable async fn](./boxable.md)

## Design notes

Recursive async functions are not currently possible. This is an artifact of how async fns work today: they allocate all the stack space they will ever need in one shot, which cannot be known for recursive functions.

### Status quo

Since all discussions of recursion must use fibonacci as the example, consider this:

```rust
async fn fib(n : u32) -> u64 {
   match n {
       0     => panic!("zero is not a valid argument to fib()!"),
       1 | 2 => 1,
       3     => 2,
       _ => fib(n-1).await + fib(n-2).await
   }
}
```

To enable recursion, this can be rewritten to return a boxed future, although there are other approaches:

```rust
fn fib(
    n: u32,
) -> Pin<Box<dyn Future<Output = u64> + Send>> {
    Box::pin(async move {
        match n {
            0 => ::std::rt::begin_panic("zero is not a valid argument to fib()!"),
            1 | 2 => 1,
            3 => 2,
            _ => fib(n - 1).await + fib(n - 2).await,
        }
    }
}
```

The [async-recursion](https://crates.io/crates/async-recursion) crate encapsulates the "return a boxed dyn Future" pattern:

```rust
use async_recursion::async_recursion;

#[async_recursion]
async fn fib(n : u32) -> u64 {
   match n {
       0     => panic!("zero is not a valid argument to fib()!"),
       1 | 2 => 1,
       3     => 2,
       _ => fib(n-1).await + fib(n-2).await
   }
}
```

### Exploration

There are two concerns to address:

- Infinitely sized _type_
  - Casting to `dyn Future` addresses this, but it's not the best solution, because one must decide whether to declare it as `Send` or not.
  - An alternative is using a nominal struct.
- Infinitely sized _value_
  - The box introduces indirection and prevents this.
  - The box could also be done at the _call site_.
  - Boxing is not the most efficient thing here, since every recursive call requires allocation; one might prefer an arena-like solution if this is a common problem (experience suggests it is not).

### Approach 1: More automated

The compiler could detect recursive async functions and automatically rewrite to a 'struct + box' pair, effectively like this:

```rust
struct Fib {
    future: Pin<Box<impl Future<Output = u64>>>
}

fn fib(
    n: u32,
) -> Fib {
    Fib {
        future: Box::pin(async move {
            match n {
                0 => ::std::rt::begin_panic("zero is not a valid argument to fib()!"),
                1 | 2 => 1,
                3 => 2,
                _ => fib(n - 1).await + fib(n - 2).await,
            }
        }
    }
}
```

There would be an "allow-by-default" lint that warns against the automatic box, so that users can "opt-in" to identifying when this occurs if they wish to be compatible with no-std or are concerned about performance.

### Approach 2

Create a syntax like `box async fn` that applies the above transform, and have an error for recursive types that suggests using this syntax. This is more explicit but forces uses to be aware of the distinction, even if they don't care.

