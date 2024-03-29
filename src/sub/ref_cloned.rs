//! Provider of [`RefCloned`].

use crate::util::msg;
use crate::{RefItem, RefIterator};

/// An iterator that clone dynamic borrowing elements.
///
/// This struct is created by the [`RefIterator::ref_cloned`].
#[must_use = msg::iter_must_use!()]
#[derive(Clone)]
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

impl<'a, I, T> Iterator for RefCloned<I>
where
    I: RefIterator<'a, Item = &'a RefItem<T>>,
    T: 'a + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|x| x.get(self.iter.ref_token()))
            .cloned()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
