# impl Trait in traits

## Summary

* Able to write `-> impl Trait` in traits
* Able to write `type Foo<..> = impl Trait` in impls ([type alias impl trait](./tait.md), [generic associated types](./gats.md))

## Requires

* [Type alias impl trait](./tait.md)
* [Generic associated types](./gats.md)

## Design notes

Support `-> impl Trait` (existential impl trait) in traits. Core idea is to desugar such thing into a (possibly generic) associated type:

```rust
trait SomeTrait {
    fn foo<(&mut self) -> impl Future<Output = ()> + '_;
}

// becomes something like:
//
// Editor's note: The name of the associated type is under debate;
// it may or may not be something user can name, though they should
// have *some* syntax for referring to it.

trait SomeTrait {
    type Foo<'me>: Future<Output = ()> + 'me
    where
        Self: 'me;

    async fn foo(&mut self) -> Self::Foo<'_>;
}
```

We also need to support `-> impl Trait` in impls, in which case the body desugars to a "type alias impl trait":

```rust
impl SomeTrait for SomeType {
    fn foo<(&mut self) -> impl Future<Output = ()> + '_ {

    }
}

// becomes something using "type alias impl Trait", like this:

trait SomeTrait {
    type Foo<'me> = impl Future<Output = ()> + 'me
    where
        Self: 'me;

    fn foo(&mut self) -> Self::Foo<'_> {
        ...
    }
}
```

## Frequently asked questions

### What is the name of that GAT we introduce?

- I called it `Bar` here, but that's somewhat arbitrary, perhaps we want to have some generic syntax for naming the method?
- Or for getting the type of the method.
- This problem applies equally to other "`-> impl Trait` in trait" scenarios.
- [Exploration doc](https://hackmd.io/IISsYc0fTGSSm2MiMqby4A)

