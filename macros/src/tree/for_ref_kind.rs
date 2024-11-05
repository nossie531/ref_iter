//! Provider of [ForRefKind].

/// Type of [ForRef](crate::tree::ForRef).
#[derive(Copy, Clone)]
pub struct ForRefKind {
    mutable: bool,
    for_map: bool,
}

impl ForRefKind {
    pub fn new(mutable: bool, for_map: bool) -> Self {
        Self { mutable, for_map }
    }

    pub fn mutable(&self) -> bool {
        self.mutable
    }

    pub fn for_map(&self) -> bool {
        self.for_map
    }
}