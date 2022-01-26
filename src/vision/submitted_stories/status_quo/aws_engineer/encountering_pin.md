# Status quo of an AWS engineer: Encountering pin

As Alan is building the server, he encounters a case where he wants to extend a stream of data to track some additional metrics. The stream implements [`AsyncRead`]. He thinks, "Ah, I'll just make a wrapper type that can extend any `AsyncRead`." He opens up the rustdoc, though, and realizes that this may be a bit tricky. "What is this `self: Pin<&mut Self>`?" notation, he thinks. He had vaguely heard of `Pin` when skimming the docs for futures and things but it was never something he had to work with directly before.

[`AsyncRead`]: https://docs.rs/tokio/1.5.0/tokio/io/trait.AsyncRead.html

Alan's experiences here are well documented in [Alan hates writing a Stream](https://rust-lang.github.io/wg-async/vision/status_quo/alan_hates_writing_a_stream.html). Suffice to say that, at long last, he does it to work, but he does not feel he really understands what is going on. Talking with his coworkers on slack he notes, "Mostly I just add `Pin` and whatever else the compiler asks for until it works; then I pray it doesn’t crash." :crossed_fingers:

*References:*

* [Alan hates writing a Stream](../alan_hates_writing_a_stream.html)
* ["Pin and suffering", by faster-than-lime](https://fasterthanli.me/articles/pin-and-suffering)
