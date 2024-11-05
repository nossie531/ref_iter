//! Provider of [`IntoRefMutKvIterator`].

use crate::prelude::*;

/// Conversion into an [`RefMutKvIterator`].
pub trait IntoRefMutKvIterator {
    /// The key type of the elements being iterated over.
    type K: ?Sized;

    /// The value type of the elements being iterated over.
    type V: ?Sized;

    /// Which kind of iterator are we turning this into?
    type IntoRefMutKvIter: RefMutKvIterator<K = Self::K, V = Self::V>;

    /// Creates an iterator from a value.
    fn into_ref_mut_kv_iter(self) -> Self::IntoRefMutKvIter;
}

impl<T: RefMutKvIterator> IntoRefMutKvIterator for T {
    type K = T::K;
    type V = T::V;
    type IntoRefMutKvIter = Self;

    fn into_ref_mut_kv_iter(self) -> Self::IntoRefMutKvIter {
        self
    }
}
