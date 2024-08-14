/*! Dynamic borrowing iterator.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This crate provides lending iterator for dynamic
borrowing objects ([`Ref`] and [`RefMut`]).

## Main items

Trait items.

* [`RefIterator`] - Immutable dynamic borrowing iterator.
* [`RefMutIterator`] - Mutable dynamic borrowing iterator.

Type items.

* [`RefIter`] - [`RefIterator`] for [`Ref`].
* [`RefMutIter`] - [`RefMutIterator`] for [`RefMut`].

[`Ref`]: core::cell::Ref
[`RefMut`]: core::cell::RefMut

# Examples

```
# use core::cell::RefCell;
# use ref_iter::{RefIter, RefIterator};
#
let samples = vec![1, 2, 3];
let src = RefCell::new(samples.clone());
let iter = RefIter::new(src.borrow(), |x| x.iter());
assert!(iter.cloned().eq(samples.iter().cloned()));
```
*/

#![no_std]
#![warn(missing_docs)]

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
