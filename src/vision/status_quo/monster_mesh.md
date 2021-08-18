# Developing MonsterMesh, a system of embedded sensors

## Embassy

* Heard about the embassy crate
* Really cool that they can use `join!` and other concurrency constructs that don't require allocation
* Has to use unsafe keyword in some places because they need to promise not to "forget" the future, since they are using DMA and have no way to cancel the ongoing request

