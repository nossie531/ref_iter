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
| [`RefMutIterI`] | [`RefMut`] | Static typing  |

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
# use core::cell::RefCell;
# use ref_iter::{AsRefIter, RefIterator};
# use ref_iter::iter::RefIter;
#
let samples = vec![1, 2, 3];
let cell = RefCell::new(samples.clone());
let iter = RefIter::new(cell.borrow(), |x| x.iter());
assert!(iter.eq::<AsRefIter<_>>(samples.iter().into()));
```
*/

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
extern crate nougat;

pub mod iter;
pub use into_ref_iterator::*;
pub use ref_iterator::*;
pub use sub::*;

mod into_ref_iterator;
mod macros;
mod ref_iterator;
mod sub;
mod util;
