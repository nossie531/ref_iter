use crate::{IntoRefIter, RefIterator};

/// Conversion into an [`RefIterator`].
pub trait IntoRefIterator {
    /// The type of the elements being iterated over.
    type Item<'a>
    where
        Self: 'a;

    /// Which kind of iterator are we turning this into?
    type IntoRefIter<'a>: RefIterator<Item<'a> = Self::Item<'a>>
    where
        Self: 'a;

    /// Creates an iterator from a value.
    fn into_ref_iter<'a>(self) -> Self::IntoRefIter<'a>
    where
        Self: 'a;
}

impl<T: IntoIterator> IntoRefIterator for T {
    type Item<'a> = T::Item
    where
        Self: 'a;

    type IntoRefIter<'a> = IntoRefIter<T::IntoIter>
    where
        Self: 'a;

    fn into_ref_iter<'a>(self) -> Self::IntoRefIter<'a>
    where
        Self: 'a,
    {
        IntoRefIter::new(self.into_iter())
    }
}
