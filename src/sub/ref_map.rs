//! Provider of [`RefMap`].

use crate::prelude::*;
use crate::util::msg;

/// Item mapper for immutable dyanmic borrowing iterator.
///
/// This struct is created by [`RefIterator::map`].
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct RefMap<I, F> {
    /// Base iterator.
    iter: I,
    /// Closure for each item mapping.
    f: F,
}

impl<I, F> RefMap<I, F> {
    /// Create new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I, F> Iterator for RefMap<I, F>
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
