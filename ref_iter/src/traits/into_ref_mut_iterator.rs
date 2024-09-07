//! Provider of [`IntoRefMutIterator`].

use crate::*;

/// Conversion into [`RefMutIterator`].
pub trait IntoRefMutIterator {
    /// Which kind of iterator are we turning this into?
    type IntoRefMutIter: RefMutIterator;

    /// Creates an iterator from a value.
    fn into_ref_mut_iter(self) -> Self::IntoRefMutIter;
}

impl<T: RefMutIterator> IntoRefMutIterator for T {
    type IntoRefMutIter = Self;

    fn into_ref_mut_iter(self) -> Self::IntoRefMutIter {
        self
    }
}
