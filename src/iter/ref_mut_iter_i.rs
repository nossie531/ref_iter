//! Provider of [`RefMutIterI`].

use crate::util::{lifetime, msg};
use crate::*;
use core::any::Any;
use core::cell::RefMut;

/// Static typing iterator wrapper for [`RefMut`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::{AsRefIter, RefIterator};
/// # use ref_iter::iter::{RefIterI, RefMutIterI};
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let mut iter = RefMutIterI::new(cell.borrow_mut(), |x| x.iter_mut());
/// while let Some(item) = iter.next() {
///     *item += 1;
/// }
/// drop(iter);
///
/// let iter = RefIterI::new(cell.borrow(), |x| x.iter());
/// let iter = iter.map(|x| *x);
/// assert!(iter.eq::<AsRefIter<_>>(samples.iter().map(|x| x + 1).into()));
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
        unsafe {
            let cell_val = lifetime::reset_mut(&mut *src);
            let cell_iter = f(cell_val);
            Self {
                _src: src,
                iter: cell_iter,
            }
        }
    }
}

#[gat]
impl<'a, I, T: 'a> RefIterator for RefMutIterI<'a, I>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    type Item<'s> = &'s mut T where Self: 's;

    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
