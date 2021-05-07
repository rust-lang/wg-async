# Status quo of an AWS engineer: Borrow check errors

Alan has more or less gotten the hang of the borrow checker, but sometimes it still surprises him. One day, he is working on a piece of code in DistriData. There are a set of connections:

```rust=
struct Connection {
    buffer: Vec<u8>,
}
```

and each `Connection` has the ability to iterate through various requests. These requests return subslices of the data in the connection:

```rust=
struct Request<'a> { 
    headers: Vec<&'a u8>,
}
```

He writes a routine to get the next request from the connection. It begins by reading data into the internal buffer and then parsing from that buffer and returning the request ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6d8f2e7349e25677b25c527964842de8)):

```rust=
impl Connection {
    pub async fn read_next(&mut self) -> Request<'_> {
       loop {
           self.read_into_buffer();
           
           // can't borrow self.buffer, even though we only hang on to it in the
           // return branch
           match Request::try_parse(&self.buffer) {    
               Some(r) => return r,
               None => continue,
           }
       }
    }   
       
    async fn read_into_buffer(&mut self) {
        self.buffer.push(1u8);
    }
}
```

This code, however, doesn't build. He gets the following error:

```
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
  --> src/lib.rs:15:12
   |
13 |     pub async fn read_next(&mut self) -> Request<'_> {
   |                            - let's call the lifetime of this reference `'1`
14 |        loop {
15 |            self.read_into_buffer().await;
   |            ^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
...
19 |            match Request::try_parse(&self.buffer) {    
   |                                     ------------ immutable borrow occurs here
20 |                Some(r) => return r,
   |                                  - returning this value requires that `self.buffer` is borrowed for `'1`
```

This is confusing. He can see that there is a mutable borrow occuring, and an immutable one, but it seems like they occur at disjoint periods of time. Why is the compiler complaining?

After asking on `#rust` in the AWS Slack, he learns that this is a pattern that Rust's borrow checker just can't support. It gets confused when you return data from functions and winds up producing errors that aren't necessary. Apparently there's some [research project named after a Hamlet play](https://github.com/rust-lang/polonius/) that might help, but that isn't going to help him now. The slack channel points him at the [ouroboros](https://github.com/joshua-maros/ouroboros) project and he eventually uses it to work around the problem ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=59b2cb72529e58c13ab00eee1e9c0435)).
