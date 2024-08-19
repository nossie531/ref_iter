//! Crate's base types.

mod into_ref_iter;
mod into_ref_mut_iter;
mod ref_iter_i;
mod ref_mut_iter_i;

#[doc(hidden)]
#[cfg(feature = "alloc")]
#[path = ""]
mod with_alloc {
    mod ref_iter;
    mod ref_mut_iter;
    pub use ref_iter::*;
    pub use ref_mut_iter::*;
}

pub use into_ref_iter::*;
pub use into_ref_mut_iter::*;
pub use ref_iter_i::*;
pub use ref_mut_iter_i::*;

#[cfg(feature = "alloc")]
pub use with_alloc::*;
