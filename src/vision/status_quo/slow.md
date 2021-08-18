# Creating a library for the SLOW protocol

## Goal: a portable library

* SLOW is an exciting new protocol
* Barbara sets out to build a reusable library
* She wants it to be usable by as many people as possible, so she wants it to be portable across runtimes and other environments, but how to do that?

## First attempt: define traits

* Starts to define traits but it's quite complicated
* Would have to make things generic

## Next attempt: cfg flags

* Starts to define cfg flags

## Gives up and just uses tokio

* Ends up using all kinds of things from the tokio library
* Factors out some logic into a "runtime independent core" but needs to have a lot of layers on both sides

## Tries later to separate out tokio

* Really hard, lots of small utilities scattered about