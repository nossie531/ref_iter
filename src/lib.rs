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
*/

#![no_std]

#![cfg_attr(docs, feature(doc_cfg))]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod iter;
pub mod sub;

mod ref_item;
mod ref_iterator;
mod ref_token;
mod util;

pub use ref_item::*;
pub use ref_iterator::*;
pub use ref_token::*;
