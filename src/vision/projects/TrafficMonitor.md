# ⚡ Projects: TrafficMonitor (Networking Infrastructure)

## What is this?

This is a sample project for use within the various ["status quo"] or ["shiny future"] stories.

["status quo"]: ../status_quo.md
["shiny future"]: ../shiny_future.md

## Description

TrafficMonitor is a utility written by AmoogleSoft, a public cloud provider, for monitoring network traffic as it comes into its data centers to prevent things like distributed denial-of-service attacks. It monitors *all* network traffic, looking for patterns, and deciding when to take action against certain threat vectors. TrafficMonitor runs across almost all server racks in a data center, and while it does run on top of an operating system, it is resource conscious. It's also extremely important that TrafficMonitor stay running and handle network traffic with as few "hiccups" as possible. 

## 🤔 Frequently Asked Questions

* **What makes networking infrastructure projects like TrafficMonitor different from others?**
* Networking infrastructure powers entire datacenters or even public internet infrastucture, and as such it is imperative that it run without failure.
* It is also extremely important that such projects take few resources as possible. Being on an operating system and large server racks *may* mean that using the standard library is possible, but memory and CPU usage should be kept to a minimum.
* This project is worked on by software developers with different backgrounds. Some are networking infrastructure experts (usually using C) while others have experience in networked applications (usually using GCed languages like Java, Go, or Node).
* **Does TrafficMonitor require a custom tailored runtime?**
* Maybe? TrafficMonitor runs on top of a full operating system and takes full advantage of that operating systems networking stack. It's possible that a runtime meant for server workloads will work with TrafficMonitor.
