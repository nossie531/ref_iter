//! Provider of [`IntoRefKvIterator`].

use crate::prelude::*;

/// Conversion into an [`RefKvIterator`].
pub trait IntoRefKvIterator {
    /// The key type of the elements being iterated over.
    type K: ?Sized;

    /// The value type of the elements being iterated over.
    type V: ?Sized;

    /// Which kind of iterator are we turning this into?
    type IntoRefKvIter: RefKvIterator<K = Self::K, V = Self::V>;

    /// Creates an iterator from a value.
    fn into_ref_kv_iter(self) -> Self::IntoRefKvIter;
}

impl<T: RefKvIterator> IntoRefKvIterator for T {
    type K = T::K;
    type V = T::V;
    type IntoRefKvIter = Self;

    fn into_ref_kv_iter(self) -> Self::IntoRefKvIter {
        self
    }
}
