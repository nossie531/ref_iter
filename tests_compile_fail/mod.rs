//! Tests that should be compile errors.

/// Test about [`crate::for_ref`].
///
/// Loop item should be irrefutable pattern.
///
/// ```compile_fail
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let samples = vec![Some(1), Some(2), None];
/// let cell = RefCell::new(samples.clone());
/// let iter = RefIter::new(cell.borrow(), |x| x.iter());
/// for_loop!((&Some(_x) in values.iter()) {
///     // nop.
/// });
/// ```
fn _for_ref() {
    // nop.
}

/// Test about [`crate::for_ref_mut`].
///
/// Loop item should be irrefutable pattern.
///
/// ```compile_fail
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let samples = vec![Some(1), Some(2), None];
/// let cell = RefCell::new(samples.clone());
/// let iter = RefIter::new(cell.borrow(), |x| x.iter());
/// for_loop!((&Some(_x) in values.iter()) {
///     // nop.
/// });
/// ```
fn _for_ref_mut() {
    // nop.
}
