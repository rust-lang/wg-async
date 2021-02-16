# User stories: Where we want to get to

Meet Sally. Sally is lucky enough to be learning async programming in the shiny future, when all the hard work is done. Like Bob, she's coming to Rust as a JavaScript programmer, and excited to learn about this nifty language she's heard so much about. The following sections tell stories from Sally's journey.

## Sally builds a web server

Sally has decided to build a web application. She's familiar with async-await from JavaScript, and she knows that Rust has a similar feature. She's eager to get started, so she goes to "Rust by Example" where she sees an example for building a web server that runs the guessing game. That sounds perfect!

Following the book's instructions, she quickly stands up some simple examples using async-await. She's using the standard Rust runtime and recommended web framework. With only a few line of code, she's able to play the ["guessing game"] in her browser[^written-in-rust]. This is fun!

["guessing game"]: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

**Key observations:**

* Async is integrated into standard Rust learning materials like RBE, not its own silo.
* Standard ways to stand up a runtime and web server, no choices to make (at first...)
    * *FIXME:* We should cover how she is able to easily change to other runtimes later and things keep working

## Sally learns about spawn

Still playing around, Sally starts to extend her 'guessing game' project with some new functionality. She adds a function that launches a few web requests to help her pick the random number and get a nice "quote of the day" to include in the website:

```rust
async fn guessing_game_configuration() -> std::io::Result<(usize, String)> {
    let random_number = async_io::request::fetch("https://random-number.org/").await?;
    let quotd = async_io::request::fetch("https://qotd.org/").await?;
    Ok((random_number, qotd))
}
```

Here, she is using the standard API for launching web requests. When she puts this into her IDE, she gets a lint warning:

```
warning[E2222]: synchronized request fetch
 --> src/main.rs:2:13
  |
2 |     let random_number = async_io::request::fetch("https://random-number.org/").await?;
  |                                                                                ------
3 |     let quotd = async_io::request::fetch("https://qotd.org/").await?;
  |                                                               ------
  |
  = help: these two request fetches will be synchronized, which probably isn't what you want
2 |     let random_number = async_io::request::fetch("https://random-number.org/").spawn().await?;
  |                                                                                ----- consider introducing calls to `spawn`
3 |     let quotd = async_io::spawn(async_io::request::fetch("https://qotd.org/")).spawn().await?;
  |                                                                                ----- here too
```

Reading a bit more into the background for E2222, she learns about how async-await in Rust works differently from async-await in javaScript. She learns about spawning. etc etc.

**Key observations:**

* Having standard or recommended APIs will let us give better suggestions.
* Exposing things like `spawn` as methods feels right to nikomatsakis =) 

## Sally goes deeper with the Rust book

Sally wants to get past simple examples, so she opens up the Rust book. She learns more about modules and ownership, while flipping over to the "Asynchronous Programming" section from time to time to learn more about that. 

**Key observations:**

* *FIXME:* This is really just wishful thinking. How do we actually make this happen? Is going to the book even realistic? Maybe this would be a good place to talk about helpful error messages, thoughtful suggetions from the compiler ('did you want to spawn?')

## Citations, action items, and other thoughts

[^written-in-rust]: Sally doesn't know it, or maybe she does, but her browser in Rust, too.
