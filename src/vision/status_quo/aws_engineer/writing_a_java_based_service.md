# Status quo of an AWS engineer: Writing a Java-based service

Alan has been working at AWS for the last six years. He's accustomed to a fairly standard workflow for launching Java-based services:

* Write a description of the service APIs using a modeling language like [Smithy](https://awslabs.github.io/smithy/).
* Submit the description to a webpage, which gives a standard service implementation based on [netty](https://netty.io/). Each of the API calls in the modeling language has a function with a `/* TODO */` comment to fill in.
* As Alan works with his team to fill in each  of those items, he makes use of a number of standard conventions:
    * Mocking with projects like [mockito] to allow for unit testing of specific components.
* Alan uses a variety of nice tools:
    * Advanced IDEs like IntelliJ, which offer him suggestions as he works.
    * Full-featured, if standard, debuggers; he can run arbitrary code, mutate state, step into and out of functions with ease. 
    * Tools for introspecting the VM state to get heap usage information and other profiling data.
    * Performance monitoring frameworks 
* As Alan is preparing to launch his service, he has to conduct an Operational Readiness Review (ORR):
    * This consists of a series of detailed questions covering all kinds of nasty scenarios that have arisen in deployments past. For each one, he has to explain how his service will handle it.
    * For most of them, the standard framework has pre-generated code that covers it, or he is able to use standard patterns.