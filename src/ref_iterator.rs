//! Provider of [`RefIterator`].

use crate::ref_token::RefToken;
use crate::sub::{RefCloned, RefMap};
use crate::util::msg;
use crate::RefItem;

/// A trait for dynamic borrowing iterators.
#[must_use = msg::iter_must_use!()]
pub trait RefIterator<'a>: Iterator {
    /// Get reference token.
    fn ref_token<'s>(&'s self) -> &'a RefToken
    where
        'a: 's;

    /// Creates an iterator that clone dynamic borrowing elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::cell::RefCell;
    /// # use ref_iter::iter::RefIter;
    /// # use ref_iter::{RefIterator, RefToken};
    /// #
    /// let samples = vec![1, 2, 3];
    /// let src = RefCell::new(samples.clone());
    /// let token = RefToken::new();
    /// let mut iter = RefIter::new(src.borrow(), &token, |x| x.iter());
    /// let mut map = iter.ref_cloned();
    /// assert!(map.eq(samples.iter().cloned()));
    /// ```
    fn ref_cloned<T>(self) -> RefCloned<Self>
    where
        Self: Sized + Iterator<Item = &'a RefItem<T>>,
        T: 'a + Clone,
    {
        RefCloned::new(self)
    }

    /// Creates an iterator that maps dynamic borrowing elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::cell::RefCell;
    /// # use ref_iter::iter::RefIter;
    /// # use ref_iter::{RefIterator, RefToken};
    /// #
    /// let samples = vec![1, 2, 3];
    /// let src = RefCell::new(samples.clone());
    /// let token = RefToken::new();
    /// let mut iter = RefIter::new(src.borrow(), &token, |x| x.iter());
    /// let mut map = iter.ref_map(|x, t| x.get(&token));
    /// assert!(map.eq(samples.iter()));
    /// ```
    fn ref_map<T, F>(self, f: F) -> RefMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item, &RefToken) -> T,
    {
        RefMap::new(self, f)
    }
}
