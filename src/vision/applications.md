# ⚡ Applications

## What is this

This section describes the kinds of projects that people build using Async Rust. As you can see, there are quite a few, and we're surely missing some! For each application area, we also have a short description and some frequently asked questions that help to identify what makes this application area special. 

### Don't find your application here?

We just started creating this list, and it's obviously very incomplete. We would welcome PRs to flesh it out!

## Application areas

### Operating systems

Core system services like filesystems and networking. IPC between various programs and services. Kernel and driver code; waiting on device I/O.

#### 🤔 Frequently Asked Questions

* What separated this application area from the others?
    * Latency profile: Typically 100's-1,000's of ns, uncontended, for IPC

## Brainstorming

Here is a list of applications we came up with when brainstorming. We'd welcome PRs to convert these into full blown "application area" descriptions!

- Web site
- High performance server
- High performance disk I/O
- Web framework
- Protocol or other leaf libraries (e.g., HTTP, QUIC, Redis, etc)
- SDKs for web services
- Middleware and other sorts of "wrapper" libraries (e.g., async-compression)
- Media streaming
- Consumer of web services (e.g., running things on a compute service, storing and retrieving things with a storage service)
- Embedded 
- GUI application
- Parallel data processing
- Distributed HPC (compute clusters)
- Database clients
- Database servers
- Async runtimes for others to use
- Operating systems
- More?

