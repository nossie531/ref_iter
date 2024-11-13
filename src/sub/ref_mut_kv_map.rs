//! Provider of [`RefMutKvMap`].

use crate::prelude::*;
use crate::util::msg;

/// Item mapper for mutable dyanmic borrowing key-value iterator.
///
/// This struct is created by [`RefMutKvIterator::map_mut`].
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct RefMutKvMap<I, F> {
    /// Base iterator.
    iter: I,
    /// Closure for each item mapping.
    f: F,
}

impl<I, F> RefMutKvMap<I, F> {
    /// Create new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I, F> Iterator for RefMutKvMap<I, F>
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
