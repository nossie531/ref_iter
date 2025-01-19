//! Provider of [`FnIFlatMap`].

/// Iterator output from map function and its input.
pub type FnIFlatMapIter<F, I> = <<F as FnIFlatMap<I>>::Out as IntoIterator>::IntoIter;

/// Item output from map function and its input.
pub type FnIFlatMapItem<F, I> = <<F as FnIFlatMap<I>>::Out as IntoIterator>::Item;

/// Closure for [`RefIterator::flat_map`] operation.
pub trait FnIFlatMap<I>: FnMut(I) -> Self::Out {
    /// Output type.
    type Out: IntoIterator;
}

impl<F, I, O> FnIFlatMap<I> for F
where
    F: FnMut(I) -> O,
    O: IntoIterator,
{
    type Out = O;
}
