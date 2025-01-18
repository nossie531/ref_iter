//! Tests that should be compile errors.

/// Test about [`crate::for_ref`].
///
/// Loop item should be irrefutable pattern.
///
/// ```compile_fail
/// # use ref_iter::prelude::*;
/// # use std::cell::RefCell;
/// #
/// let samples = vec![Some(1), Some(2), None];
/// let cell = RefCell::new(samples.clone());
/// let iter = RefIter::new(cell.borrow(), |x| x.iter());
/// for_ref!(&Some(_) in iter {
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
/// # use ref_iter::prelude::*;
/// # use std::cell::RefCell;
/// #
/// let samples = vec![Some(1), Some(2), None];
/// let cell = RefCell::new(samples.clone());
/// let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
/// for_ref_mut!(&mut Some(_) in iter {
///     // nop.
/// });
/// ```
fn _for_ref_mut() {
    // nop.
}
