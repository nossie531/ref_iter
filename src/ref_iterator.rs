//! Provider of [`RefIterator`].

use crate::util::msg;
use crate::*;
use core::ops::Deref;

/// Shorthand for iterator item.
/// 
/// The following will have the same meaning.
/// 
/// | approach | code          |
/// | -------- | ------------- |
/// | GAT      | `I::Item<'a>` |
/// | nougat   | `Item<'a, I>` |
pub type Item<'a, I> = Gat!(<I as RefIterator>::Item<'a>);

/// A trait for dynamic borrowing iterators.
#[must_use = msg::iter_must_use!()]
#[gat]
pub trait RefIterator {
    /// The type of the elements being iterated over.
    type Item<'a>
    where
        Self: 'a;

    /// Advances the iterator and returns the next value.
    fn next(&mut self) -> Option<Item<'_, Self>>;

    /// Returns the bounds on the remaining length of the iterator.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Creates an iterator that clone dynamic borrowing elements.
    fn cloned<T>(self) -> RefCloned<Self>
    where
        Self: Sized,
        for<'a> Item<'a, Self>: Deref<Target = T>,
        T: Clone,
    {
        RefCloned::new(self)
    }

    /// Creates an iterator that maps dynamic borrowing elements.
    fn map<F, T>(self, f: F) -> RefMap<Self, F>
    where
        Self: Sized,
        F: for<'a> FnMut(Item<'a, Self>) -> T,
    {
        RefMap::new(self, f)
    }

    /// Determines if the elements of this is equal to those of another.
    fn eq<I>(mut self, other: I) -> bool
    where
        Self: Sized,
        for<'a> Item<'a, Self>: PartialEq<Item<'a, I::IntoRefIter>>,
        I: IntoRefIterator,
    {
        let mut iter = other.into_ref_iter();
        loop {
            let x = self.next();
            let y = iter.next();
            if x.is_none() && y.is_none() {
                return true;
            } else if x.is_some() != y.is_some() {
                return false;
            } else if x.unwrap() != y.unwrap() {
                return false;
            }
        }
    }
}
