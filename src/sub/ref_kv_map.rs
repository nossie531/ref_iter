//! Provider of [`RefKvMap`].

use crate::prelude::*;
use crate::util::msg;

/// Item mapper for immutable dyanmic borrowing key-value iterator.
///
/// This struct is created by [`RefKvIterator::map`].
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct RefKvMap<I, F> {
    /// Base iterator.
    iter: I,
    /// Closure for each item mapping.
    f: F,
}

impl<I, F> RefKvMap<I, F> {
    /// Create new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I, F> Iterator for RefKvMap<I, F>
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
