//! Provider of [`FnFlatMap`].

use crate::prelude::*;

/// Iterator output from map function and its input.
pub type FnFlatMapIter<F, I> = <<F as FnFlatMap<I>>::Out as IntoRefIterator>::IntoRefIter;

/// Item output from map function and its input.
pub type FnFlatMapItem<F, I> = <<F as FnFlatMap<I>>::Out as IntoRefIterator>::Item;

/// Closure for [`RefIterator::flat_map`] operation.
pub trait FnFlatMap<I>: FnMut(I) -> Self::Out {
    /// Output type.
    type Out: IntoRefIterator;
}

impl<F, I, O> FnFlatMap<I> for F
where
    F: FnMut(I) -> O,
    O: IntoRefIterator,
{
    type Out = O;
}
