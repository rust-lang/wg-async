# 😱 Status quo stories: Barbara Builds a Hydrodynamics Simulator

## 🚧 Warning: Draft status 🚧

This is a draft "status quo" story submitted as part of the brainstorming period. It is derived from real-life experiences of actual Rust users and is meant to reflect some of the challenges that Async Rust programmers face today. 

If you would like to expand on this story, or adjust the answers to the FAQ, feel free to open a PR making edits (but keep in mind that, as they reflect peoples' experiences, status quo stories [cannot be wrong], only inaccurate). Alternatively, you may wish to [add your own status quo story][htvsq]!

## The story
### Problem
Barbara needed to build a tool to solve hydrodynamics simulations; there is a common method for this that subdivides a region into a grid and computes the solution for each grid patch. All the patches in a grid for a point in time are independent and can be computed in parallel, but they are dependent on neighboring patches in the previously computed frame in time.  This is a well known computational model and the patterns for basic parallelization are well established.

Barabara wanted to write a performant tool to compute the solutions to the simulations of her research.  She chose Rust because she was already familiar with it and it had good qualities for writing performant code. After implementing the core mathematical formulas, Barbara began implementing the parallelization architecture. 

Her first attempt to was to emulate a common CFD design pattern: using message passing to communicate between processes that are each assigned a specific patch in the grid. So she assign one thread to each patch and used messages to communicate solution state to dependent patches.  With one thread per patch this usually meant that there were 5-10x more threads than CPU cores.

This solution was fine, but Barbara was not satisified.  She had two problems with it: first, she didn't like that the design would greedily use all the resources on the machine and, second, when her team added a new feature (tracer particles) that increased the complexity of how patches interact with each other and the number of messages that were passsed between threads increased to the point that it became a performance bottleneck and parallel processing became no faster than serial processing. So, Barbara decided to find a better solution.

### Solution Path
What Barbara wanted to do was find a way to more efficiently use threads: have a fixed number of threads that each mapped to a core on the CPU and assign patches to those threads as patches became ready to compute. The design of the `async` framework seemed to provide exactly that behavior. And to move away from the message passing design, because the number of messages being passed was proportional to the number of trace particles being traced.

As Barbara began working on her new design with `tokio`, her use of `async` went from a general (from the textbook) use of basic `async` features to a more specific implementation leveraging exactly the features that were most suited for her needs.

At first, Barbara was under a false impression about what async executors do. She had assumed that a multi-threaded executor could automatically move the execution of an async block to a worker thread. Then she discovered that async tasks must be explicitly spawned into into a thread pool if they are to be executed on a worker thread. This meant that the algorithm to be parallelized became strongly coupled to both the spawner and the executor. Code that used to cleanly express a physics algorithm now had interspersed references to the task spawner, not only making it harder to understand, but also making it impossible to try different execution strategies, since with Tokio the spawner and executor are the same object (the Tokio runtime). Barbara feels that a better design for data parallelism would enable better separation of concerns: a group of interdependent compute tasks, and a strategy to execute them in parallel.

In order to remove the need for message passing, Barbara moved to a shared state design: she would keep a table tracking the solution state for every grid patch and a specific patch would only start its computation task when solutions were written for all the patches it was dependent on. So, each task needs to access the table with the solution results of all the other tasks. Learning how to properly use shared data with `async` was a new challenge. The initial design:

```rust
    let mut stage_primitive_and_scalar = |index: BlockIndex, state: BlockState<C>, hydro: H, geometry: GridGeometry| {
        let stage = async move {
            let p = state.try_to_primitive(&hydro, &geometry)?;
            let s = state.scalar_mass / &geometry.cell_volumes / p.map(P::lorentz_factor);
            Ok::<_, HydroError>( ( p.to_shared(), s.to_shared() ) )
        };
        stage_map.insert(index, runtime.spawn(stage).map(|f| f.unwrap()).shared());
    };
```
lacked performance because she needed to clone the value for every task.  So, Barbara switched over to using `Arc` to keep a thread safe RC to the shared data. But this change introduced a lot of `.map` and `.unwrap` function calls, making the code much harder to read. She realized that managing the dependency graph was not intuitive when using `async` for concurrency.

During the move to `async` Barbara ran into a steep learning curve with error handling. The initial version of her design just used `panic!`s to fail the program if an error was encountered, but the stack traces were almost unreadable. She asked her teammate Grace to migrate over to using the `Result` idiom for error handling and Grace found a major inconvenience. The Rust type inference inconsistently breaks when propagating `Result` in `async` blocks. Grace frequently found that she had to specify the type of the error when creating a result value:
```rust
Ok::<_, HydroError>( ( p.to_shared(), s.to_shared() ) )  
```
And she could not figure out why she had to add the `::<_, HydroError>` to some of the `Result` values.

Once her team began using the new `async` design for their simulations, they noticed an important issue that impacted productivity: compilation time had now increased to between 30 and 60 seconds. The nature of their work requires frequent changes to code and recompilation and 30-60 seconds is long enough to have a noticeable impact on their quality of life.  What her and her team want is for compilation to be 2 to 3 seconds. Barbara believes that the use of `async` is a major contributor to the long compilation times.

This new solution works, but Barbara is not satisfied with how complex her code wound up becoming with the move to `async` and the fact that compilation time is now 30-60 seconds.  The state sharing adding a large amount of cruft with `Arc` and `async` is not well suited for using a dependency graph to schedule tasks so implementing this solution created a key component of her program that was difficult to understand and pervasive. Ultimately, her conclusion was that `async` is not appropriate for parallelizing computational tasks. She will be trying a new design based upon Rayon in the next version of her application.

## 🤔 Frequently Asked Questions

### **What are the morals of the story?**
- `async` looks to be the wrong choice for parallelizing compute bound/computational work
- There is a lack of guidance to help people solving such problems get started on the right foot
- Quality of Life issues (compilation time, type inference on `Result`) can create a drag on users ability to focus on their domain problem

### **What are the sources for this story?**
This story is based on the experience of building the [kilonova](https://github.com/clemson-cal/app-kilonova) hydrodynamics simulation solver.

### **Why did you choose *NAME* to tell this story?**
I chose Barbara as the primary character in this story because this work was driven by someone with experience in Rust specifically but does not have much systems level experience. Grace was chosen as a supporting character because of that persons experience with C/C++ programming and to avoid repeating characters.

### **How would this story have played out differently for the other characters?**
For Alan, there's a good chance he would have already had experience working with either async workflows in another language or doing parallelization of compute bound tasks; and so would already know from experience that `async` was not the right place to start.  Grace, likewise, might already have experience with problems like this and would know what to look for when searching for tools. For Niklaus, the experience would probably be the same, as it's very easy to assume that `tokio` is the starting place for concurrency in Rust.

[character]: ../characters.md
[status quo stories]: ./status_quo.md
[Alan]: ../characters/alan.md
[Grace]: ../characters/grace.md
[Niklaus]: ../characters/niklaus.md
[Barbara]: ../characters/barbara.md
[htvsq]: ../how_to_vision/status_quo.md
[cannot be wrong]: ../how_to_vision/comment.md#comment-to-understand-or-improve-not-to-negate-or-dissuade
