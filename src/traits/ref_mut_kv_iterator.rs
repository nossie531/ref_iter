//! Provider of [`RefMutKvIterator`].

use crate::prelude::*;
use crate::RefMutKvMap;

/// Mutable dynamic borrowing key-value iterator.
pub trait RefMutKvIterator: RefKvIterator {
    /// Advances the iterator and returns the next mutable key-value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
    /// # use std::collections::BTreeMap;
    /// #
    /// let mut samples = BTreeMap::from([(1, 1), (2, 2)]);
    /// let src = RefCell::new(samples.clone());
    /// let mut iter = RefMutIter::new(src.borrow_mut(), |x| x.iter_mut());
    /// assert_eq!(iter.next_mut(), Some((&1, &mut 1)));
    /// assert_eq!(iter.next_mut(), Some((&2, &mut 2)));
    /// assert_eq!(iter.next_mut(), None);
    /// ```
    fn next_mut(&mut self) -> Option<(&Self::K, &mut Self::V)>;

    /// Creates an iterator that maps dynamic borrowing elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
    /// # use std::collections::HashMap;
    ///
    /// let samples = HashMap::from([(1, 0), (2, 0), (3, 0)]);
    /// let src = RefCell::new(samples.clone());
    /// let iter = RefMutIter::new(src.borrow_mut(), |x| x.iter_mut());
    /// let iter = iter.map_mut(|k, v| { *v += 1; *k });
    ///
    /// assert!(iter.eq(samples.iter().map(|x| *x.0)));
    /// let iter = RefIter::new(src.borrow(), |x| x.iter());
    /// let iter = iter.map(|_, v| *v);
    /// assert!(iter.eq(samples.iter().map(|x| x.1 + 1)));
    /// ```
    fn map_mut<B, F>(self, f: F) -> RefMutKvMap<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::K, &mut Self::V) -> B,
    {
        RefMutKvMap::new(self, f)
    }
}
