//! Provider of [`RefMutIterI`].

use crate::util::{lt, msg};
use crate::{RefItem, RefIterator};
use core::any::Any;
use core::cell::RefMut;

/// Static typing iterator wrapper for [`RefMut`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::iter::RefMutIterI;
/// # use ref_iter::RefIterator;
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples.clone());
/// let mut iter = RefMutIterI::new(src.borrow_mut(), |x| x.iter_mut());
/// iter.ref_act(|x, t| {
///     for item in x {
///         *item.get_mut(t) += 1;
///     }
/// });
/// drop(iter);
///
/// let iter = RefMutIterI::new(src.borrow_mut(), |x| x.iter_mut());
/// let iter = iter.ref_map(|x, t| *x.get(t));
/// assert!(iter.eq(samples.iter().map(|x| x + 1)));
/// ```
#[must_use = msg::iter_must_use!()]
pub struct RefMutIterI<'a, I> {
    /// Dynamic borrowing source.
    _src: RefMut<'a, dyn Any>,
    /// Iterator generated from source.
    iter: I,
}

impl<'a, I> RefMutIterI<'a, I> {
    /// Create a new value.
    pub fn new<S, F>(mut src: RefMut<'a, S>, f: F) -> Self
    where
        S: Any,
        F: Fn(&'a mut S) -> I,
    {
        let src_ref = unsafe { lt::reset_mut_lifetime(&mut src) };
        Self {
            _src: src,
            iter: f(src_ref),
        }
    }
}

impl<'a, I, T> Iterator for RefMutIterI<'a, I>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    type Item = &'a mut RefItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| RefItem::use_mut(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, T: 'a> RefIterator for RefMutIterI<'a, I>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    // NOP.
}
