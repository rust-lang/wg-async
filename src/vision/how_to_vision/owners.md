# Owning a goal or initiative

This page describes the roles and responsibilities associated with being the **owner** of an item on the [roadmap](../roadmap.md). Roadmap items fall into two categories, top-level goals and initiatives. In both cases, being an owner means that you are responsible for ensuring that the item gets done, but the details of owning a top-level goal are different from owning an initiative.

## Summary

Goal owners are responsible for splitting their area into a set of **initiatives**. These can be active or on hold.

They are also responsible for ensuring that for each active initiative:

- An owner is assigned
- A landing page exists (see below)
- Milestones are defined on the landing page
- Stakeholders are identified and looped in at the proper stages

Finally, they are expected to attend sprint meetings.

## Sprint meetings

We are organizing the working group in **two week sprints**. This means that every two weeks we have a sprint planning meeting. **All goal owners are expected to attend!** Initiative owners or other contributors are welcome as well.

The purpose of the sprint planning meeting is to check-in on the progress towards the milestones for each initiative and to see if they need to be adjusted. It's also a chance to raise interesting questions or get advice about tricky things or unexpected problems, as well as to celebrate our progress.

## Owning a top-level goal

As the owner of a **top-level goal** your role is to figure out overall plan for how that goal will be achieved and to track progress. This means breaking up the goal into different initiatives, finding owners for those initiatives (which can be you!), and helping those owners to plan milestones. You are also generally responsible for staying on top of the state of things and updating other owners as to new or interesting developments.

## Owning an initiative

Our definition of [initiative] is precisely the same as that used by the Rust lang team: it corresponds to a some active effort with a clear goal or deliverable(s). As the owner of an initiative, your role is to ensure that the work gets done (Which doesn't necessarily mean you do it yourself, it may be that you instead coordinate with volunteers or other implementors). You also guide the design of the deliverables within the initiative.

As in the lang team process, the role of the owner is not to make the final decision (that belongs to the relevant rust team(s)), but to develop the "menu" of design choices, elaborate the tradeoffs involved, and make recommendations. For particularly complex designs, these evaluations will take the form of [evaluation documents] and are developed in collaboration with a defined set of [stakeholders].

[initiative]: https://lang-team.rust-lang.org/initiatives.html
[initiative owners]: https://lang-team.rust-lang.org/initiatives/process/roles/owner.html
[evaluation documents]: ./evaluations.md

### Making a landing page

Each initiative should have a landing page, linked to from the [roadmap]. This can be a page on this website or a dedicated repo. It should include, or have pointers to:

- Goals and impact of the initiative
- Milestones
- Design notes and documentation
- Links to any organizing tools, such as a project board
- The initiative owner
- The current set of [stakeholders] and the area(s) they represent
- Notes on how to get involved
- For landing pages not on this website, a link back to the overall [roadmap]

For making a dedicated repo, it's recommended to use this [initiative template][template] as a starting point.

[roadmap]: ../roadmap.md
[template]: https://github.com/rust-lang/initiative-template

### Planning initiative milestones

When you own an initiative, you should work with the owner of the top-level goal and others to plan out a series of **milestones** around the initiative. These milestones correspond to the various steps you need to take to complete the initiative.

Milestones are not fixed and they frequently change as you progress. They usually start out quite vague, such as "author an RFC", and then get more precise as you learn more about what is required: "figure out the design for X", "implement feature Y". We update the status and set of milestones for each sprint status meeting.

[stakeholders]: ./stakeholders.md