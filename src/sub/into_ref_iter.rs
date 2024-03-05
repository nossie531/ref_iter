//! Provider of [`IntoRefIter`].

use crate::RefIterator;

/// Adapter for converting [`Iterator`] to [`RefIterator`].
///
/// This struct is created by the [`into_ref_iter`] for [`IntoIterator`].
/// 
/// [`into_ref_iter`]: IntoRefIterator::into_ref_iter
#[derive(Clone, Debug)]
pub struct IntoRefIter<I: Iterator> {
    base: I,
}

impl<I: Iterator> IntoRefIter<I> {
    pub(crate) fn new(base: I) -> Self {
        Self { base }
    }
}

impl<I: Iterator> RefIterator for IntoRefIter<I> {
    type Item<'a> = I::Item where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.base.next()
    }
}
