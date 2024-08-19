/*! Dynamic borrowing iterator.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This crate provides lending iterator for dynamic
borrowing objects ([`Ref`] and [`RefMut`]).

[`Ref`]: core::cell::Ref
[`RefMut`]: core::cell::RefMut

## Main items

Trait items.

* [`RefIterator`] - Immutable dynamic borrowing iterator.
* [`RefMutIterator`] - Mutable dynamic borrowing iterator.

Type items.

| Wrapper         | Main trait         | Approach       |
|-----------------|--------------------|----------------|
| [`RefIter`]     | [`RefIterator`]    | Dynamic typing |
| [`RefIterI`]    | [`RefIterator`]    | Static typing  |
| [`RefMutIter`]  | [`RefMutIterator`] | Dynamic typing |
| [`RefMutIterI`] | [`RefMutIterator`] | Static typing  |

* Dynamic typing approach is simple in coding (Iterator type can omit).
* Static typing approach is bit fast in execution.
* Static typing approach can be used in `no_std` environment.

# Examples

```
# use core::cell::RefCell;
# use ref_iter::{RefIterI, RefIterator};
#
let samples = vec![1, 2, 3];
let src = RefCell::new(samples.clone());
let iter = RefIterI::new(src.borrow(), |x| x.iter());
assert!(iter.cloned().eq(samples.iter().cloned()));
```
*/

#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod closure;
pub mod prelude;

mod macros;
mod sub;
mod traits;
mod types;
mod util;

pub use sub::*;
pub use traits::*;
pub use types::*;
