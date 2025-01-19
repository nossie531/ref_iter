//! Provider of [`RefIterator`].

use crate::closures::FnIFlatMap;
use crate::prelude::*;
use crate::sub::{ICloned, IFlatMap, IMap};
use crate::util::msg;
use core::ops::DerefMut;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

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
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
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
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
    /// #
    /// let samples = vec![1, 2, 3];
    /// let src = RefCell::new(samples.clone());
    /// let iter = RefIter::new(src.borrow(), |x| x.iter());
    /// assert!(iter.icloned().eq(samples.iter().cloned()));
    /// ```
    fn icloned(self) -> ICloned<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        ICloned::new(self)
    }

    /// Creates an iterator that flattens mapped each dynamic borrowing iterators.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
    /// #
    /// let samples = vec![vec![1, 2], vec![3, 4, 5]];
    /// let expecteds = vec![1, 2, 3, 4, 5];
    /// let src = RefCell::new(samples.clone());
    /// let iter = RefIter::new(src.borrow(), |x| x.iter());
    /// let iter = iter.iflat_map(vec_iter);
    /// assert!(iter.eq(expecteds.iter().cloned()));
    ///
    /// fn vec_iter(x: &Vec<i32>) -> impl Iterator<Item = i32> + '_ {
    ///     x.iter().cloned()
    /// }
    /// ```
    fn iflat_map<'call, F>(self, f: F) -> IFlatMap<'call, Self, F>
    where
        Self: Sized,
        F: for<'a> FnIFlatMap<&'a Self::Item>,
    {
        IFlatMap::new(self, f)
    }

    /// Creates an iterator that maps dynamic borrowing elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
    /// #
    /// let samples = vec![1, 2, 3];
    /// let src = RefCell::new(samples.clone());
    /// let iter = RefIter::new(src.borrow(), |x| x.iter());
    /// let iter = iter.imap(|x: &_| x + 1);
    /// assert!(iter.eq(samples.iter().map(|x| x + 1)));
    /// ```
    fn imap<B, F>(self, f: F) -> IMap<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> B,
    {
        IMap::new(self, f)
    }

    /// Determines if the elements of this is equal to those of another.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ref_iter::prelude::*;
    /// # use std::cell::RefCell;
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
        Self::Item: PartialEq<I::Item>,
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

impl<I> RefIterator for Box<I>
where
    I: RefIterator + ?Sized,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<&Self::Item> {
        self.deref_mut().next()
    }
}
