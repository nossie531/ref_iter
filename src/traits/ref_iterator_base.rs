//! Provider of [`RefIteratorBase`].

use crate::util::msg;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Dynamic borrowing iterator base.
#[must_use = msg::iter_must_use!()]
pub trait RefIteratorBase {
    /// Returns the bounds on the remaining length of the iterator.
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
    /// let hint = iter.size_hint();
    /// drop(iter);
    /// assert_eq!(hint, src.borrow().iter().size_hint());
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

#[cfg(feature = "alloc")]
impl<I> RefIteratorBase for Box<I>
where
    I: RefIteratorBase + ?Sized,
{
    // nop.
}
