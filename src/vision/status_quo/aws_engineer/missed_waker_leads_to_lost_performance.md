# Status quo of an AWS engineer: Missed `Waker` leads to lost performance

Once the server is working, Alan starts to benchmark it. He is not really sure what to expect, but he is hoping to see an improvement in performance relative to the baseline service they were using before. To his surprise, it seems to be running slower!

Digging into the problem a bit, he wishes -- not for the first time -- that he had better tools to understand what was happening. This time he doesn't even bother trying to fire up the debugger. He just starts looking at his metrics. He's accumulated a pretty decent set of metrics by now, and he can often get a picture of the state of the runtime from them.

After a few days of poking at the problem, Alan notices something odd. It seems like there is often a fairly large delay between the completion of a particular event and the execution of the code that is meant to respond to that event. Looking more closely, he realizes that the code for handling that event fails to trigger the `Waker` associated with the future, and hence the future never wakes up.

As it happens, this problem was hidden from him because that particular future was combined with a number of others. Eventually, the other futures get signalled, and hence the event does get handled -- but less promptly than it should be. He fixes the problem and performance is restored.

"I'm glad I had a baseline to compare this against!", he thinks. "I doubt I would have noticed this problem otherwise."

