use core::cell::RefCell;
use ref_iter::{iter::RefIter, RefIterator};

#[test]
fn test() {
    let samples = vec![1, 2, 3];
    let src = RefCell::new(samples);
    let mut iter = RefIter::new(src.borrow(), |x| x.iter());
    loop {
        iter.next();
    }


    // let samples = vec![1, 2, 3];
    // let src = RefCell::new(samples);
    // let mut iter = RefIter::new(src.borrow(), |x| x.iter());
    // let iter2 = RefIter::new(src.borrow(), |x| x.iter());
    // let item = iter.next().unwrap();
    // drop(iter);
    // item;
}
