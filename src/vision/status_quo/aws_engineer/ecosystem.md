# Status quo of an AWS engineer: Exploring the ecosystem

Alan finds that cargo is a super powerful tool, but he finds it very hard to find crates to use. He doesn't really feel he discovers crates so much as "falls upon" them by chance. For example, he happened to see a stray mention of `cargo bloat` in the internals form, and that turned out to be exactly what he needed. He finds the `async-trait` crate in a similar way. He's happy these tools exist, but he wishes he had more assurance of finding them; he wonders what useful things are out there that he *doesn't* know about.

In some cases, there are a lot of choices and it's really hard to tell which is best. Alan spent some time evaluating crates that do md5 hashing, for example, and found tons of choices. He does some quick performance testing and finds huge differences: openssl seems to be the fastest, so he takes that, but he is worried he may have missed some crates.

He had decided to use tokio because it was the thing that everyone else is using. But he gradually learns that there are more runtimes out there. Sometimes, when he adds a crate, he finds that it is bringing in a new set of dependencies that don't seem necessary.

He also gets confused by the vast array of options. `tokio` seems to have an `AsyncRead` trait, for example, but so does `futures` -- which one should he use? 

He's heard of other runtimes and he might like to be able to try them out, but it would be too much work. Occasionally he winds up with multiple versions of the same crate, which can lead either to compilation or runtime errors. For example, when rusoto upgraded to a new version of tokio, this spilled over into confusing huge error messages from the rusoto builder owing to subtle trait and type mismatches. Fortunately the recent tokio 1.0 release promises to solve some of those problems.
