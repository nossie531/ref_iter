//! Provider of [`RefIterator`].

use crate::closure::FnMap;
use crate::prelude::*;
use crate::sub::{RefCloned, RefMap};
use crate::util::msg;

/// Immutable dynamic borrowing iterator.
#[must_use = msg::iter_must_use!()]
pub trait RefIterator: RefIteratorBase {
    /// The type of the elements being iterated over.
    type Item: ?Sized;

    /// Advances the iterator and returns the next value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::RefCell;
    /// # use ref_iter::prelude::*;
    /// #
    /// let samples = vec![1, 2];
    /// let src = RefCell::new(samples.clone());
    /// let mut iter = RefIter::new(src.borrow(), |x| x.iter());
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<&Self::Item>;

    /// Creates an iterator that clone dynamic borrowing elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::RefCell;
    /// # use ref_iter::prelude::*;
    /// #
    /// let samples = vec![1, 2, 3];
    /// let src = RefCell::new(samples.clone());
    /// let iter = RefIter::new(src.borrow(), |x| x.iter());
    /// assert!(iter.cloned().eq(samples.iter().cloned()));
    /// ```
    fn cloned(self) -> RefCloned<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        RefCloned::new(self)
    }

    /// Creates an iterator that maps dynamic borrowing elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::RefCell;
    /// # use ref_iter::prelude::*;
    /// #
    /// let samples = vec![1, 2, 3];
    /// let src = RefCell::new(samples.clone());
    /// let iter = RefIter::new(src.borrow(), |x| x.iter());
    /// let iter = iter.map(|x: &_| x + 1).into_iter();
    /// assert!(iter.eq(samples.iter().map(|x| x + 1)));
    /// ```
    fn map<F>(self, f: F) -> RefMap<Self, F>
    where
        Self: Sized,
        F: for<'a> FnMap<&'a Self::Item>,
    {
        RefMap::new(self, f)
    }

    /// Determines if the elements of this is equal to those of another.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::RefCell;
    /// # use ref_iter::prelude::*;
    /// #
    /// let samples = vec![1, 2, 3];
    /// let src = RefCell::new(samples.clone());
    /// let iter1 = RefIter::new(src.borrow(), |x| x.iter());
    /// let iter2 = RefIter::new(src.borrow(), |x| x.iter());
    /// assert!(iter1.eq(iter2));
    /// ```
    fn eq<I>(mut self, other: I) -> bool
    where
        Self: Sized,
        Self::Item: PartialEq<<I::IntoRefIter as RefIterator>::Item>,
        I: IntoRefIterator,
    {
        let mut iter = other.into_ref_iter();
        loop {
            let x = self.next();
            let y = iter.next();
            match (x, y) {
                (None, None) => return true,
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
                (Some(x), Some(y)) => {
                    if x != y {
                        return false;
                    }
                }
            }
        }
    }
}
