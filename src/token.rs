//! Provider of [`Token`].

/// Key for [`RefItem`][crate::RefItem].
pub struct Token();

impl Token {
    /// Create new value.
    pub(crate) fn new() -> Self {
        Self()
    }
}
