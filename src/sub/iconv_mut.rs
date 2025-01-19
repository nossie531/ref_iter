//! Provider of [`IConvMut`].

use crate::prelude::*;
use crate::util::msg;

/// An iterator that converts mutable dyanmic borrowing iterator items.
///
/// This struct is created by [`RefMutIterator::iconv_mut`].
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct IConvMut<I, F> {
    /// Base iterator.
    iter: I,
    /// Closure for each item mapping.
    f: F,
}

impl<I, F> IConvMut<I, F> {
    /// Creates a new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I, F> Iterator for IConvMut<I, F>
where
    I: RefMutIterator,
    F: FnMut(&mut I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.f)(self.iter.next_mut()?))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, F> ExactSizeIterator for IConvMut<I, F>
where
    Self: Iterator,
    I: ExactSizeIterator,
{
    // nop.
}
