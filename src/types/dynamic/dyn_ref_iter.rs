//! Provider of [`RefIter`].

use crate::util::{lifetime, msg};
use crate::{prelude::*, type_name};
use alloc::boxed::Box;
use core::any::Any;
use core::cell::Ref;
use core::fmt::{Debug, Formatter, Result};
use nameof::name_of;

/// Dynamic typing iterator wrapper for [`Ref`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let samples = vec![1, 2, 3];
/// let cell = RefCell::new(samples.clone());
/// let iter = DynRefIter::new(cell.borrow(), |x| x.iter());
/// assert!(iter.cloned().eq(samples.iter().cloned()));
/// ```
#[must_use = msg::iter_must_use!()]
pub struct DynRefIter<'a, T> {
    /// Dynamic borrowing source.
    _src: Ref<'a, dyn Any>,
    /// Iterator generated from source.
    iter: Box<dyn Iterator<Item = &'a T> + 'a>,
}

impl<'a, T> DynRefIter<'a, T> {
    /// Create a new value.
    pub fn new<S, I, F>(src: Ref<'a, S>, f: F) -> Self
    where
        S: Any,
        I: Iterator<Item = &'a T> + 'a,
        F: Fn(&'a S) -> I,
    {
        unsafe {
            let cell_val = lifetime::reset_ref(&*src);
            let cell_iter = Box::new(f(cell_val));
            Self {
                _src: src,
                iter: cell_iter,
            }
        }
    }
}

impl<'a, T> Debug for DynRefIter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct(type_name!(DynRefIter<'a, T>))
            .field(name_of!(_src in DynRefIter<'a, T>), &self._src)
            .finish_non_exhaustive()
    }
}

#[test]
fn test_debug() {
    use std::cell::RefCell;
    let cell = RefCell::new([1, 2, 3]);
    let target = DynRefIter::new(cell.borrow(), |x| x.iter());
    let result = format!("{target:?}");
    assert_eq!(result, "DynRefIter { _src: Any { .. }, .. }");
}

impl<'a, T> RefIterator for DynRefIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
