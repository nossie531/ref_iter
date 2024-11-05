use ref_iter::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[test]
fn for_ref() {
    with_underscore();
    with_pattern();

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
}

#[test]
fn for_ref_kv() {
    with_underscore();
    with_pattern();

    fn with_underscore() {
        let samples = HashMap::from([(1, 0), (2, 0), (3, 0)]);
        let cell = RefCell::new(samples.clone());
        let iter = RefIter::new(cell.borrow(), |x| x.iter());
        for_ref_kv!(_ in iter {
            // nop.
        });
    }

    fn with_pattern() {
        let samples = HashMap::from([(1, 0), (2, 0), (3, 0)]);
        let cell = RefCell::new(samples.clone());
        let iter = RefIter::new(cell.borrow(), |x| x.iter());
        for_ref_kv!((_k, _v) in iter {
            // nop.
        });
    }
}

#[test]
fn for_ref_mut() {
    with_underscore();
    with_pattern();

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
}

#[test]
fn for_ref_mut_kv() {
    with_underscore();
    with_pattern();

    fn with_underscore() {
        let samples = HashMap::from([(1, 0), (2, 0), (3, 0)]);
        let cell = RefCell::new(samples.clone());
        let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
        for_ref_mut_kv!(_ in iter {
            // nop.
        });
    }

    fn with_pattern() {
        let samples = HashMap::from([(1, 0), (2, 0), (3, 0)]);
        let cell = RefCell::new(samples.clone());
        let iter = RefMutIter::new(cell.borrow_mut(), |x| x.iter_mut());
        for_ref_kv!((_k, _v) in iter {
            // nop.
        });
    }
}
