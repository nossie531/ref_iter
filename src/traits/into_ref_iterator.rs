//! Provider of [`IntoRefIterator`].

use crate::prelude::*;

/// Conversion into [`RefIterator`].
pub trait IntoRefIterator {
    /// The type of the elements being iterated over.
    type Item: ?Sized;

    /// Which kind of iterator are we turning this into?
    type IntoRefIter: RefIterator<Item = Self::Item>;

    /// Creates an iterator from a value.
    fn into_ref_iter(self) -> Self::IntoRefIter;
}

impl<T: RefIterator> IntoRefIterator for T {
    type Item = T::Item;
    type IntoRefIter = Self;

    fn into_ref_iter(self) -> Self::IntoRefIter {
        self
    }
}
