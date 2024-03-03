//! Provider of [`RefItem`].

use crate::token::Token;
use core::mem;

/// Item of dynamic borrowing iterator.
pub struct RefItem<T>(T);

impl<T> RefItem<T> {
    /// Get immutable reference to underlying data.
    pub fn get<'c: 'i, 'i>(&'c self, _: &'i Token) -> &'i T {
        &self.0
    }

    /// Get mutable reference to underlying data.
    pub fn get_mut<'c: 'i, 'i>(&'c mut self, _: &'i Token) -> &'i mut T {
        &mut self.0
    }

    /// Convert immutable reference type from data type to item type.
    pub(crate) fn use_ref(base: &T) -> &Self {
        unsafe { mem::transmute(base) }
    }

    /// Convert mutable reference type from data type to item type.
    pub(crate) fn use_mut(base: &mut T) -> &mut Self {
        unsafe { mem::transmute(base) }
    }
}
