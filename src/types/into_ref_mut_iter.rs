//! Provider of [`IntoRefMutIter`].

use crate::prelude::*;
use crate::util::msg;
use core::fmt::{Debug, Formatter, Result};

/// Adapter that turns [`Iterator`] into [`RefMutIterator`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::prelude::*;
/// #
/// let mut samples = vec![1, 2, 3];
/// let mut iter = IntoRefMutIter::new(samples.iter_mut());
/// while let Some(item) = iter.next_mut() {
///     *item += 1;
/// }
///
/// let expecteds = vec![2, 3, 4];
/// assert_eq!(samples, expecteds);
/// ```
#[must_use = msg::iter_must_use!()]
pub struct IntoRefMutIter<'a, I, T>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    /// Base iterator.
    iter: I,
}

impl<'a, I, T> IntoRefMutIter<'a, I, T>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    /// Create a new value.
    pub fn new<A: IntoIterator<IntoIter = I>>(iter: A) -> Self {
        Self {
            iter: iter.into_iter(),
        }
    }
}

impl<'a, I, T> Clone for IntoRefMutIter<'a, I, T>
where
    I: Iterator<Item = &'a mut T> + Clone,
    T: 'a,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, I, T> Debug for IntoRefMutIter<'a, I, T>
where
    I: Iterator<Item = &'a mut T> + Debug,
    T: 'a,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("IntoRefMutIter")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<'a, I, T> RefIterator for IntoRefMutIter<'a, I, T>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<&Self::Item> {
        self.iter.next().map(|x| &*x)
    }
}

impl<'a, I, T> RefMutIterator for IntoRefMutIter<'a, I, T>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    fn next_mut(&mut self) -> Option<&mut Self::Item> {
        self.iter.next()
    }
}
