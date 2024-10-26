//! Provider of [`RefMutIter`].

use crate::util::{lifetime, msg};
use crate::*;
use alloc::boxed::Box;
use core::any::Any;
use core::cell::RefMut;
use core::fmt::{Debug, Formatter, Result};
use nameof::name_of;

/// Dynamic typing iterator wrapper for [`RefMut`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let mut iter = DynRefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
/// while let Some(item) = iter.next_mut() {
///     *item += 1;
/// }
/// drop(iter);
///
/// let results = cell.into_inner();
/// let expecteds = vec![2, 3, 4];
/// assert_eq!(results, expecteds);
/// ```
#[must_use = msg::iter_must_use!()]
pub struct DynRefMutIter<'a, T> {
    /// Dynamic borrowing source.
    _src: RefMut<'a, dyn Any>,
    /// Iterator generated from source.
    iter: Box<dyn Iterator<Item = &'a mut T> + 'a>,
}

impl<'a, T> DynRefMutIter<'a, T> {
    /// Create a new value.
    pub fn new<S, I, F>(mut src: RefMut<'a, S>, f: F) -> Self
    where
        S: Any,
        I: Iterator<Item = &'a mut T> + 'a,
        F: Fn(&'a mut S) -> I,
    {
        unsafe {
            let cell_val = lifetime::reset_mut(&mut *src);
            let cell_iter = Box::new(f(cell_val));
            Self {
                _src: src,
                iter: cell_iter,
            }
        }
    }
}

impl<'a, T> Debug for DynRefMutIter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct(type_name!(DynRefMutIter<'a, T>))
            .field(name_of!(_src in DynRefMutIter<'a, T>), &self._src)
            .finish_non_exhaustive()
    }
}

#[test]
fn test_debug() {
    use std::cell::RefCell;
    let cell = RefCell::new([1, 2, 3]);
    let target = DynRefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
    let result = format!("{target:?}");
    assert_eq!(result, "DynRefMutIter { _src: Any { .. }, .. }");
}

impl<'a, T> RefIterator for DynRefMutIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next().map(|x| &*x)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> RefMutIterator for DynRefMutIter<'a, T> {
    fn next_mut(&mut self) -> Option<&mut Self::Item> {
        self.iter.next()
    }
}
