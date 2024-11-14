//! Provider of [`RefMutIterator`].

use crate::prelude::*;
use crate::sub::RefMutMap;
use crate::util::msg;
use core::ops::DerefMut;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Mutable dynamic borrowing iterator.
#[must_use = msg::iter_must_use!()]
pub trait RefMutIterator: RefIterator {
    /// Advances the iterator and returns the next mutable value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
    /// #
    /// let samples = vec![1, 2];
    /// let src = RefCell::new(samples.clone());
    /// let mut iter = RefMutIter::new(src.borrow_mut(), |x| x.iter_mut());
    /// assert_eq!(iter.next_mut(), Some(&mut 1));
    /// assert_eq!(iter.next_mut(), Some(&mut 2));
    /// assert_eq!(iter.next_mut(), None);
    /// ```
    fn next_mut(&mut self) -> Option<&mut Self::Item>;

    /// Creates an iterator that maps mutable dynamic borrowing elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
    /// #
    /// let samples = vec![1, 2, 3];
    /// let src = RefCell::new(samples.clone());
    /// let iter = RefMutIter::new(src.borrow_mut(), |x| x.iter_mut());
    /// let iter = iter.map_mut(|x| { *x += 1; *x }).into_iter();
    ///
    /// assert!(iter.eq(samples.iter().map(|x| x + 1)));
    /// let iter = RefIter::new(src.borrow(), |x| x.iter()).cloned();
    /// assert!(iter.eq(samples.iter().map(|x| x + 1)));
    /// ```
    fn map_mut<B, F>(self, f: F) -> RefMutMap<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self::Item) -> B,
    {
        RefMutMap::new(self, f)
    }
}

impl<I> RefMutIterator for Box<I>
where
    I: RefMutIterator + ?Sized,
{
    fn next_mut(&mut self) -> Option<&mut Self::Item> {
        self.deref_mut().next_mut()
    }
}
