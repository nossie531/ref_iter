//! Provider of [`RefMutIterI`].

use crate::util::{lifetime, msg};
use crate::{RefItem, RefIterator, RefToken};
use core::any::Any;
use core::cell::RefMut;
use core::ops::DerefMut;

/// Static typing iterator wrapper for [`RefMut`].
///
/// # Examples
///
/// ```
/// # use core::cell::RefCell;
/// # use ref_iter::iter::RefMutIterI;
/// # use ref_iter::{RefIterator, RefToken};
/// #
/// let samples = vec![1, 2, 3];
/// let src = RefCell::new(samples);
/// let token = RefToken::new();
/// for item in RefMutIterI::new(src.borrow_mut(), &token, |x| x.iter_mut()) {
///     *item.get_mut(&token) += 1;
/// }
///
/// let mut iter = RefMutIterI::new(src.borrow_mut(), &token, |x| x.iter_mut());
/// assert_eq!(iter.next().unwrap().get(&token), &2);
/// assert_eq!(iter.next().unwrap().get(&token), &3);
/// assert_eq!(iter.next().unwrap().get(&token), &4);
/// ```
#[must_use = msg::iter_must_use!()]
pub struct RefMutIterI<'a, I> {
    /// Dynamic borrowing source.
    #[allow(dead_code)]
    refs: RefMut<'a, dyn Any>,
    /// Reference token.
    token: &'a RefToken,
    /// Iterator generated from source.
    iter: I,
}

impl<'a, I> RefMutIterI<'a, I> {
    /// Create a new value.
    pub fn new<S, F>(mut refs: RefMut<'a, S>, token: &'a RefToken, f: F) -> Self
    where
        S: Any,
        F: Fn(&'a mut S) -> I,
    {
        let src = unsafe { lifetime::reset_mut(refs.deref_mut()) };
        Self {
            refs,
            token,
            iter: f(src),
        }
    }
}

impl<'a, I, T> Iterator for RefMutIterI<'a, I>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    type Item = &'a mut RefItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| RefItem::use_mut(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, T: 'a> RefIterator<'a> for RefMutIterI<'a, I>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
{
    fn ref_token<'s>(&'s self) -> &'a RefToken
    where
        'a: 's,
    {
        self.token
    }
}
