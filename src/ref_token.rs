//! Provider of [`RefToken`].

/// Reference token.
pub struct RefToken {
    _private: (),
}

impl RefToken {
    /// Create new instance.
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for RefToken {
    fn default() -> Self {
        Self::new()
    }
}
