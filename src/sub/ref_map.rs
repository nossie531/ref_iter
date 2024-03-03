//! Provider of [`RefMap`].

use crate::token::Token;
use crate::util::msg;
use crate::RefIterator;

/// An Iterator that maps dynamic borrowing elements.
///
/// This struct is created by the [`RefIterator::ref_map`].
#[must_use = msg::iter_must_use!()]
#[derive(Clone)]
pub struct RefMap<I, F> {
    /// Base iterator.
    iter: I,
    /// Action closure for each iterator item.
    f: F,
}

impl<I, F> RefMap<I, F> {
    /// Create new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<T, I, F> Iterator for RefMap<I, F>
where
    I: RefIterator,
    F: FnMut(I::Item, &Token) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let token = Token::new();
        self.iter.next().map(|x| (self.f)(x, &token))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
