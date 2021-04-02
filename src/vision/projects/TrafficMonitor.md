# ⚡ Projects: TrafficMonitor (Custom Infrastructure)

## What is this?

This is a sample project for use within the various ["status quo"] or ["shiny future"] stories.

["status quo"]: ../status_quo.md
["shiny future"]: ../shiny_future.md

## Description

TrafficMonitor is a utility written by AmoogleSoft, a public cloud provider, for monitoring network traffic as it comes into its data centers to prevent things like distributed denial-of-service attacks. It monitors *all* network traffic, looking for patterns, and deciding when to take action against certain threat vectors. TrafficMonitor runs across almost all server racks in a data center, and while it does run on top of an operating system, it is resource conscious. It's also extremely important that TrafficMonitor stay running and handle network traffic with as few "hiccups" as possible. TrafficMonitor is highly tuned to the needs of AmoogleSoft's cloud offering and won't run anywhere else.

## 🤔 Frequently Asked Questions

### **What makes networking infrastructure projects like TrafficMonitor different from others?**
    * Networking infrastructure powers entire datacenters or even public internet infrastructure, and as such it is imperative that it run without failure.
    * It is also extremely important that such projects take few resources as possible. Being on an operating system and large server racks *may* mean that using the standard library is possible, but memory and CPU usage should be kept to a minimum.
    * This project is worked on by software developers with different backgrounds. Some are networking infrastructure experts (usually using C) while others have experience in networked applications (usually using GCed languages like Java, Go, or Node).

### **Does TrafficMonitor require a custom tailored runtime?**
Maybe? TrafficMonitor runs on top of a full operating system and takes full advantage of that operating systems networking stack. It's possible that a runtime meant for server workloads will work with TrafficMonitor.

### **How much of this project is likely to be built with open source components from crates.io?**
    * TrafficMonitor is highly specialized to the internal workings of AmoogleSoft's public cloud offering. Thus, "off-the-shelf" solutions will only work if they're highly flexible and highly tuneable. 
    * TrafficMonitor is central to AmoogleSoft's success meaning that getting things "just right" is much more important than having something from crates.io that mostly works but requires little custom tuning.

### **What is of most concern to this project?**
    * Reliability is the number one concern. This infrastructure is at the core of the business - it needs to work extremely reliable. A close second is being easily monitorible. If something goes wrong, AmoogleSoft needs to know very quickly what the issue is.
    * AmoggleSoft is a large company with many existing custom tooling for building, monitoring, and deploying its software. TrafficMonitor has to play nicely in a world that existed long before it came around.

### **What is of least concern to this project?**
AmoogleSoft is a large company with time and resources. High-level frameworks that remove control in favor of peak developer productivity is not what they're after. Sure, the easier things are to get working, the better, but that should not be at the sacrifice of control.
