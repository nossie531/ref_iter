//! Provider of [ForRefKind].

/// Type of [ForRef](crate::tree::ForRef).
#[derive(Copy, Clone)]
pub struct ForRefKind {
    /// Mutable loop flag.
    is_mutable: bool,

    /// For map flag.
    is_for_map: bool,
}

impl ForRefKind {
    /// Create a new value.
    pub fn new(is_mutable: bool, is_for_map: bool) -> Self {
        Self {
            is_mutable,
            is_for_map,
        }
    }

    /// Returns `true` if this node is for mutable loop.
    pub fn is_mutable(&self) -> bool {
        self.is_mutable
    }

    /// Returns `true` if this node is for map.
    pub fn is_for_map(&self) -> bool {
        self.is_for_map
    }
}
