use std::collections::BinaryHeap;

#[test]
fn rust_binaryheap_is_max_heap() {
    let source = [2, 3, 100, 80, 70, 200];
    let mut heap = BinaryHeap::from(source);

    assert_eq!(heap.pop(), Some(200));
    assert_eq!(heap.pop(), Some(100));
    assert_eq!(heap.pop(), Some(80));
}

#[test]
pub fn heap_value_is_vec_will_compare_by_each_element_in_vec() {
    let v = vec![vec![-43,-5, 4],vec![-61,-6,-5 ],vec![-52,4,6]];
    let mut heap = BinaryHeap::from(v);
    println!("{heap:?}");
    assert_eq!(heap.pop(),Some(vec![-43,-5, 4]));
    assert_eq!(heap.pop(),Some(vec![-52,4,6]));
    assert_eq!(heap.pop(),Some(vec![-61,-6,-5 ]));
    assert_eq!(heap.pop(),None);
}
