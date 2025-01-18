//! Provider of [`RefMutMap`].

use crate::prelude::*;
use crate::util::msg;

/// An iterator that maps mutable dyanmic borrowing iterator items.
///
/// This struct is created by [`RefMutIterator::map_mut`].
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct RefMutMap<I, F> {
    /// Base iterator.
    iter: I,
    /// Closure for each item mapping.
    f: F,
}

impl<I, F> RefMutMap<I, F> {
    /// Creates a new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I, F> Iterator for RefMutMap<I, F>
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

impl<I, F> ExactSizeIterator for RefMutMap<I, F>
where
    Self: Iterator,
    I: ExactSizeIterator,
{
    // nop.
}
