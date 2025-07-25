//! Provider of [`RefKvIterator`].

use crate::KvIMap;
use crate::prelude::*;
use core::ops::DerefMut;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Immutable dynamic borrowing key-value iterator.
pub trait RefKvIterator: RefIteratorBase {
    /// The key type of the elements being iterated over.
    type K: ?Sized;

    /// The value type of the elements being iterated over.
    type V: ?Sized;

    /// Advances the iterator and returns the next key-value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
    /// # use std::collections::BTreeMap;
    /// #
    /// let samples = BTreeMap::from([(1, 1), (2, 2)]);
    /// let src = RefCell::new(samples.clone());
    /// let mut iter = RefIter::new(src.borrow(), |x| x.iter());
    /// assert_eq!(iter.next(), Some((&1, &1)));
    /// assert_eq!(iter.next(), Some((&2, &2)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<(&Self::K, &Self::V)>;

    /// Creates an iterator that maps dynamic borrowing elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
    /// # use std::collections::HashMap;
    /// #
    /// let samples = HashMap::from([(1, 10), (2, 20)]);
    /// let src = RefCell::new(samples.clone());
    /// let iter = RefIter::new(src.borrow(), |x| x.iter());
    /// let iter = iter.imap(|k, v| k + v);
    /// assert!(iter.eq(samples.iter().map(|x| x.0 + x.1)));
    /// ```
    fn imap<B, F>(self, f: F) -> KvIMap<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::K, &Self::V) -> B,
    {
        KvIMap::new(self, f)
    }
}

impl<I> RefKvIterator for Box<I>
where
    I: RefKvIterator + ?Sized,
{
    type K = I::K;
    type V = I::V;

    fn next(&mut self) -> Option<(&Self::K, &Self::V)> {
        self.deref_mut().next()
    }
}
