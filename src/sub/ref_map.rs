//! Provider of [`RefMap`].

use crate::util::msg;
use crate::*;

/// Lending iterator that maps dynamic borrowing elements.
///
/// This struct is created by the [`RefIterator::map`].
#[must_use = msg::iter_must_use!()]
#[derive(Clone, Debug)]
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

#[gat]
impl<I, F, T> RefIterator for RefMap<I, F>
where
    I: RefIterator,
    F: for<'any> FnMut(<I as RefIterator>::Item<'any>) -> T,
{
    type Item<'a> = T where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|x| (self.f)(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[gat]
impl<I, F, T> IntoIterator for RefMap<I, F>
where
    I: RefIterator,
    F: for<'any> FnMut(<I as RefIterator>::Item<'any>) -> T,
{
    type Item = T;
    type IntoIter = RefMapIter<I, F>;

    fn into_iter(self) -> Self::IntoIter {
        RefMapIter::new(self.iter, self.f)
    }
}

/// Normal (not lending) iterator that maps dynamic borrowing elements.
///
/// This struct is created by [RefMap::into_iter].
pub struct RefMapIter<I, F> {
    iter: I,
    f: F,
}

impl<I, F> RefMapIter<I, F> {
    /// Create new instance.
    fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F, T> Iterator for RefMapIter<I, F>
where
    I: RefIterator,
    F: FnMut(Gat!(<I as RefIterator>::Item<'_>)) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.f)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
