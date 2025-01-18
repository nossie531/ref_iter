ref_iter
===

Dynamic borrowing iterator.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides [iterator-like](#lending-iterator) types
for dynamic borrowing objects ([`Ref`] and [`RefMut`]).

[`Ref`]:https://doc.rust-lang.org/core/cell/struct.Ref.html
[`RefMut`]:https://doc.rust-lang.org/core/cell/struct.RefMut.html

## Examples

```rust
let samples = vec![1, 2, 3];
let src = RefCell::new(samples.clone());
let iter = RefIter::new(src.borrow(), |x| x.iter());
assert!(iter.cloned().eq(samples.iter().cloned()));
```

## Main items

**Traits**
* `RefIterator` - Immutable dynamic borrowing iterator.
* `RefKvIterator` - Immutable dynamic borrowing key-value iterator.
* `RefMutIterator` - Mutable dynamic borrowing iterator.
* `RefMutKvIterator` - Mutable dynamic borrowing key-value iterator.

**Types**
* `RefIter` - Iterator from `Ref`.
* `RefMutIter` - Iterator from `RefMut`.

## Lending-iterator

The main iterator in this crate is a kind of lending-iterator.
At lending-iterator, when it is destroyed, the item is also unavailable.

Core idea of this crate is to link this lending iterator behavior
with the destruction of dynamic borrowing.

Note lending-iterator does not implement [`Iterator`].
So, followings are not supported.

* For-in loop syntax 
* Various methods of `Iterator` 

However, these are not so big problems. Because, instead of for-in loop,
[loop macros](#loop-macros) is supported. Instead of iterator methods,
[iterator conversion](#iterator-conversion) is supported. And also,
we can use iterator methods at lending-iterator construction.

[`Iterator`]:https://doc.rust-lang.org/core/iter/trait.Iterator.html

### Old-fashioned implementation.

Currently, this crate's lending-Iterator implementation is not so cool.
Because ideal its implementations requires GAT (Generic Associated Type).
However, as of 2024, GAT has some [limitations][gat-issue]. And workarounds
like [nougat] complicate the API. Therefore, We are not using GAT for this
crate for simplicity.

If future Rust resolve this problem, this crate can delete many types and
traits. for example, `RefMutIterator` will be merged into `RefIterator`.

[gat-issue]:https://blog.rust-lang.org/2022/10/28/gats-stabilization.html
[nougat]:https://crates.io/crates/nougat

## Loop macros

Followings are macros to perform for-in loops with various lending-iterator.

* `for_ref` - Immutable loop.
* `for_ref_kv` - Immutable key-value loop.
* `for_ref_mut` - Mutable loop.
* `for_ref_mut_kv` - Mutable key-value loop.

For example, `for_ref` macro can be used as follows.

```rust
let samples = vec![1, 2, 3];
let cell = RefCell::new(samples.clone());
let iter = RefIter::new(cell.borrow(), |x| x.iter());
let mut sum = 0;

for_ref!(x in iter {
    sum += *x
});

assert_eq!(sum, 6);
```

## Iterator conversions

The following items provide cross-conversion of normal-iterator
and lending-iterator.

**Lending -> Normal**
* `RefIterator::cloned()`
* `RefIterator::map(f)`
* `RefKvIterator::map(f)`
* `RefMutIterator::map_mut(f)`
* `RefMutKvIterator::map_mut(f)`

**Normal -> Lending**
* `IntoRefIter::new(i)`
* `IntoRefMutIter::new(i)`
* `RefIter::new(s, f)`
* `RefMutIter::new(s, f)`

## Under the hood

Unsafe operation is used.

For example, about `RefIter`.

* Iterators taken from `Ref` are safe to use as long as `Ref` is available.
* However, borrow checker does not allow to save the iterator with `Ref`.
* Unsafe operation solves this problem by hiding origin of references.

## What's new

v0.2.0

* Add `RefIterator::flat_map` method.
* Fix `ExactSizeRefIterator` base trait.
