//! Private items for macro.

use crate::{IntoRefIterator, IntoRefMutIterator, RefIterator, RefMutIterator};

/// Exec immutable for loop.
///
/// This function is intended for use from macro.
pub fn exec_for_ref<I, F>(iter: I, mut f: F)
where
    I: IntoRefIterator,
    F: FnMut(&I::Item),
{
    let mut iter = iter.into_ref_iter();
    while let Some(item) = iter.next() {
        f(item);
    }
}

/// Exec mutable for loop.
///
/// This function is intended for use from macro.
pub fn exec_for_ref_mut<I, F>(iter: I, mut f: F)
where
    I: IntoRefMutIterator,
    F: FnMut(&mut I::Item),
{
    let mut iter = iter.into_ref_mut_iter();
    while let Some(item) = iter.next_mut() {
        f(item);
    }
}
