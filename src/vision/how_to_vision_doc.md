# ❓ How to vision doc

This page describes the process for contributing to the vision doc.

[sq]: status_quo.md
[sf]: shiny_future.md
[r]: roadmap.md
[dd]: ../design_docs.md
[wgl]: ../welcome.md#leads

## 🚧 Under construction! Help needed! 🚧

This document is not yet complete! We are actively working on it as part of the working group, and we would like your help! This page talks about what you can do to contribute.

## Schedule and status

We are currently working to complete a first draft of the **status quo** section. 

## Use Async Rust? Tell us about your experiences!

We want to make sure all Async Rust users and their experiences are reflected in the async vision doc, so please tell us your story! Share it in any form you want: blog posts, tweets, letters to the editor, whatever, as long as it has a URL. Be sure to include two things:

* Which of the [cast of characters][cc] fits you best? If none of them seem to fit you, then tell us what makes you different from them.
* What are some of the challenges that you've encountered? Bonus points if you write in narrative style.

After you write your post, let us know about it by submitting the URL using this [google form][gf]. Also, if you tweet about it, use the hashtag `#asyncvisiondoc`. If you want to read other people's stories, the URLs submitted via that form can be found [in this spreadsheet][gr].

## Who owns this document?

This document is owned and maintained by the leads of the Async Foundations Working Group.
They decide what content to accept or reject. 
This decision is made in consultation with the Rust teams that will be making the ultimate decisions. 
For example, if a design doc or part of the shiny future is describing a new language feature, the leads ought to discuss that with the language design team, since that team will ultimately have to approve the RFCs for its design.

## How can I finish or add a section to the document?

If you'd like to volunteer to finish a section in the document, ping the [working group leads][wgl] on Zulip, either in privmsg or the `#wg-async-foundations` stream.

If you think you have a new section you'd like to add, write up a short description or general outline and float it on `#wg-async-foundations`. Again, please ping the [working group leads][wgl].

Assuming you get positive feedback, you can open a PR. The PR should add one or more of the following things:

* A new section in the [status quo][sq] document, or edits to an existing section
    * This may be accompanied by new evidence, though it doesn't have to be.
* A new section in the [shiny future][sf] document, or edits to an existing section
    * This may be accompanied by new [design docs][dd], or edits to existing ones.
    * It doesn't have to be, but the PR discussion process may lead to requests to write design docs to help people imagine how this future could come about.
* New entries in the [roadmap][r], for work you propose to do.
    * This may be accompanied by new design docs, or edits to existing ones.

## Editing the narratives

The narratives in the vision doc are there to establish the state of how things are (the "status quo"), and the ultimate goals we are working towards (the "shiny future"). The goals in the shiny future are not necessarily meant to be achievable in a short time frame. Think about how you would like things to look 3-4 years down the road -- that is the "shiny future" we are describing.

Each section have very different characteristics. Here is a summary:

* [Status quo][sq] narrative
    * *establishes:* problem framing
    * *identifies:* root problems
    * *provides:* supporting evidence for your theses
    * *links to:* evidence, if you want to provide it
* [Shiny future][sf] narrative
    * *establishes:* north star
    * *identifies:* key elements of the solution
    * *provides:* a detailed description of how things should work for Rust's users down the road
    * *links to:* design notes

## The "Frequently Asked Questions" section

The FAQ section is there to elaborate on the narrative and to answer common questions that people have when reading it. Assembling the FAQ always begins with a few required questions. After that, you can add whatever questions come to mind -- but usually the FAQ is populated by literally recording your answers to the questions that people ask you. 

### Required and suggested FAQs

The required FAQs vary by depending on what kind of section it is.

* Status quo
    * What are the morals of the story?
    * Are you just making this stuff up?
    * In addition to the above, add FAQs 
* Shiny future
    * What is *NAME* most excited about and why?
    * What is *NAME* most disappointed with?
    * What key things did the Rust org do to make this happen?
    * What things did the other groups do to make this happen?
* Characters
    * What is most important to NAME about async Rust? Why?
    * What is least important to NAME about async Rust? Why?
    * What are key parts of NAME's background or story that distinguishes them from the other characters?
* Design docs
    * What assumptions are you using as the basis for this design?
    * Why not do ALTERNATIVE-X? (For each compelling alternative)
    * What design decisions cannot easily be reversed?

## The roadmap

The roadmap describes the work we plan to do over the next year. It consists of a series of design documents. If you want to add something to the list, the first thing is to create the relevant design doc, and then talk to the [working group leads][wgl].
