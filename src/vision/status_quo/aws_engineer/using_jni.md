# Status quo of an AWS engineer: Using JNI

One other problem that Alan's team has encountered is that some of the standard libraries they would use at AWS are only available in Java. After some tinkering, Alan's team decides to stand-up a java server as part of their project. The idea is that the server can accept the connections and then use JNI to invoke the Rust code; having the Rust code in process means it can communicate directly with the underlying file descriptor and avoid copies.

They stand up the Java side fairly quickly and then spend a bit of time experimenting with different ways to handle the "handoff" to Rust code. The first problem is keeping the tokio runtime alive. Their first attempt to connect using JNI was causing the runtime to get torn down. But they figure out that they can store the Runtime in a static variable.

Next, they find having Rust code access Java objects is quite expensive; it's cheaper to pass bytebuffers at the boundary using protobuf. They try a few options for serialization and deserialization to find which works best.

Overall, the integration with the JNI works fairly smoothly for them, but they wish there had been some documented pattern to have just shown them the best way to set things up, rather than them having to discover it.