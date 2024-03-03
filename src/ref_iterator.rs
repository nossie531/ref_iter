//! Provider of [`RefIterator`].

use crate::sub::{RefCloned, RefMap};
use crate::token::Token;
use crate::util::msg;
use crate::RefItem;

/// A trait for dynamic borrowing iterators.
#[must_use = msg::iter_must_use!()]
pub trait RefIterator: Iterator {
    /// Creates an iterator that clone dynamic borrowing elements.
    fn ref_cloned<'a, T>(self) -> RefCloned<Self>
    where
        Self: Sized + Iterator<Item = &'a RefItem<T>>,
        T: 'a + Clone,
    {
        RefCloned::new(self)
    }

    /// Creates an iterator that maps dynamic borrowing elements.
    fn ref_map<T, F>(self, f: F) -> RefMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item, &Token) -> T,
    {
        RefMap::new(self, f)
    }

    /// Execute action with token.
    fn ref_act<F>(&mut self, mut f: F)
    where
        Self: Sized,
        F: FnMut(&mut Self, &Token),
    {
        f(self, &Token::new())
    }
}
