//! Provider of [`RefMutIterator`].

use crate::closure::FnMap;
use crate::prelude::*;
use crate::sub::RefMutMap;
use crate::util::msg;

/// Mutable dynamic borrowing iterator.
#[must_use = msg::iter_must_use!()]
pub trait RefMutIterator: RefIterator {
    /// Advances the iterator and returns the next mutable value.
    fn next_mut(&mut self) -> Option<&mut Self::Item>;

    /// Creates an iterator that maps mutable dynamic borrowing elements.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::cell::RefCell;
    /// # use ref_iter::prelude::*;
    /// #
    /// let samples = vec![(1, 1), (2, 2), (3, 3)];
    /// let sec_mut = (|x: &mut(i32, i32)| &mut x.1) as fn(&mut _) -> &mut _;
    /// let cell = RefCell::new(samples.clone());
    /// let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
    /// let mut iter = iter.map_mut(sec_mut);
    /// while let Some(item) = iter.next_mut() {
    ///     *item = 0;
    /// }
    /// drop(iter);
    ///
    /// let results = cell.into_inner();
    /// let expecteds = vec![(1, 0), (2, 0), (3, 0)];
    /// assert_eq!(results, expecteds);
    /// ```
    fn map_mut<F>(self, f: F) -> RefMutMap<Self, F>
    where
        Self: Sized,
        F: for<'a> FnMap<&'a mut Self::Item>,
    {
        RefMutMap::new(self, f)
    }
}
