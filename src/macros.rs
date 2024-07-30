/// for-in loop macro for [`IntoRefIterator`](crate::IntoRefIterator).
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::iter::RefIter;
/// # use ref_iter::for_ref;
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples.clone());
/// let iter = RefIter::new(src.borrow(), |x| x.iter());
/// let mut counter = 0;
/// for_ref!((x in iter) {
///     assert_eq!(*x, samples[counter]);
///     counter += 1;
/// });
/// ```
#[macro_export]
macro_rules! for_ref {
    (($item:ident in $iter:expr) {$($tt:tt)*}) => {
        let mut iter = ref_iter::IntoRefIterator::into_ref_iter($iter);
        while let Some($item) = ref_iter::RefIterator::next(&mut iter) {
            $($tt)*
        }
    }
}
