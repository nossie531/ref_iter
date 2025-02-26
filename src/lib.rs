//! Dynamic borrowing iterator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*
//!
//! This crate provides lending iterator for dynamic
//! borrowing objects ([`Ref`] and [`RefMut`]).
//!
//! [`Ref`]: core::cell::Ref
//! [`RefMut`]: core::cell::RefMut
//!
//! # Examples
//!
//! ```
//! # use ref_iter::prelude::*;
//! # use std::cell::RefCell;
//! #
//! let samples = vec![1, 2, 3];
//! let src = RefCell::new(samples.clone());
//! let iter = RefIter::new(src.borrow(), |x| x.iter());
//! assert!(iter.icloned().eq(samples.iter().cloned()));
//! ```

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod closures;
pub mod macros;
pub mod prelude;

pub use ref_iter_macro;
pub use sub::*;
pub use traits::*;
pub use types::*;

mod sub;
mod traits;
mod types;
mod util;

#[doc(hidden)]
#[path = "../tests_compile_fail/mod.rs"]
mod tests_compile_fail;
