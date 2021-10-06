# Stakeholders

Many initiatives in the [roadmap] have an associated set of **stakeholders**. The role of a stakeholder is as follows:

* They are consulted by the owner over the course of working on the initiative.
* They do not have veto power; that belongs to the team.
* When they do raise concerns, those concerns should either be addressed in the design or discussed explicitly in the FAQ.

Stakeholders can be:

* Domain experts (perhaps from other languages)
* Representatives from major libraries
* Production users

Stakeholders can be selected in coordination with the async foundations working group leads. Potential new stakeholders can also get in touch with the owner.

## Feedback on the design

One role for stakeholders is to give feedback on the design as it progresses. Stakeholders are thus consulted in course of preparing evaluation docs or RFCs.

## Experimenting with the implementation

Another role for stakeholders is evaluating the implemenation. This is partiularly important for production users. Stakeholders might, for example, agree to port their code to use the nightly version of the feature and adapt it as the design evolves.

## Goals of the stakeholder program

The goal of the stakeholder program is to make Rust's design process even more inclusive. We have observed that existing mechanisms like the RFC process or issue threads are often not a very good fit for certain categories of users, such as production users or the maintainers of large libraries, as they are not able to keep up with the discussion. As a result, they don't participate, and we wind up depriving ourselves of valuable feedback. The stakeholder program looks to supplement those mechanisms with direct contact.

Another goal is to get more testing: one problem we have observed is that features are often developed and deployed on nightly, but production users don't really want to try them out until they hit stable! We would like to get some commitment from people to give things a try so that we have a better chance of finding problems before stabilization.

We want to emphasize that we welcome design feedback from **all Rust users**, regardless of whether you are a named stakeholder or not. If you're using async Rust, or have read through the designs and have a question or idea for improvement, please feel free to open an issue on the appropriate repository.