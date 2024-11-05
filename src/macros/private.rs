//! Private items for macro.

use crate::prelude::*;

/// Exec immutable for-in loop.
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

/// Exec immutable key-value for-in loop.
///
/// This function is intended for use from macro.
pub fn exec_for_ref_kv<I, F>(iter: I, mut f: F)
where
    I: IntoRefKvIterator,
    F: FnMut((&I::K, &I::V)),
{
    let mut iter = iter.into_ref_kv_iter();
    while let Some(item) = iter.next() {
        f(item);
    }
}

/// Exec mutable for-in loop.
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

/// Exec mutable key-value for-in loop.
///
/// This function is intended for use from macro.
pub fn exec_for_ref_mut_kv<I, F>(iter: I, mut f: F)
where
    I: IntoRefMutKvIterator,
    F: FnMut((&I::K, &mut I::V)),
{
    let mut iter = iter.into_ref_mut_kv_iter();
    while let Some(item) = iter.next_mut() {
        f(item);
    }
}
