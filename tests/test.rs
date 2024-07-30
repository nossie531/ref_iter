use core::cell::RefCell;
use ref_iter::iter::RefIter;
use ref_iter::{AsRefIter, RefIterator};

#[test]
fn test() {
    let samples = vec![1, 2, 3];
    let src = RefCell::new(samples.clone());
    let iter = RefIter::new(src.borrow(), |x| x.iter());
    assert!(iter.eq::<AsRefIter<_>>(samples.iter().into()));
}
