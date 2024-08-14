//! Provider of [`RefCloned`].

use crate::prelude::*;
use crate::util::msg;

/// An iterator that clones elements of dynamic borrowing itefator.
///
/// This struct is created by the [`RefIterator::cloned`].
#[must_use = msg::iter_must_use!()]
#[derive(Clone, Debug)]
pub struct RefCloned<I> {
    /// Base iterator.
    iter: I,
}

impl<I> RefCloned<I> {
    /// Create new value.
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for RefCloned<I>
where
    I: RefIterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().cloned()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
