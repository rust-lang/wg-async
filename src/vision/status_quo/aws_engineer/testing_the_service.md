# Status quo of an AWS engineer: Testing the service

At first, Alan is content to test by hand. But once the server is starting to really work, he realizes he needs to do unit testing. He wants to do something like [Mockito](https://site.mockito.org/) in Rust, so he starts searching the internet to find out what the options are. To his surprise, he learns that there doesn't seem to be any comparable framework in Rust. 

One option he considers is making all of his functions generic. For example, he could create a trait to model, for example, the network, so that he can insert artificial pauses and other problems during testing:

```rust
trait Network {
    ...
}
```

Writing such a trait is fairly complicated, but even if he wrote it, he would have to make all of his structs and functions generic:

```rust
struct MyService<N: Network> {
    ...
}
```

Alan starts threading these parameters through the code and quickly gets overwhelmed.

He decides instead to test his real code without any mocking. He and his team start building a load-testing framework, they call it "simworld". They need to be able to inject network errors, control timing, and force other unusual situations.

Building simworld takes a lot of time, but it is very useful, and they start to gain some confidence in their code.
