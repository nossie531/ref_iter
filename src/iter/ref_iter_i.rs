//! Provider of [`RefIterI`].

use crate::*;
use crate::util::{lifetime, msg};
use core::any::Any;
use core::cell::Ref;

/// Static typing iterator wrapper for [`Ref`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use core::slice::Iter;
/// # use ref_iter::iter::RefIterI;
/// # use ref_iter::RefIterator;
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples.clone());
/// let iter = RefIterI::new(src.borrow(), |x| x.iter());
/// assert!(iter.eq::<Iter<_>>(samples.iter()));
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

#[gat]
impl<'a, I, T> RefIterator for RefIterI<'a, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    type Item<'x> = &'x T where Self: 'x;

    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
