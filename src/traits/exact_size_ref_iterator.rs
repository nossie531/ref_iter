//! Provider of [`ExactSizeRefIterator`].

use crate::prelude::*;

/// Dynamic borrowing iterator that knows its exact length.
pub trait ExactSizeRefIterator: RefIterator {
    /// Returns the exact remaining length of the iterator.
    #[inline]
    fn len(&self) -> usize {
        let (lower, upper) = self.size_hint();
        assert_eq!(upper, Some(lower));
        lower
    }

    /// Returns `true` if the iterator is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
