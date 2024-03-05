/*! Dynamic borrowing iterator.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This crate provides iterators that can be created
from dynamic borrowing types ([`Ref`] and [`RefMut`]).

## Main items

| Wrapper         | Target     | Approach       |
|-----------------|------------|----------------|
| [`RefIter`]     | [`Ref`]    | Dynamic typing |
| [`RefIterI`]    | [`Ref`]    | Static typing  |
| [`RefMutIter`]  | [`RefMut`] | Dynamic typing |
| [`RefMutIterI`] | [`RefMut`] | Static typpng  |

* Dynamic typing approach is simple in coding (Iterator type can omit).
* Static typing approach is bit fast in execution.
* Static typing approach can be used in `no_std` environment.

[`RefIter`]: iter::RefIter
[`RefIterI`]: iter::RefIterI
[`RefMutIter`]: iter::RefMutIter
[`RefMutIterI`]: iter::RefMutIterI
[`Ref`]: core::cell::Ref
[`RefMut`]: core::cell::RefMut

# Examples

```
# use std::cell::RefCell;
# use ref_iter::iter::RefIter;
# use ref_iter::RefIterator;
#
let samples = vec![1, 2, 3];
let src = RefCell::new(samples.clone());
let iter = RefIter::new(src.borrow(), |x| x.iter());
let iter = iter.ref_map(|x, token| *x.get(token));
assert!(iter.eq(samples));
```
*/

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod iter;

mod into_ref_iterator;
mod ref_iterator;
mod sub;
mod util;

pub use into_ref_iterator::*;
pub use ref_iterator::*;
pub use sub::*;
