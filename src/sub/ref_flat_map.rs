//! Provider of [`RefFlatMap`].

use crate::closures::{FnFlatMap, FnFlatMapItem, FnFlatMapIter};
use crate::prelude::*;
use crate::util::msg;
use core::fmt::{self, Debug, Formatter};
use core::mem;

/// An iterator that flatten maped each dynamic borrowing iterators.
///
/// This struct is created by [`RefIterator::flat_map`].
#[must_use = msg::iter_must_use!()]
pub struct RefFlatMap<'this, I, F>
where
    I: RefIterator,
    F: for<'x> FnFlatMap<&'x I::Item>,
{
    /// Base iterator.
    iter: I,
    /// Sub iterator.
    sub_iter: Option<FnFlatMapIter<F, &'this I::Item>>,
    /// Closure for each item mapping.
    f: F,
}

impl<'this, I, F> RefFlatMap<'this, I, F>
where
    I: RefIterator,
    F: for<'x> FnFlatMap<&'x I::Item>,
{
    /// Creates a new value.
    pub fn new(iter: I, f: F) -> Self {
        let sub_iter = None;
        Self { iter, sub_iter, f }
    }

    /// Advance base iterator and returns next sub iterator.
    fn next_sub_iter(&mut self) -> Option<FnFlatMapIter<F, &'this I::Item>> {
        let ret = (self.f)(self.iter.next()?).into_ref_iter();
        Some(unsafe { mem::transmute(ret) })
    }
}

impl<'this, I, F> Clone for RefFlatMap<'this, I, F>
where
    I: RefIterator + Clone,
    F: for<'x> FnFlatMap<&'x I::Item> + Clone,
    FnFlatMapIter<F, &'this I::Item>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            sub_iter: self.sub_iter.clone(),
            f: self.f.clone(),
        }
    }
}

impl<'this, I, F> Debug for RefFlatMap<'this, I, F>
where
    I: RefIterator + Debug,
    F: for<'x> FnFlatMap<&'x I::Item> + Debug,
    FnFlatMapIter<F, &'this I::Item>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RefFlatMap")
            .field("iter", &self.iter)
            .field("sub_iter", &self.sub_iter)
            .field("f", &self.f)
            .finish()
    }
}

impl<I, F> RefIteratorBase for RefFlatMap<'_, I, F>
where
    I: RefIterator,
    F: for<'x> FnFlatMap<&'x I::Item>,
{
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl<'this, I, F> RefIterator for RefFlatMap<'this, I, F>
where
    I: RefIterator,
    F: for<'x> FnFlatMap<&'x I::Item>,
{
    type Item = FnFlatMapItem<F, &'this I::Item>;

    fn next(&mut self) -> Option<&Self::Item> {
        if self.sub_iter.is_none() {
            self.sub_iter = self.next_sub_iter();
        }

        loop {
            let sub_iter = self.sub_iter.as_mut().unwrap();
            let item = sub_iter.next();
            if item.is_some() {
                return unsafe { mem::transmute(item) };
            }

            self.sub_iter = Some(self.next_sub_iter()?);
        }
    }
}

impl<I, F> ExactSizeRefIterator for RefFlatMap<'_, I, F>
where
    Self: RefIterator,
    I: ExactSizeRefIterator,
    F: for<'x> FnFlatMap<&'x I::Item>,
{
    // nop.
}
