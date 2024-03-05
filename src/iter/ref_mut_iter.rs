//! Provider of [`RefMutIter`].

use crate::util::{lifetime, msg};
use crate::RefIterator;
use alloc::boxed::Box;
use core::any::Any;
use core::cell::RefMut;

/// Dynamic typing iterator wrapper for [`RefMut`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::iter::RefMutIter;
/// # use ref_iter::RefIterator;
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples.clone());
/// let mut iter = RefMutIter::new(src.borrow_mut(), |x| x.iter_mut());
/// iter.ref_act(|x, t| {
///     for item in x {
///         *item.get_mut(t) += 1;
///     }
/// });
/// drop(iter);
///
/// let iter = RefMutIter::new(src.borrow_mut(), |x| x.iter_mut());
/// let iter = iter.ref_map(|x, t| *x.get(t));
/// assert!(iter.eq(samples.iter().map(|x| x + 1)));
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
        let src_ref = unsafe { lifetime::reset_mut(&mut src) };
        Self {
            _src: src,
            iter: Box::new(f(src_ref)),
        }
    }
}

impl<'a, T> RefIterator for RefMutIter<'a, T> {
    type Item<'x> = &'a mut T where Self: 'x;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
