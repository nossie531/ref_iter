use util::msg;
use crate::*;

/// Adapter from [`Iterator`] to [`RefIterator`].
#[must_use = msg::iter_must_use!()]
#[derive(Clone, Debug)]
pub struct AsRefIter<I> {
    iter: I,
}

impl<I> AsRefIter<I> {
    /// Create new value.
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> From<I> for AsRefIter<I> {
    fn from(value: I) -> Self {
        Self::new(value)
    }
}

#[gat]
impl<I> RefIterator for AsRefIter<I>
where
    I: Iterator,
{
    type Item<'x> = I::Item;

    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
