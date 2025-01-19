//! Provider of [`KvIConv`].

use crate::prelude::*;
use crate::util::msg;

/// An iterator that converts dyanmic borrowing key-value iterator items.
///
/// This struct is created by [`RefKvIterator::iconv`].
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct KvIConv<I, F> {
    /// Base iterator.
    iter: I,
    /// Closure for each item mapping.
    f: F,
}

impl<I, F> KvIConv<I, F> {
    /// Creates a new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I, F> Iterator for KvIConv<I, F>
where
    I: RefKvIterator,
    F: FnMut(&I::K, &I::V) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        Some((self.f)(item.0, item.1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, F> ExactSizeIterator for KvIConv<I, F>
where
    Self: Iterator,
    I: ExactSizeIterator,
{
    // nop.
}
