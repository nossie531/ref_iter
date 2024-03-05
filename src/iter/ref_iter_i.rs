//! Provider of [`RefIterI`].

use crate::util::{lifetime, msg};
use crate::{RefItem, RefIterator, RefToken};
use core::any::Any;
use core::cell::Ref;
use core::ops::Deref;

/// Static typing iterator wrapper for [`Ref`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::iter::RefIterI;
/// # use ref_iter::{RefIterator, RefToken};
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples);
/// let token = RefToken::new();
/// let mut iter = RefIterI::new(src.borrow(), &token, |x| x.iter());
/// assert_eq!(iter.next().unwrap().get(&token), &1);
/// assert_eq!(iter.next().unwrap().get(&token), &2);
/// assert_eq!(iter.next().unwrap().get(&token), &3);
/// ```
#[must_use = msg::iter_must_use!()]
pub struct RefIterI<'a, I> {
    /// Dynamic borrowing source.
    #[allow(dead_code)]
    refs: Ref<'a, dyn Any>,
    /// Reference token.
    token: &'a RefToken,
    /// Iterator generated from source.
    iter: I,
}

impl<'a, I> RefIterI<'a, I> {
    /// Create a new value.
    pub fn new<S, F>(refs: Ref<'a, S>, token: &'a RefToken, f: F) -> Self
    where
        S: Any,
        F: Fn(&'a S) -> I,
    {
        let src = unsafe { lifetime::reset_ref(refs.deref()) };
        Self {
            refs,
            token,
            iter: f(src),
        }
    }
}

impl<'a, I, T> Iterator for RefIterI<'a, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    type Item = &'a RefItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| RefItem::use_ref(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, T> RefIterator<'a> for RefIterI<'a, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    fn ref_token<'s>(&'s self) -> &'a RefToken
    where
        'a: 's,
    {
        self.token
    }
}
