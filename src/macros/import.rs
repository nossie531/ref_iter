//! Macro imports.

/// Immutable for-in loop with dynamic borrowings.
///
/// # Examples
///
/// ```
/// # use ref_iter::prelude::*;
/// # use std::cell::RefCell;
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let iter = RefIter::new(cell.borrow(), |x| x.iter());
/// let mut sum = 0;
///
/// for_ref!(x in iter {
///     sum += *x
/// });
///
/// assert_eq!(sum, 6);
/// ```
#[macro_export]
macro_rules! for_ref {
    ($($args:tt)*) => {
        $crate::ref_iter_macro::for_ref!($($args)*)
    };
}

/// Immutable key-value for-in loop with dynamic borrowings.
///
/// # Examples
///
/// ```
/// # use ref_iter::prelude::*;
/// # use std::cell::RefCell;
/// # use std::collections::HashMap;
/// #
/// let samples = HashMap::from([(1, 1.0f32), (2, 2.0), (3, 3.0)]);
/// let cell = RefCell::new(samples.clone());
/// let iter = RefIter::new(cell.borrow(), |x| x.iter());
/// for_ref_kv!((k, v) in iter {
///     assert_eq!(*k as f32, *v);
/// });
/// ```
#[macro_export]
macro_rules! for_ref_kv {
    ($($args:tt)*) => {
        $crate::ref_iter_macro::for_ref_kv!($($args)*)
    };
}

/// Mutable for-in loop with dynamic borrowings.
///
/// # Examples
///
/// ```
/// # use ref_iter::prelude::*;
/// # use std::cell::RefCell;
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
#[macro_export]
macro_rules! for_ref_mut {
    ($($args:tt)*) => {
        $crate::ref_iter_macro::for_ref_mut!($($args)*)
    };
}

/// Mutable key-value for-in loop with dynamic borrowings.
///
/// # Examples
///
/// ```
/// # use ref_iter::prelude::*;
/// # use std::cell::RefCell;
/// # use std::collections::HashMap;
/// #
/// let mut samples = HashMap::from([(1, 0.0f32), (2, 0.0), (3, 0.0)]);
/// let cell = RefCell::new(samples);
/// let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
/// for_ref_mut_kv!((k, v) in iter {
///     *v = *k as f32;
/// });
///
/// let iter = RefIter::new(cell.borrow(), |x| x.iter());
/// for_ref_kv!((k, v) in iter {
///     assert_eq!(*k as f32, *v);
/// });
/// ```
#[macro_export]
macro_rules! for_ref_mut_kv {
    ($($args:tt)*) => {
        $crate::ref_iter_macro::for_ref_mut_kv!($($args)*)
    };
}
