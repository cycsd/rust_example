#![allow(dead_code, unused_variables, unused)]

use std::cell::RefCell;

#[test]
fn test_refcell_clone() {
    let ref_cell = RefCell::new(vec![0]);

    let ref_cell_clone = ref_cell.clone();

    ref_cell_clone.borrow_mut().push(1);

    let right = ref_cell.borrow();

    let res = ref_cell_clone.borrow();
    ///refcell clone will use internal value clone
    assert_ne!(right.to_vec(), res.to_vec());
}
