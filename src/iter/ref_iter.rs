//! Provider of [`RefIter`].

use crate::util::{lt, msg};
use crate::{RefItem, RefIterator};
use alloc::boxed::Box;
use core::any::Any;
use core::cell::Ref;

/// Dynamic typing iterator wrapper for [`Ref`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::iter::RefIter;
/// # use ref_iter::RefIterator;    
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples.clone());
/// let iter = RefIter::new(src.borrow(), |x| x.iter());
/// let iter = iter.ref_map(|x, t| *x.get(t));
/// assert!(iter.eq(samples));
/// ```
#[must_use = msg::iter_must_use!()]
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
        let src_ref = unsafe { lt::reset_ref_lifetime(&src) };
        Self {
            _src: src,
            iter: Box::new(f(src_ref)),
        }
    }
}

impl<'a, T> Iterator for RefIter<'a, T> {
    type Item = &'a RefItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| RefItem::use_ref(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> RefIterator for RefIter<'a, T> {
    // NOP.
}
