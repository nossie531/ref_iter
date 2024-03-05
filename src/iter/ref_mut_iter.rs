//! Provider of [`RefMutIter`].

use crate::util::{lifetime, msg};
use crate::{RefItem, RefIterator, RefToken};
use alloc::boxed::Box;
use core::any::Any;
use core::cell::RefMut;
use core::ops::DerefMut;

/// Dynamic typing iterator wrapper for [`RefMut`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::iter::RefMutIter;
/// # use ref_iter::{RefIterator, RefToken};
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples);
/// let token = RefToken::new();
/// let mut iter = RefMutIter::new(src.borrow_mut(), &token, |x| x.iter_mut());
/// for item in iter {
///     *item.get_mut(&token) += 1;
/// }
///
/// let mut iter = RefMutIter::new(src.borrow_mut(), &token, |x| x.iter_mut());
/// assert_eq!(iter.next().unwrap().get(&token), &2);
/// assert_eq!(iter.next().unwrap().get(&token), &3);
/// assert_eq!(iter.next().unwrap().get(&token), &4);
/// ```
#[must_use = msg::iter_must_use!()]
#[cfg(feature = "alloc")]
#[cfg_attr(docs, doc(cfg(feature = "alloc")))]
pub struct RefMutIter<'a, T> {
    /// Dynamic borrowing source.
    #[allow(dead_code)]
    refs: RefMut<'a, dyn Any>,
    /// Reference token.
    token: &'a RefToken,
    /// Iterator generated from source.
    iter: Box<dyn Iterator<Item = &'a mut T> + 'a>,
}

impl<'a, T> RefMutIter<'a, T> {
    /// Create a new value.
    pub fn new<S, I, F>(mut refs: RefMut<'a, S>, token: &'a RefToken, f: F) -> Self
    where
        S: Any,
        I: Iterator<Item = &'a mut T> + 'a,
        F: Fn(&'a mut S) -> I,
    {
        let src = unsafe { lifetime::reset_mut(refs.deref_mut()) };
        Self {
            refs,
            token,
            iter: Box::new(f(src)),
        }
    }
}

impl<'a, T> Iterator for RefMutIter<'a, T> {
    type Item = &'a mut RefItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| RefItem::use_mut(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> RefIterator<'a> for RefMutIter<'a, T> {
    fn ref_token<'s>(&'s self) -> &'a RefToken
    where
        'a: 's,
    {
        self.token
    }
}
