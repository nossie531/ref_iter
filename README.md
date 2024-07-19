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
let cell = RefCell::new(samples.clone());
let iter = RefIter::new(cell.borrow(), |x| x.iter());
assert!(iter.eq::<Iter<_>>(samples.iter()));
```

## Other Solution

See [Stack Overflow Question][AtStackoverflow].

This is good approach!

[AtStackoverflow]:[https://stackoverflow.com/questions/33541492]

## Main items

| Wrapper       | Target   | Approach       |
|---------------|----------|----------------|
| `RefIter`     | `Ref`    | Dynamic typing |
| `RefIterI`    | `Ref`    | Static typing  |
| `RefMutIter`  | `RefMut` | Dynamic typing |
| `RefMutIterI` | `RefMut` | Static typing  |

* Dynamic typing approach is simple in coding (Iterator type can omit).
* Static typing approach is bit fast in execution.
* Static typing approach can be used in `no_std` environment.

## Lending Iterator

The main iterator in this crate is a kind of Lending Iterator (though
for simplicity, it does not provide abstraction as Lending Iterator,
and focuses only on working with dynamic borrowing).

In Lending Iterator, when the iterator is destroyed, the item is also
unavailable. The core idea of this crate is to link this behavior with
the destruction of dynamic borrowing.

Lending Iterator does not implement [`Iterator`][Iterator]. Therefore,
it does not support for-in syntax, and the author is in the process of
implementing various methods on his own.

A common implementation of Lending Iterator uses GAT (Generic Associated
Type). However, as of 2024, GAT has some [limitations][gat-issue]. This
crate uses [nougat][nougat], a polyfill of GAT, to avoid them. However,
as a side effect, the type become a little more complicated.

[gat-issue]:[https://blog.rust-lang.org/2022/10/28/gats-stabilization.html]
[nougat]:[https://crates.io/crates/nougat]
[Iterator]:[https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html]

## Under the hood

Unsafe operation is used.

About `RefIter`.

- Iterators taken from `Ref` are safe to use as long as `Ref` is available.
- However, borrow checker does not allow to save the iterator with `Ref`.
- Unsafe operation solves this problem by hiding origin of references.
