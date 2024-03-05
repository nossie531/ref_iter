//! Lifetime operation utilities.

/// Reset immutable reference lifetime.
///
/// # Safety
///
/// Be careful when handling the return value. Access to a referenced
/// target over its lifetime will result in undefined behavior.
#[inline(always)]
pub unsafe fn reset_ref<'a, T>(src: &T) -> &'a T
where
    T: ?Sized,
{
    let src_ptr = src as *const _;
    unsafe { &*src_ptr }
}

/// Reset mutable reference lifetime.
///
/// # Safety
///
/// Be careful when handling the return value. Access to a referenced
/// target over its lifetime will result in undefined behavior.
#[inline(always)]
pub unsafe fn reset_mut<'a, T>(src: &mut T) -> &'a mut T
where
    T: ?Sized,
{
    let src_ptr = src as *mut _;
    unsafe { &mut *src_ptr }
}
