//! Provider of [`IntoRefMutIterator`].

use crate::prelude::*;

/// Conversion into an [`RefMutIterator`].
pub trait IntoRefMutIterator {
    /// The type of the elements being iterated over.
    type Item: ?Sized;

    /// Which kind of iterator are we turning this into?
    type IntoRefMutIter: RefMutIterator<Item = Self::Item>;

    /// Creates an iterator from a value.
    fn into_ref_mut_iter(self) -> Self::IntoRefMutIter;
}

impl<T: RefMutIterator> IntoRefMutIterator for T {
    type Item = T::Item;
    type IntoRefMutIter = Self;

    fn into_ref_mut_iter(self) -> Self::IntoRefMutIter {
        self
    }
}
