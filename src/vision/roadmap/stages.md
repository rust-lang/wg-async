# Deliverable stages

The stages of a deliverable are as follows. Note that all stages do not necessarily apply to all deliverables. For example, the "async book" doesn't really require an RFC.

**Not all the stages apply to all deliverables.** For example, one of the deliverables is an improved async book, and no RFC is required there. Other deliverables live outside the Rust org, so "RFC" and "stability" may not apply (depending on whether the deliverable's home has those concepts, or something we can map to those concepts).

## Experimentation

This is a call for people in the community to do experiments aimed at fleshing out the design space. We will try to collect those experiments and list them so that later we can survey what approaches people found. This is used when there is a "wide open" design space that hasn't really been explored. In many cases, the prototypes here may rely on features not yet fully implemented, but for which a procedural macro or other "mock-up" can be used.

## Evaluation

An *evaluation* is a document that captures the design space and the tradeoffs. It is best done after experimentation has occurred. The evaluation is not an RFC: it is an attempt to narrow the design space to a "menu" of the 2 or 3 best approaches. The process for creating evaluations is described below. We are trying to do better than "wide open" discussion, and instead intentionally involve known stakeholders in a structured fashion.

### Evaluation plan

A lot of the issues here involve a lot of coordination amongst different kinds of stakeholders. We wish to adopt a structured model to help these conversations proceed successfully. The idea is based around the lang team [initiatives] structure, which means that each of the areas is assigned an **owner**, who is responsible for developing the "menu" of design choices, elaborating on the tradeoffs involved, and making a recomendation. Final choices are made by the relevant Rust team(s).

#### Stakeholders

In addition to the owner, each project will maintain a list of **stakeholders**. The role of a stakeholder:

* Consulted by the owner to aid in preparation of the report
* Do not have veto power; that belongs to the team
* All of their concerns should either be addressed in the design or discussed explicitly in the FAQ

Stakeholders can be:

* Domain experts (perhaps from other languages)
* Representatives from major libraries
* Production users

Stakeholders can be selected in coordination with the async foundations working group leads. Potential new stakeholders can also get in touch with the owner.

#### Developing an evaluation

A number of tasks below begin with an **evaluation**. An evaluation is a write-up of the various design options and their tradeoffs, coupled with a recommendation. This is presented to the relevant Rust teams which will discuss with the owner and ultimately make a choice on how to proceed.

The current draft for each evaluation will be maintained on a dedicated repository. The repository will also list the stakeholders associated with that particular effort. 

Developing an evaluation consists of first preparing an initial draft by surveying initial work and then taking the following steps (repeat until satisfied):

* Review draft in meetings with stakeholders
    * These meetings can be a small, productive group of people
    * Often better to have multiple stakeholders together so people can brainstorm together, but 1:1 may be useful too
* Present the draft to the teams and take feedback
* Review issues raised on the repo (see below)
* Adjust draft in response to the above comments

#### Issues on the repo

In addition to the active outreach to stakeholders, people can submit feedback by opening issues on the repositories storing the draft evaluations. These reposies will have issue categories with templates that categorize the feedback and provide some structure. For example:

* Experience report
* Proposal feedback
* Crazy new idea

## RFC

The *RFC* is the point where we make a specific proposal. It draws on the results from the evaluation.

## Feature complete

A deliverable is "feature complete" when all of the implementation work is done and it's ready for experimentation on nightly.

## Stable

A deliverable is "stable" when it is fully documented and available for use on the stable channel. This is the end goal, most of the time!
