//! Provider of [`RefMutIter`].

use crate::util::{lifetime, msg};
use crate::*;
use alloc::boxed::Box;
use core::any::Any;
use core::cell::RefMut;

/// Dynamic typing iterator wrapper for [`RefMut`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let mut iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
/// while let Some(item) = iter.next_mut() {
///     *item += 1;
/// }
/// drop(iter);
///
/// let results = cell.into_inner();
/// let expecteds = vec![2, 3, 4];
/// assert_eq!(results, expecteds);
/// ```
#[must_use = msg::iter_must_use!()]
pub struct RefMutIter<'a, T> {
    /// Dynamic borrowing source.
    _src: RefMut<'a, dyn Any>,
    /// Iterator generated from source.
    iter: Box<dyn Iterator<Item = &'a mut T> + 'a>,
}

impl<'a, T> RefMutIter<'a, T> {
    /// Create a new value.
    pub fn new<S, I, F>(mut src: RefMut<'a, S>, f: F) -> Self
    where
        S: Any,
        I: Iterator<Item = &'a mut T> + 'a,
        F: Fn(&'a mut S) -> I,
    {
        unsafe {
            let cell_val = lifetime::reset_mut(&mut *src);
            let cell_iter = Box::new(f(cell_val));
            Self {
                _src: src,
                iter: cell_iter,
            }
        }
    }
}

impl<'a, T> RefIterator for RefMutIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next().map(|x| &*x)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> RefMutIterator for RefMutIter<'a, T> {
    fn next_mut(&mut self) -> Option<&mut Self::Item> {
        self.iter.next()
    }
}
