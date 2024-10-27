//! Provider of [`FnMap`].

/// [`FnMut`] with single argument and result.
pub trait FnMap<I>: FnMut(I) -> <Self as FnMap<I>>::Output {
    /// The returned type after the call operator is used.
    type Output;
}

impl<F, I, O> FnMap<I> for F
where
    F: FnMut(I) -> O,
{
    type Output = F::Output;
}
