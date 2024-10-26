use ref_iter::prelude::*;
use std::cell::RefCell;

#[test]
fn for_ref() {
    with_underscore();
    with_pattern();

    #[cfg(feature = "alloc")]
    with_box();

    fn with_underscore() {
        let samples = vec![1, 2, 3];
        let cell = RefCell::new(samples.clone());
        let iter = RefIter::new(cell.borrow(), |x| x.iter());
        for_ref!(_ in iter {
            // nop.
        })
    }

    fn with_pattern() {
        let samples = vec![1, 2, 3];
        let cell = RefCell::new(samples.clone());
        let iter = RefIter::new(cell.borrow(), |x| x.iter());
        for_ref!(&_ in iter {
            // nop.
        })
    }

    #[cfg(feature = "alloc")]
    fn with_box() {
        let samples = vec![1, 2, 3];
        let cell = RefCell::new(samples.clone());
        let iter = RefIter::new(cell.borrow(), |x| x.iter());
        for_ref!(_x in Box::new(iter) {
            // nop.
        })
    }
}

#[test]
fn for_ref_mut() {
    with_underscore();
    with_pattern();

    #[cfg(feature = "alloc")]
    with_box();

    fn with_underscore() {
        let samples = vec![1, 2, 3];
        let cell = RefCell::new(samples.clone());
        let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
        for_ref_mut!(_ in iter {
            // nop.
        })
    }

    fn with_pattern() {
        let samples = vec![1, 2, 3];
        let cell = RefCell::new(samples.clone());
        let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
        for_ref_mut!(&mut _ in iter {
            // nop.
        })
    }

    #[cfg(feature = "alloc")]
    fn with_box() {
        let samples = vec![1, 2, 3];
        let cell = RefCell::new(samples.clone());
        let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
        for_ref_mut!(_x in Box::new(iter) {
            // nop.
        })
    }
}
