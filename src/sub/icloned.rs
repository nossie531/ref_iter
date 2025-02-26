//! Provider of [`ICloned`].

use crate::prelude::*;
use crate::util::msg;

/// An iterator that clones elements of dynamic borrowing iterator.
///
/// This struct is created by [`RefIterator::icloned`].
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct ICloned<I> {
    /// Base iterator.
    iter: I,
}

impl<I> ICloned<I> {
    /// Creates a new value.
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for ICloned<I>
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

impl<I> ExactSizeIterator for ICloned<I>
where
    Self: Iterator,
    I: ExactSizeIterator,
{
    // nop.
}
