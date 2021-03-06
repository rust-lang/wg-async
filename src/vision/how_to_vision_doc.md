# ❓ How to vision doc

## What is this

This page describes the process for contributing to the vision doc.

[sq]: status_quo.md
[sf]: shiny_future.md
[r]: roadmap.md
[dd]: ../design_docs.md
[wgl]: ../welcome.md#leads
[conversations]: ../conversations.md

### 🚧 Under construction! Help needed! 🚧

This document is not yet complete! We are actively working on it as part of the working group, and we would like your help! This page talks about what you can do to contribute.

### Schedule and status

We are currently working to complete a first draft of the **status quo** section. 

### Who owns this document?

This document is owned and maintained by the leads of the Async Foundations Working Group.
They decide what content to accept or reject. 
This decision is made in consultation with the Rust teams that will be making the ultimate decisions. 
For example, if a design doc or part of the shiny future is describing a new language feature, the leads ought to discuss that with the language design team, since that team will ultimately have to approve the RFCs for its design.

## Ways to help

### Use Async Rust? Write blog posts about your experiences!

We want to make sure all Async Rust users and their experiences are reflected in the async vision doc, so please tell us your story! Share it in any form you want: blog posts, tweets, letters to the editor, whatever, as long as it has a URL. Be sure to include two things:

* Which of the [cast of characters][cc] fits you best? If none of them seem to fit you, then tell us what makes you different from them.
* What are some of the challenges that you've encountered? Bonus points if you write in narrative style.

After you write your post, let us know about it by submitting the URL using this [google form][gf]. Also, if you tweet about it, use the hashtag `#asyncvisiondoc`. If you want to read other people's stories, the URLs submitted via that form can be found [in this spreadsheet][gr].

### Adding or adjusting the axes or set of characters

Feel free to open PRs adjusting the values for the "axes" that cover the space of people using Async Rust, but keep in mind that we want this list to be relatively short and readable, so we may try to 

Similarly, you can propose a new axis, but it's probably best to bring that up on Zulip first to discuss.

Finally, if you'd like to propose an adjustment to one of the characters or their background, please discuss it on Zulip first. At this point, those charactes are not yet "fixed" so changes to them are definitely in scope. Once we start to accumulate more stories, however, that will get harder!

When proposing a new character, make sure to include the following FAQs (and potentially others):

* How would you describe NAME in terms of the [axes](./characters.md#axes)?
* What is most important to NAME about async Rust? Why?
* What is least important to NAME about async Rust? Why?
* What are key parts of NAME's background or story that distinguishes them from the other characters?

### Adding or adjusting applications

We aim for the list of applications to be comprehensive. Feel free to open PRs that either add new applications or adjust the wording on 
existing ones. The [wg leads] may choose however not to add an application, or to generalize an existing category instead.

Make sure to include the following FAQs (and potentially others):

* What makes this application different from most others?
* What are some examples of crates in this category?
    * We have to be careful with this, we don't want it to seem like we're picking favorites, but we also don't want the pressure to be comprehensive. Maybe we can link to crates.io tags or something?

### Adding or adjusting narratives (status quo, shiny future)

We are currently focused on filling out the "status quo" section. If you think you'd like to write a story there, feel free to come on Zulip and talk about it. You could either be completing one of the existing stories or creating a new one altogether.

When you write status quo stories, they are best supported by evidence. This may mean you want to add new [conversations] at the same time, thogh sometimes we just link or describe the conversations in the FAQ.

The narratives in the two sections have different characteristics. Here is a summary:

* [Status quo][sq] narrative
    * *establishes:* problem framing
    * *identifies:* root problems
    * *provides:* supporting evidence for your theses
    * *links to:* evidence, if you want to provide it
    * *Frequently asked questions:* Include at least the following questions:
        * What are the morals of the story?
        * What are the sources for this story?
        * Why did you choose *NAME* to tell this story?
        * How would this story have played out differently for the other characters?
* [Shiny future][sf] narrative
    * *establishes:* north star
    * *identifies:* key elements of the solution
    * *provides:* a detailed description of how things should work for Rust's users down the road
    * *links to:* design notes
    * *Frequently asked questions:* Include at least the following questions:
        * What is *NAME* most excited about and why?
        * What is *NAME* most disappointed with?
        * Which other characters are helped by this?
        * Are any characters upset by this?
        * What do the other characters think about this?
        * What key things did the Rust org do to make this happen?
        * What things did the other groups do to make this happen?

Whether it is status quo or shiny future, when you open the PR with your story, people will no doubt have many questions (e.g., why did Grace do X and not Y?). This may cause you to update your story; but if not, please update the FAQ for your story with the answer!

#### 🤔 Frequently Asked Questions

* What is the process to propose a status quo story?
    * You can write it and open a PR, but you can also just hop onto [Zulip] and let's talk about it!
* How should I describe the "evidence" for my status quo story?
    * The more specific you can get, the better. If you can link to tweets or blog posts, that's ideal. You can also add notes into the [conversations] folder and link to those. Of course, you should be sure people are ok with that.
* What is the process to propose a shiny future story?
    * We are not yet accepting shiny future stories. We will announce the process soon!
* What if my story applies to multiple characters?
    * Look at the "morals" of your story and decide which character will let you get those across the best.
    * Use the FAQ to talk about how other characters might have been impacted.
    * If the story would play out really differently for other characters, maybe write it more than once!
* None of the characters are a fit for my story.
    * Uh oh! Come talk to us on [Zulip] about it!

### Design docs

We're still working out the process around design docs! Ask on Zulip.

Include at least the following FAQs:

* What assumptions are you using as the basis for this design?
* Why not do ALTERNATIVE-X? (For each compelling alternative)
* What design decisions cannot easily be reversed?

### The roadmap

We're still working out the process around the roadmap! Ask on Zulip.
