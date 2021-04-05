# ⚡ Projects: YouBuy (Traditional Server Application)

## What is this?

This is a sample project for use within the various ["status quo"] or ["shiny future"] stories.

["status quo"]: ../status_quo.md
["shiny future"]: ../shiny_future.md

## Description

YouBuy is a growing e-commerce website that now has millions of users. The team behind YouBuy is struggling to keep up with traffic and keep server costs low. Having originally written YouBuy in a mix of Ruby on Rails and Node, the YouBuy team decides to rewrite many parts of their service in Rust which they've investigated and found to be performant while still allowing for high levels of abstraction they're used to.

## 🤔 Frequently Asked Questions

### **What makes YouBuy and other server applications different from others?**
* Many server applications are written in languages with garbage collectors. Many of the things that Rust forces users to care about are not first order concerns for those working on server applications (e.g., memory management, stack vs heap allocations, etc.). 
* Many server applications are written in languages without static type checking. The developers of YouBuy don't have much experience with statically typed languages and some of the developers early in their Rust learning journeys expressed frustration that they found it hard to get their programs to compile especially when using async constructs.

### **Does YouBuy require a custom tailored runtime?**
YouBuy should be perfectly fine with a runtime from crates.io. In fact, their concern isn't at the runtime level but at the high-level server framework level.

### **How much of this project is likely to be built with open source components from crates.io?**
YouBuy is in fierce competition with many other e-commerce sites. Therefore, the less that YouBuy engineers have to write themselves, the better. Ideally, YouBuy can focus 100% of its energy on features that differentiate it from its competition and none of its time on tweaking its networking stack.

### **What is of most concern to this project?**
It seems like YouBuy is always on the verge of either becoming the next billion-dollar company with hundreds of millions of users or completely going out of business. YouBuy needs to be able to move fast and focus on the application business logic.

### **What is of least concern to this project?**
Since moving fast is of primary concern, the ins and outs of the underlying networking stack are only of concern when something goes wrong. The hope is that that rarely if ever happens and when it does, it's easy to find the source of the issue.
