//! Provider of [`IFlatMap`].

use crate::closures::{FnIFlatMap, FnIFlatMapItem, FnIFlatMapIter};
use crate::prelude::*;
use crate::util::msg;
use core::fmt::{self, Debug, Formatter};
use core::mem;

/// An iterator that flatten maped each dynamic borrowing iterators.
///
/// This struct is created by [`RefIterator::iflat_map`].
#[must_use = msg::iter_must_use!()]
pub struct IFlatMap<'this, I, F>
where
    I: RefIterator,
    F: for<'x> FnIFlatMap<&'x I::Item>,
{
    /// Base iterator.
    iter: I,
    /// Sub iterator.
    sub_iter: Option<FnIFlatMapIter<F, &'this I::Item>>,
    /// Closure for each item mapping.
    f: F,
}

impl<'this, I, F> IFlatMap<'this, I, F>
where
    I: RefIterator,
    F: for<'x> FnIFlatMap<&'x I::Item>,
{
    /// Creates a new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        let sub_iter = None;
        Self { iter, sub_iter, f }
    }

    /// Advance base iterator and returns next sub iterator.
    fn next_sub_iter(&mut self) -> Option<FnIFlatMapIter<F, &'this I::Item>> {
        let ret = (self.f)(self.iter.next()?).into_iter();
        Some(unsafe { Self::adjust_sub_iter(ret) })
    }

    /// Adjust sub iterator lifetime.
    unsafe fn adjust_sub_iter<'a>(
        x: FnIFlatMapIter<F, &'_ I::Item>,
    ) -> FnIFlatMapIter<F, &'a I::Item> {
        #[rustfmt::skip]
        return unsafe {
            mem::transmute::<
                FnIFlatMapIter<F, &'_ _>,
                FnIFlatMapIter<F, &'a _>
            >(x)
        };
    }
}

impl<'this, I, F> Clone for IFlatMap<'this, I, F>
where
    I: RefIterator + Clone,
    F: for<'x> FnIFlatMap<&'x I::Item> + Clone,
    FnIFlatMapIter<F, &'this I::Item>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            sub_iter: self.sub_iter.clone(),
            f: self.f.clone(),
        }
    }
}

impl<'this, I, F> Debug for IFlatMap<'this, I, F>
where
    I: RefIterator + Debug,
    F: for<'x> FnIFlatMap<&'x I::Item> + Debug,
    FnIFlatMapIter<F, &'this I::Item>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RefFlatMap")
            .field("iter", &self.iter)
            .field("sub_iter", &self.sub_iter)
            .field("f", &self.f)
            .finish()
    }
}

impl<'this, I, F> Iterator for IFlatMap<'this, I, F>
where
    I: RefIterator,
    F: for<'x> FnIFlatMap<&'x I::Item>,
{
    type Item = FnIFlatMapItem<F, &'this I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sub_iter.is_none() {
            self.sub_iter = self.next_sub_iter();
        }

        loop {
            let item = self.sub_iter.as_mut().unwrap().next();
            if item.is_some() {
                return item;
            }

            self.sub_iter = Some(self.next_sub_iter()?);
        }
    }
}
