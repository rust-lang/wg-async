# Async read/write

## Impact

* Able to abstract over "something readable" and "something writeable"
* Able to use these traits with `dyn Trait`
* Able to easily write wrappers that "instrument" other readables/writeables
* Able to author wrappers like SSL, where reading may require reading *and* writing on the underlying data stream

## Design notes

### Challenge: Permitting simultaneous reads/writes

The obvious version of the existing `AsyncRead` and `AsyncWrite` traits would be:

```rust
#[repr(inline_async)]
trait AsyncRead {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
}

#[repr(inline_async)]
trait AsyncWrite {
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
}
```

This form doesn't permit one to simultaneously be reading and writing. Moreover, SSL requires changing modes, so that e.g. performing a read may require writing to the underlying socket, and vice versa. (Link?)

### Variant A: Readiness

One possibility is the design that [CarlLerche proposed](https://gist.github.com/carllerche/5d7037bd55dac1cb72891529a4ff1540), which separates "readiness" from the actual (non-async) methods to acquire the data:

````rust
pub struct Interest(...);
pub struct Ready(...);

impl Interest {
    pub const READ = ...;
    pub const WRITE = ...;
}

#[repr(inline)]
pub trait AsyncIo {
    /// Wait for any of the requested input, returns the actual readiness.
    ///
    /// # Examples
    ///
    /// ```
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    ///     loop {
    ///         let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;
    ///
    ///         if ready.is_readable() {
    ///             let mut data = vec![0; 1024];
    ///             // Try to read data, this may still fail with `WouldBlock`
    ///             // if the readiness event is a false positive.
    ///             match stream.try_read(&mut data) {
    ///                 Ok(n) => {
    ///                     println!("read {} bytes", n);
    ///                 }
    ///                 Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                     continue;
    ///                 }
    ///                 Err(e) => {
    ///                     return Err(e.into());
    ///                 }
    ///             }
    ///
    ///         }
    ///
    ///         if ready.is_writable() {
    ///             // Try to write data, this may still fail with `WouldBlock`
    ///             // if the readiness event is a false positive.
    ///             match stream.try_write(b"hello world") {
    ///                 Ok(n) => {
    ///                     println!("write {} bytes", n);
    ///                 }
    ///                 Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                     continue
    ///                 }
    ///                 Err(e) => {
    ///                     return Err(e.into());
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    async fn ready(&mut self, interest: Interest) -> io::Result<Ready>;
}

pub trait AsyncRead: AsyncIo {
    fn try_read(&mut self, buf: &mut ReadBuf<'_>) -> io::Result<()>;
}

pub trait AsyncWrite: AsyncIo {
    fn try_write(&mut self, buf: &[u8]) -> io::Result<usize>;
}
````

This allows users to:

- Take `T: AsyncRead`, `T: AsyncWrite`, or `T: AsyncRead + AsyncWrite`

Note that it is always possible to ask whether writes are "ready", even for a read-only source; the answer will just be "no" (or perhaps an error).

#### Can we convert all existing code to this form?

The `try_read` and `try_write` methods are basically identical to the existing "poll" methods. So the real question is what it takes to implement the `ready` async function. Note that tokio internally already adopts a model very similar to this on many types (though there is no trait for it).

It seems like the torture case to validate this is openssl.

### Variant B: Some form of split

Another alternative is to have read/write traits and a way to "split" a single object into separate read/write traits:

```rust
#[repr(inline_async)]
trait AsyncRead {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
}

#[repr(inline_async)]
trait AsyncWrite {
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
}

#[repr(inline_async)]
trait AsyncBidirectional: AsyncRead + AsyncWrite {
    async fn split(&mut self) -> (impl AsyncRead + '_, impl AsyncWrite + '_)
}
```

The challenge here is to figure out exactly how that definition should look. The version I gave above includes the possibility that the resulting readers/writers have access to the fields of `self`.

### Variant C: Extend traits to permit expressing that functions can both execute

Ranging further out into unknowns, it is possible to imagine extending traits with a way to declare that two `&mut self` methods could both be invoked concurrently. This would be generally useful but would be a fundamental extension to the trait system for which we don't really have any existing design. There is a further complication that the `read` and `write` methods are in distinct traits (`AsyncRead` and `AsyncWrite`, respectively) and hence cannot

```rust
#[repr(inline_async)]
trait AsyncRead {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
}

#[repr(inline_async)]
trait AsyncWrite {
}

#[repr(inline_async)]
trait AsyncBidirectional: AsyncRead + AsyncWrite {
    async fn split(&mut self) -> (impl AsyncRead + '_, impl AsyncWrite + '_)
}
```

### Variant D: Implement the `AsyncRead` and `AsyncWrite` traits for `&T`

In `std`, [there are `Read` and `Write` impls for `&File`](https://doc.rust-lang.org/std/fs/struct.File.html#impl-Read-1), and [the async-std runtime has followed suit](https://docs.rs/async-std/1.9.0/async_std/fs/struct.File.html#impl-Read-1). This means that you can express "can do both `AsyncRead + AsyncWrite`" as `AsyncRead + AsyncWrite + Copy`, more or less, or other similar tricks. However, it's not possible to do this for _any type_. Worth exploring.
