//! Provider of [`RefMutMap`].

use crate::prelude::*;
use crate::util::msg;

/// Item mapper for mutable dyanmic borrowing iterator.
///
/// This struct is created by the [`RefMutIterator::map_mut`].
#[must_use = msg::iter_must_use!()]
#[derive(Clone, Debug)]
pub struct RefMutMap<I, F> {
    /// Base iterator.
    iter: I,
    /// Closure for each item mapping.
    f: F,
}

impl<I, F> RefMutMap<I, F> {
    /// Create new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F, T> RefIterator for RefMutMap<I, F>
where
    I: RefMutIterator,
    F: FnMut(&mut I::Item) -> &mut T,
{
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next_mut().map(|x| &*(self.f)(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, F, T> RefMutIterator for RefMutMap<I, F>
where
    I: RefMutIterator,
    F: FnMut(&mut I::Item) -> &mut T,
{
    fn next_mut(&mut self) -> Option<&mut Self::Item> {
        self.iter.next_mut().map(|x| (self.f)(x))
    }
}

impl<I, F, T> IntoIterator for RefMutMap<I, F>
where
    I: RefMutIterator,
    F: FnMut(&mut I::Item) -> T,
{
    type Item = T;
    type IntoIter = RefMutMapIter<I, F>;

    fn into_iter(self) -> Self::IntoIter {
        RefMutMapIter::new(self.iter, self.f)
    }
}

/// Adapter that turns [`RefMutMap`] into [`Iterator`].
///
/// This struct is created by [`RefMutMap::into_iter`].
pub struct RefMutMapIter<I, F> {
    iter: I,
    f: F,
}

impl<I, F> RefMutMapIter<I, F> {
    /// Create new instance.
    fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F, T> Iterator for RefMutMapIter<I, F>
where
    I: RefMutIterator,
    F: FnMut(&mut I::Item) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_mut().map(&mut self.f)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
