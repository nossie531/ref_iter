use core::cell::RefCell;
use std::slice::Iter;
use ref_iter::{iter::RefIter, RefIterator};

#[test]
fn test() {
    let samples = vec![1, 2, 3];
    let src = RefCell::new(samples.clone());
    let iter = RefIter::new(src.borrow(), |x| x.iter());
    assert!(iter.eq::<Iter<_>>(samples.iter()));
}
