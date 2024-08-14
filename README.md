ref_iter
===

Dynamic borrowing iterator.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides [iterator-like](#lending-iterator) types
for dynamic borrowing objects (`Ref` and `RefMut`).

## Examples

```rust
let samples = vec![1, 2, 3];
let src = RefCell::new(samples.clone());
let iter = RefIter::new(src.borrow(), |x| x.iter());
iter.cloned().eq(samples.iter().cloned());
```

## Main items

Trait items.

* `RefIterator` - Immutable dynamic borrowing iterator.
* `RefMutIterator` - Mutable dynamic borrowing iterator.

Type items.

* `RefIter` - `RefIterator` from [`Ref`].
* `RefMutIter` - `RefMutIterator` from [`RefMut`].

[`Ref`]: https://doc.rust-lang.org/std/cell/struct.Ref.html
[`RefMut`]: https://doc.rust-lang.org/std/cell/struct.RefMut.html

## Lending-iterator

The main iterator in this crate is a kind of Lending-iterator.

At lending-iterator, when it is destroyed, the item is also unavailable.
The core idea of this crate is to link this behavior with the destruction
of dynamic borrowing.

Note lending-iterator does not implement [`Iterator`]. Therefore, it does
not support iterator loop syntax (for-in). And also it does not support
various methods like `Iterator`. However, these are not so big problems.
Because, instead of iterator loop syntax, you can use other loop syntax
or this crate's macros (`for_ref` and `for_ref_mut`). And also, the lack
of methods can be covered by [iterator conversion](#iterator-conversion).

This crate is not keen on abstraction for lending-iterator. Because ideal
implementations of lending-Iterator requires GAT (Generic Associated Type).
However, as of 2024, GAT has some [limitations][gat-issue]. And workarounds
like [nougat] complicate the API. Therefore, We are not using GAT for this
crate for simplicity.

[gat-issue]:https://blog.rust-lang.org/2022/10/28/gats-stabilization.html
[nougat]:https://crates.io/crates/nougat
[`Iterator`]:https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html

## Iterator conversion

The following items provide cross-conversion of normal-iterator
and lending-iterator.

**Lending -> Normal**
* `RefIterator::cloned()`
* `RefIterator::map(f)` with `RefMap::into_iter()`
* `RefMutIterator::map_mut(f)` with `RefMutMap::into_iter()`

**Normal -> Lending**
* `IntoRefIter::new(i)`
* `IntoRefMutIter::new(i)`
* `RefIter::new(s, f)`
* `RefMutIter::new(s, f)`

## Under the hood

Unsafe operation is used.

About `RefIter`.

- Iterators taken from `Ref` are safe to use as long as `Ref` is available.
- However, borrow checker does not allow to save the iterator with `Ref`.
- Unsafe operation solves this problem by hiding origin of references.
