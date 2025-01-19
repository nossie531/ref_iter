//! Provider of [`RefMutKvIterator`].

use crate::prelude::*;
use core::ops::DerefMut;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

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
}

impl<I> RefMutKvIterator for Box<I>
where
    I: RefMutKvIterator + ?Sized,
{
    fn next_mut(&mut self) -> Option<(&Self::K, &mut Self::V)> {
        self.deref_mut().next_mut()
    }
}
