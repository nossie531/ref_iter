//! Provider of [`IntoRefIter`].

use crate::prelude::*;
use crate::util::msg;

/// Adapter that turns [`Iterator`] into [`RefIterator`].
///
/// # Examples
///
/// ```
/// # use ref_iter::prelude::*;
/// # use std::cell::RefCell;
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples.clone());
/// let iter1 = RefIter::new(src.borrow(), |x| x.iter());
/// let iter2 = IntoRefIter::new(samples.iter());
/// assert!(iter1.eq(iter2));
/// ```
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct IntoRefIter<'a, I, T>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    /// Base iterator.
    iter: I,
}

impl<'a, I, T> IntoRefIter<'a, I, T>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    /// Creates a new value.
    pub fn new<A: IntoIterator<IntoIter = I>>(iter: A) -> Self {
        Self {
            iter: iter.into_iter(),
        }
    }
}

impl<'a, I, T> RefIteratorBase for IntoRefIter<'a, I, T>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, T> RefIterator for IntoRefIter<'a, I, T>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next()
    }
}
