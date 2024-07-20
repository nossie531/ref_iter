//! Provider of [`RefCloned`].

use crate::util::msg;
use crate::*;
use core::ops::Deref;

/// An iterator that clone dynamic borrowing elements.
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

#[gat]
impl<I, T> Iterator for RefCloned<I>
where
    I: RefIterator,
    for<'a> Item<'a, I>: Deref<Target = T>,
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| x.clone())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
