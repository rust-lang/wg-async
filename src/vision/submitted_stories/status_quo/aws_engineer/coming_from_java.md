# Status quo of an AWS engineer: Coming from Java

At first, Alan is trying to write Rust code as if it were Java. He's accustomed to avoiding direct dependencies between types and instead modeling everything with an `interface`, so at first he creates a lot of Rust traits. He quickly learns that `dyn Trait` can be kind of painful to use. 

He also learns that Rust doesn't really permit you to add references willy nilly. It was pretty common in Java to have a class that was threaded everywhere with all kinds of references to various parts of the system. This pattern often leads to borrow check errors in Rust.

He gets surprised by parallelism. He wants a concurrent hash map but can't find one in the standard library. There are a lot of crates on crates.io but it's not clear which would be best. He decides to use a mutex-protected lock. 

He is surprised because futures in Java correspond to things executed in parallel, but in Rust they don't. It takes him some time to get used to this. Eventually he learns that a Rust future is more akin to a java *callable*.