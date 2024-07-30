use crate::*;

/// Conversion into an [`RefIterator`].
pub trait IntoRefIterator {
    /// Which kind of iterator are we turning this into?
    type IntoRefIter: RefIterator;

    /// Creates an iterator from a value.
    fn into_ref_iter(self) -> Self::IntoRefIter;
}

impl<T: RefIterator> IntoRefIterator for T {
    type IntoRefIter = Self;

    fn into_ref_iter(self) -> Self::IntoRefIter {
        self
    }
}
