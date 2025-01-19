//! Provider of [`KvIConvMut`].

use crate::prelude::*;
use crate::util::msg;

/// An iterator that converts mutable dyanmic borrowing key-value iterator items.
///
/// This struct is created by [`RefMutKvIterator::iconv_mut`].
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct KvIConvMut<I, F> {
    /// Base iterator.
    iter: I,
    /// Closure for each item mapping.
    f: F,
}

impl<I, F> KvIConvMut<I, F> {
    /// Creates a new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I, F> Iterator for KvIConvMut<I, F>
where
    I: RefMutKvIterator,
    F: FnMut(&I::K, &mut I::V) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next_mut()?;
        Some((self.f)(item.0, item.1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, F> ExactSizeIterator for KvIConvMut<I, F>
where
    Self: Iterator,
    I: ExactSizeIterator,
{
    // nop.
}
