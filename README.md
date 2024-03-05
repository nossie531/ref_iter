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
let src = RefCell::new(samples);
let token = RefToken::new();
let mut iter = RefIter::new(src.borrow(), &token, |x| x.iter());
assert_eq!(iter.next().unwrap().get(&token), &1);
assert_eq!(iter.next().unwrap().get(&token), &2);
assert_eq!(iter.next().unwrap().get(&token), &3);
```

## Under the hood

Because some `unsafe` was required at inside,<br/>
we carefully considered the safety of our logic.<br/>
For example, here is the case of `RefIter`.

- `Ref` outlives effective span of `RefIter`.
  - So, while `RefIter` alive, `Ref`'s internal value can be accessed.
  - So, the internal value reference is extended by unsafe operation.
  - Its reference is passed to the iterator generation closure.
  - The closure is `Fn` to avoid spilling out the unsafe reference.
- `Ref` outlives effective span of `RefIter`'s each item.
  - Creating `RefIter` requires `Ref` and `RefToken`.
  - Access to `RefIter`'s each item requires `RefToken`.
