ref_iter
===

Dynamic borrowing iterator.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides iterators that can be created
from dynamic borrowing types (`Ref` and `RefMut`).

## Main items

| Wrapper       | Target   | Approach       |
|---------------|----------|----------------|
| `RefIter`     | `Ref`    | Dynamic typing |
| `RefMutIter`  | `RefMut` | Dynamic typing |
| `RefIterI`    | `Ref`    | Static typing  |
| `RefMutIterI` | `RefMut` | Static typing  |

* Dynamic typing approach is simple in coding (Iterator type can omit).
* Static typing approach is bit fast in execution.
* Static typing approach can be used in `no_std` environment.

## Examples

```rust
let samples = vec![1, 2, 3];
let src = RefCell::new(samples.clone());
let iter = RefIter::new(src.borrow(), |x| x.iter());
let iter = iter.ref_map(|x, token| *x.get(token));
assert!(iter.eq(samples));
```

## Under the hood

Points of `RefIter`.

- With unsafe operation, extend lifetime of dynamic borrowed value reference.
  - This reference is safe until `RefIter` (that stores `Ref`) is dropped.
  - This reference is passed to the iterator generation closure.
  - The closure is `Fn` to avoid spilling out the unsafable reference.
- Wrap the iterator to protect items by token.
  - Tokens are only provided within the callbacks of `RefIter` methods.
  - Lifetime of item reference depends on lifetime of token reference.
  - Lifetime of token reference depends on `RefIter` existance.
  - Therefore, when `RefIter` is droped, then item is inaccessible.
