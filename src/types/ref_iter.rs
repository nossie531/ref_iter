//! Provider of [`RefIter`].

use crate::util::{lifetime, msg};
use crate::*;
use alloc::boxed::Box;
use core::any::Any;
use core::cell::Ref;

/// Dynamic typing iterator wrapper for [`Ref`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::*;
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let iter = RefIter::new(cell.borrow(), |x| x.iter());
/// assert!(iter.cloned().eq(samples.iter().cloned()));
/// ```
#[must_use = msg::iter_must_use!()]
#[cfg(feature = "alloc")]
pub struct RefIter<'a, T> {
    /// Dynamic borrowing source.
    _src: Ref<'a, dyn Any>,
    /// Iterator generated from source.
    iter: Box<dyn Iterator<Item = &'a T> + 'a>,
}

impl<'a, T> RefIter<'a, T> {
    /// Create a new value.
    pub fn new<S, I, F>(src: Ref<'a, S>, f: F) -> Self
    where
        S: Any,
        I: Iterator<Item = &'a T> + 'a,
        F: Fn(&'a S) -> I,
    {
        unsafe {
            let cell_val = lifetime::reset_ref(&*src);
            let cell_iter = Box::new(f(cell_val));
            Self {
                _src: src,
                iter: cell_iter,
            }
        }
    }
}

impl<'a, T> RefIterator for RefIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
