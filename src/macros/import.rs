//! Macro imports.

/// for-in loop with [`IntoRefIterator`](crate::IntoRefIterator).
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let iter = RefIter::new(cell.borrow(), |x| x.iter());
/// let mut counter = 0;
///
/// for_ref!(x in iter {
///     assert_eq!(*x, samples[counter]);
///     counter += 1;
/// });
/// ```
///
/// # Document from macro crate
/// <!-- insert -->
pub use ref_iter_macro::for_ref;

/// for-in loop with [`IntoRefMutIterator`](crate::IntoRefMutIterator).
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
///
/// for_ref_mut!(x in iter {
///     *x += 1;
/// });
///
/// let results = cell.into_inner();
/// let expecteds = vec![2, 3, 4];
/// assert_eq!(results, expecteds);
/// ```
///
/// # Document from macro crate
/// <!-- insert -->
pub use ref_iter_macro::for_ref_mut;
