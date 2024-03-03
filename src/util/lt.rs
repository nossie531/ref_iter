//! Lifetime operation utilities.

use core::ops::{Deref, DerefMut};

/// Reset immutable reference lifetime.
///
/// # Safety
///
/// Be careful when handling the return value. Access to a referenced
/// target over its lifetime will result in undefined behavior.
#[inline(always)]
pub unsafe fn reset_ref_lifetime<'a, S, D>(src: &S) -> &'a D
where
    S: ?Sized + Deref<Target = D>,
    D: ?Sized,
{
    let src_ptr = src.deref() as *const D;
    unsafe { &*src_ptr }
}

/// Reset mutable reference lifetime.
///
/// # Safety
///
/// Be careful when handling the return value. Access to a referenced
/// target over its lifetime will result in undefined behavior.
#[inline(always)]
pub unsafe fn reset_mut_lifetime<'a, S, D>(src: &mut S) -> &'a mut D
where
    S: ?Sized + DerefMut<Target = D>,
    D: ?Sized,
{
    let src_ptr = src.deref_mut() as *mut D;
    unsafe { &mut *src_ptr }
}
