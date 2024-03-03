//! Provider of [`RefIterI`].

use crate::util::{lt, msg};
use crate::{RefItem, RefIterator};
use core::any::Any;
use core::cell::Ref;

/// Static typing iterator wrapper for [`Ref`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::iter::RefIterI;
/// # use ref_iter::RefIterator;
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples.clone());
/// let iter = RefIterI::new(src.borrow(), |x| x.iter());
/// let iter = iter.ref_map(|x, t| *x.get(t));
/// assert!(iter.eq(samples));
/// ```
#[must_use = msg::iter_must_use!()]
pub struct RefIterI<'a, I> {
    /// Dynamic borrowing source.
    _src: Ref<'a, dyn Any>,
    /// Iterator generated from source.
    iter: I,
}

impl<'a, I> RefIterI<'a, I> {
    /// Create a new value.
    pub fn new<S, F>(src: Ref<'a, S>, f: F) -> Self
    where
        S: Any,
        F: Fn(&'a S) -> I,
    {
        let src_ref = unsafe { lt::reset_ref_lifetime(&src) };
        Self {
            _src: src,
            iter: f(src_ref),
        }
    }
}

impl<'a, I, T> Iterator for RefIterI<'a, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    type Item = &'a RefItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| RefItem::use_ref(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, T> RefIterator for RefIterI<'a, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    // NOP.
}
