use crate::RefIteratorBase;

/// Provider of [`RefKvIterator`].

/// Dynamic borrowing key-value iterator.
pub trait RefKvIterator: RefIteratorBase {
    /// The key type of the elements being iterated over.
    type K: ?Sized;

    /// The value type of the elements being iterated over.
    type V: ?Sized;

    /// Advances the iterator and returns the next key-value.
    ///
    /// # Examples
    /// ```
    /// # use std::cell::RefCell;
    /// # use std::collections::BTreeMap;
    /// # use ref_iter::prelude::*;
    /// #
    /// let samples = BTreeMap::from([(1, 1), (2, 2)]);
    /// let src = RefCell::new(samples.clone());
    /// let mut iter = RefIter::new(src.borrow(), |x| x.iter());
    /// assert_eq!(iter.next(), Some((&1, &1)));
    /// assert_eq!(iter.next(), Some((&2, &2)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<(&Self::K, &Self::V)>;
}