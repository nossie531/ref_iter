//! Provider of [`RefIter`].

use crate::util::{lifetime, msg};
use crate::{RefItem, RefIterator, RefToken};
use alloc::boxed::Box;
use core::any::Any;
use core::cell::Ref;
use core::ops::Deref;

/// Dynamic typing iterator wrapper for [`Ref`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::iter::RefIter;
/// # use ref_iter::{RefIterator, RefToken};
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples);
/// let token = RefToken::new();
/// let mut iter = RefIter::new(src.borrow(), &token, |x| x.iter());
/// assert_eq!(iter.next().unwrap().get(&token), &1);
/// assert_eq!(iter.next().unwrap().get(&token), &2);
/// assert_eq!(iter.next().unwrap().get(&token), &3);
/// ```
#[must_use = msg::iter_must_use!()]
#[cfg(feature = "alloc")]
#[cfg_attr(docs, doc(cfg(feature = "alloc")))]
pub struct RefIter<'a, T> {
    /// Dynamic borrowing source.
    #[allow(dead_code)]
    refs: Ref<'a, dyn Any>,
    /// Reference token.
    token: &'a RefToken,
    /// Iterator generated from source.
    iter: Box<dyn Iterator<Item = &'a T> + 'a>,
}

impl<'a, T> RefIter<'a, T> {
    /// Create a new value.
    pub fn new<S, I, F>(refs: Ref<'a, S>, token: &'a RefToken, f: F) -> Self
    where
        S: Any,
        I: Iterator<Item = &'a T> + 'a,
        F: Fn(&'a S) -> I,
    {
        let src = unsafe { lifetime::reset_ref(refs.deref()) };
        Self {
            refs,
            token,
            iter: Box::new(f(src)),
        }
    }
}

impl<'a, T> Iterator for RefIter<'a, T> {
    type Item = &'a RefItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| RefItem::use_ref(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> RefIterator<'a> for RefIter<'a, T> {
    fn ref_token<'s>(&'s self) -> &'a RefToken
    where
        'a: 's,
    {
        self.token
    }
}
