//! Provider of [`RefMutIter`].

use crate::prelude::*;
use crate::util::{lifetime, msg};
use core::any::Any;
use core::cell::RefMut;

/// Iterator from [`RefMut`].
///
/// # Examples
///
/// ```
/// # use std::cell::RefCell;
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
#[derive(Debug)]
#[must_use = msg::iter_must_use!()]
pub struct RefMutIter<'a, I> {
    /// Dynamic borrowing source.
    _src: RefMut<'a, dyn Any>,
    /// Iterator generated from source.
    iter: I,
}

impl<'a, I> RefMutIter<'a, I> {
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

impl<I> RefIteratorBase for RefMutIter<'_, I>
where
    I: Iterator,
{
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, T: 'a> RefIterator for RefMutIter<'a, I>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next().map(|x| &*x)
    }
}

impl<'a, I, K, V> RefKvIterator for RefMutIter<'a, I>
where
    I: Iterator<Item = (&'a K, &'a mut V)>,
    K: 'a,
    V: 'a,
{
    type K = K;
    type V = V;

    fn next(&mut self) -> Option<(&Self::K, &Self::V)> {
        self.iter.next().map(|(k, v)| (k, &*v))
    }
}

impl<'a, I, T: 'a> RefMutIterator for RefMutIter<'a, I>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    fn next_mut(&mut self) -> Option<&mut Self::Item> {
        self.iter.next()
    }
}

impl<'a, I, K, V> RefMutKvIterator for RefMutIter<'a, I>
where
    I: Iterator<Item = (&'a K, &'a mut V)>,
    K: 'a,
    V: 'a,
{
    fn next_mut(&mut self) -> Option<(&Self::K, &mut Self::V)> {
        self.iter.next()
    }
}
