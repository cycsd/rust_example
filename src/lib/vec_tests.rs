#[test]
fn remove_element_in_o1() {
    let mut v = vec![0, 1, 2, 3, 4];
    let origin_last = *v.last().expect("v has elements");
    let remove_index = 2;
    let origin_len = v.len();
    let origin_capacity = v.capacity();
    let remove_element = v.swap_remove(remove_index);

    assert_eq!(remove_element, 2);
    assert_eq!(v.len(), origin_len - 1);
    assert_eq!(v.capacity(), origin_capacity);
    assert_eq!(v[remove_index], origin_last);
    assert_eq!(v, [0, 1, 4, 3]);
}
