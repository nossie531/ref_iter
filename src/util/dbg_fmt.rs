//! Debug format utility.

/// Get type name without type parameter.
macro_rules! type_name {
    ($type:ty) => {{
        use nameof::name_of;
        name_of!(type $type).split('<').next().unwrap()
    }}
}
