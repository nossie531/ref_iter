//! Provider of [`RefMutIterI`].

use crate::prelude::*;
use crate::util::{lifetime, msg};
use core::any::Any;
use core::cell::RefMut;

/// Static typing iterator wrapper for [`RefMut`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let mut iter = RefMutIterI::new(cell.borrow_mut(), |x| x.iter_mut());
/// while let Some(item) = iter.next_mut() {
///     *item += 1;
/// }
/// drop(iter);
///
/// let results = cell.into_inner();
/// let expecteds = vec![2, 3, 4];
/// assert_eq!(results, expecteds);
/// ```
#[derive(Debug)]
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

impl<'a, I, T: 'a> RefIterator for RefMutIterI<'a, I>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next().map(|x| &*x)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, T: 'a> RefMutIterator for RefMutIterI<'a, I>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    fn next_mut(&mut self) -> Option<&mut Self::Item> {
        self.iter.next()
    }
}
