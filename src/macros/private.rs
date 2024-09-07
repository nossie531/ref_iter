//! Private items for macro.

use crate::{IntoRefIterator, IntoRefMutIterator, RefIterator, RefMutIterator};

/// Exec immutable for loop.
///
/// This function is intended for use from macro.
pub fn exec_for_ref<S, I, F, E>(iter: S, mut f: F)
where
    S: IntoRefIterator<IntoRefIter = I>,
    I: RefIterator<Item = E>,
    F: FnMut(&E),
{
    let mut iter = iter.into_ref_iter();
    while let Some(item) = iter.next() {
        f(item);
    }
}

/// Exec mutable for loop.
///
/// This function is intended for use from macro.
pub fn exec_for_ref_mut<S, I, F, E>(iter: S, mut f: F)
where
    S: IntoRefMutIterator<IntoRefMutIter = I>,
    I: RefMutIterator<Item = E>,
    F: FnMut(&mut E),
{
    let mut iter = iter.into_ref_mut_iter();
    while let Some(item) = iter.next_mut() {
        f(item);
    }
}
