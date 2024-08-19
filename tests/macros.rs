use core::cell::RefCell;
use ref_iter::prelude::*;

#[test]
fn for_ref() {
    with_underscore();

    fn with_underscore() {
        let samples = vec![1, 2, 3];
        let cell = RefCell::new(samples.clone());
        let iter = RefIter::new(cell.borrow(), |x| x.iter());
        for_ref!((_ in iter) {
            // nop.
        })
    }
}

#[test]
fn for_ref_mut() {
    with_underscore();

    fn with_underscore() {
        let samples = vec![1, 2, 3];
        let cell = RefCell::new(samples.clone());
        let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
        for_ref_mut!((_ in iter) {
            // nop.
        })
    }
}