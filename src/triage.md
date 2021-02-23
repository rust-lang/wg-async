# 🔍 Triage meetings

## When, where

The weekly triage meeting is held on [Zulip] at 13:00 US Eastern time on Fridays ([google calendar event for meeting](https://calendar.google.com/event?action=TEMPLATE&tmeid=M2VhYjRjczZxanE5ODcwbzR1bnZsNTV0MGFfMjAyMTAyMjZUMTgwMDAwWiA2dTVycnRjZTZscnR2MDdwZmkzZGFtZ2p1c0Bn&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com&scp=ALL)).

[Zulip]: ./welcome.md#zulip

## So you want to fix a bug?

If you're interested in fixing bugs, there is no need to wait for the triage meeting.
Take a look at the [mentored async-await bugs that have no assignee][bugs].
Every mentored bug should have a few comments.
If you see one you like, you can add the `@rustbot claim` comment into the bug and start working on it!
Feel to reach out to the mentor on [Zulip] to ask questions.

[bugs]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AE-mentor+label%3AA-async-await+no%3Aassignee

## Project board

The [project board] tracks various bugs and other work items for the async foundation group.
It is used to drive the triage process.

[project board]: https://github.com/orgs/rust-lang/projects/2

## Triage process

In our weekly triage meetings, we take new issues assigned [`A-async-await`] and categorize them. 
The process is:

- Review the [project board], from right to left:
  - Look at what got **Done**, and celebrate! :tada:
  - Review **In progress** issues to check we are making progress and there is a clear path to finishing (otherwise, move to the appropriate column)
  - Review **Blocked** issues to see if there is anything we can do to unblock
  - Review **Claimed** issues to see if they are in progress, and if the assigned person still intends to work on it
  - Review **To do** issues and assign to anyone who wants to work on something
- Review [uncategorized issues]
  - Mark `P-low`, `P-medium`, or `P-high`
  - Add `P-high` and _assigned_ `E-needs-mentor` issues to the [project board]
  - Mark `AsyncAwait-triaged`
- If there's still a shortage of **To do** issues, review the list of [`P-medium`] or [`P-low`] issues for candidates

## Mentoring

If an issue is a good candidate for mentoring, mark `E-needs-mentor` and try to find a mentor.

Mentors assigned to issues should write up mentoring instructions. 
**Often, this is just a couple lines pointing to the relevant code.** 
Mentorship doesn't require intimate knowledge of the compiler, just some familiarity and a willingness to look around for the right code.

After writing instructions, mentors should un-assign themselves, add `E-mentor`, and remove `E-needs-mentor`. 
On the project board, if a mentor is assigned to an issue, it should go to the **Claimed** column until mentoring instructions are provided. 
After that, it should go to **To do** until someone has volunteered to work on it.

[`A-async-await`]: https://github.com/rust-lang/rust/labels/A-async-await
[uncategorized issues]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AA-async-await+is%3Aopen+-label%3AAsyncAwait-Triaged&type=Issues
[`P-high`]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AAsyncAwait-Triaged+label%3AP-high+is%3Aopen&type=Issues
[`P-medium`]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AAsyncAwait-Triaged+label%3AP-medium+is%3Aopen&type=Issues
[`P-low`]: https://github.com/search?q=org%3Arust-lang+is%3Aissue+label%3AAsyncAwait-Triaged+label%3AP-low+is%3Aopen&type=Issues
