//! Provider of [`RefIter`].

use crate::prelude::*;
use crate::util::{lifetime, msg};
use core::any::Any;
use core::cell::Ref;

/// Immutable dynamic borrowing iterator for [`Ref`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples.clone());
/// let iter = RefIter::new(src.borrow(), |x| x.iter());
/// assert!(iter.cloned().eq(samples.iter().cloned()));
/// ```
#[must_use = msg::iter_must_use!()]
#[derive(Debug)]
pub struct RefIter<'a, I> {
    /// Dynamic borrowing source.
    _src: Ref<'a, dyn Any>,
    /// Iterator generated from source.
    iter: I,
}

impl<'a, I> RefIter<'a, I> {
    /// Create a new value.
    pub fn new<S, F>(src: Ref<'a, S>, f: F) -> Self
    where
        S: Any,
        F: Fn(&'a S) -> I,
    {
        unsafe {
            let cell_val = lifetime::reset_ref(&*src);
            let cell_iter = f(cell_val);
            Self {
                _src: src,
                iter: cell_iter,
            }
        }
    }
}

impl<I> Clone for RefIter<'_, I>
where
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            _src: Ref::clone(&self._src),
            iter: self.iter.clone(),
        }
    }
}

impl<'a, I, T> RefIterator for RefIter<'a, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
