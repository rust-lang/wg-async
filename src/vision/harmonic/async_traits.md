# Async traits are possible

It's Monday. The birds are singing. Alan walks upstairs to his home office and logs on. He is working on a Rust library for the [YouBuy] service.  Currently the library is hard-coding the use of the `reqwest` http library, but he has gotten several requests from users who would like to shim out the HTTP library or do other strange things, so he would like to make it possible for users to provide their own. He begins by defining a trait:

[YouBuy]: ../projects/YouBuy.md

```rust
trait HttpClient {
    async fn perform(&mut self, request: Request) -> Response;
}
```

The compiler gives him an error, though, explaining that async traits are not supported in 

And then he starts to make his code generic over the http client:

```rust
struct MyService<HC: HttpClient> {
    hc: HC
}
```

