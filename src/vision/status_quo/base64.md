# Implementing a base64 decoding library

* Trying to implement a library that can be used in both embedded and more "full featured" environments
* XXX what is this experience like?
* maybe take inspiration from https://docs.rs/drogue-tls/0.2.0/drogue_tls/index.html
    * it had to define its own AsyncRead/AsyncWrite traits due to the ones from Future not working in nostd
