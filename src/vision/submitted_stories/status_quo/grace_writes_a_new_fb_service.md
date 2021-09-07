# 😱 Status quo stories: Template

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story

This tells the story of Grace, an engineer working at Facebook on C++
services.

* Grace writes C++ services at Facebook, built upon many libraries and support
  infrastructure
* Grace's last project had several bad bugs related to memory safety, and she
  is motivated to give Rust a shot on a new service she's writing
* First, she must determine if there are Rust bindings to the other FB
  services her new service will depend on
* She determines that she'll need to write a binding to the FooDB service
  using cxx
* She also determines that several crates she'll need from crates.io aren't
  vendored in the FB monorepo, so she'll need to get them and their
  dependencies imported. She'll need to address any version conflicts and
  special build rules since FB uses Buck and not Cargo to build all code
* While developing her service, Grace discovers that IDE features she's used
  to in VS Code don't always work for Rust code
* Grace writes up the performance and safety benefits of her new service after
  it's first month of deployment. Despite the tooling issues, the end result
  is a success

## 🤔 Frequently Asked Questions

*Here are some standard FAQ to get you started. Feel free to add more!*

### **What are the morals of the story?**

* Building successful Rust services in a company that has lots of existing
  tooling and infrastructure can be difficult, as Grace must do extra work
  when new ground is tread
  * Big companies like Facebook have large monorepos and custom build systems
    and the standard Rust tooling may not be useable in that environment
  * Facebook has a large team making developer's lives easier, but it is
    focused around the most common workflows, and Grace must work a little
    harder for now as Rust support is in its early days
  * Integrating with existing C++ code is quite important as Grace cannot
    rewrite existing services
    
### **What are the sources for this story?**

This story is compiled from internal discussions with Facebook engineers and
from internal reports of successful Rust projects.

### **Why did you choose Grace to tell this story?**

Both Alan or Grace could be appropriate, but I chose Grace in order to focus
on tooling and C++ service integration issues.

### **How would this story have played out differently for the other characters?**

Had I chosen Alan, a Python programmer at Facebook, there is probably a lot
more learning curve with Rust's async mechanics. Python programmers using
async don't necessarily have analogs for things like `Pin` for example.

[character]: ../../characters.md
[status quo stories]: ../status_quo.md
[Alan]: ../../characters/alan.md
[Grace]: ../../characters/grace.md
[Niklaus]: ../../characters/niklaus.md
[Barbara]: ../../characters/barbara.md
[htvsq]: ../status_quo.md
[cannot be wrong]: ../../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
