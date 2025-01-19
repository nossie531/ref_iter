//! Provider of [`IMap`].

use crate::prelude::*;
use crate::util::msg;

/// An iterator that maps dyanmic borrowing iterator items.
///
/// This struct is created by [`RefIterator::imap`].
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct IMap<I, F> {
    /// Base iterator.
    iter: I,
    /// Closure for each item mapping.
    f: F,
}

impl<I, F> IMap<I, F> {
    /// Creates a new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I, F> Iterator for IMap<I, F>
where
    I: RefIterator,
    F: FnMut(&I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.f)(self.iter.next()?))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, F> ExactSizeIterator for IMap<I, F>
where
    Self: Iterator,
    I: ExactSizeIterator,
{
    // nop.
}
