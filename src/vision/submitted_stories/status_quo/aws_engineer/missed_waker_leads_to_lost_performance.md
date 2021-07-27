# Status quo of an AWS engineer: Missed `Waker` leads to lost performance

Once the server is working, Alan starts to benchmark it. He is not really sure what to expect, but he is hoping to see an improvement in performance relative to the baseline service they were using before. To his surprise, it seems to be running slower!

After trying a few common tricks to improve performance without avail, Alan wishes -- not for the first time -- that he had better tools to understand what was happening. He decides instead to add more metrics and logs in his service, to understand where the bottlenecks are. Alan is used to using a well-supported internal tool (or a mature open source project) to collect metrics, where all he needed to do was pull in the library and set up a few configuration parameters.

However, in Rust, there is no widely-used, battle-tested library inside and outside his company. Even less so in an async code base! So Alan just used what seemed to be the best options: tracing and metrics crate, but he quickly found that they couldn't do a few of the things he wants to do, and somehow invoking the metrics is causing his service to be even slower. Now, Alan has to debug and profile his metrics implementation before he can even fix his service. (Cue another story on how that's difficult...)

After a few days of poking at the problem, Alan notices something odd. It seems like there is often a fairly large delay between the completion of a particular event and the execution of the code that is meant to respond to that event. Looking more closely, he realizes that the code for handling that event fails to trigger the `Waker` associated with the future, and hence the future never wakes up.

As it happens, this problem was hidden from him because that particular future was combined with a number of others. Eventually, the other futures get signalled, and hence the event does get handled -- but less promptly than it should be. He fixes the problem and performance is restored.

"I'm glad I had a baseline to compare this against!", he thinks. "I doubt I would have noticed this problem otherwise."

