//! Provider of [`IntoRefIter`].

use crate::*;

/// Adapter for converting [`Iterator`] to [`RefIterator`].
///
/// This struct is created by the [`into_ref_iter`] for [`IntoIterator`].
///
/// [`into_ref_iter`]: IntoRefIterator::into_ref_iter
#[derive(Clone, Debug)]
pub struct IntoRefIter<I>
where
    I: Iterator,
{
    base: I,
}

impl<I> IntoRefIter<I>
where
    I: Iterator,
{
    pub(crate) fn new(base: I) -> Self {
        Self { base }
    }
}

#[gat]
impl<I> RefIterator for IntoRefIter<I>
where
    I: Iterator,
{
    type Item<'a> = I::Item where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.base.next()
    }
}
