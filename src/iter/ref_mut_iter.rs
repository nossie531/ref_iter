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
/// # use ref_iter::{AsRefIter, RefIterator};
/// # use ref_iter::iter::{RefIter, RefMutIter};
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let mut iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
/// while let Some(item) = iter.next() {
///     *item += 1;
/// }
/// drop(iter);
///
/// let iter = RefIter::new(cell.borrow(), |x| x.iter());
/// let iter = iter.map(|x| *x);
/// assert!(iter.eq::<AsRefIter<_>>(samples.iter().map(|x| x + 1).into()));
/// ```
#[must_use = msg::iter_must_use!()]
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
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

#[gat]
impl<'a, T> RefIterator for RefMutIter<'a, T> {
    type Item<'x> = &'a mut T where Self: 'x;

    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
