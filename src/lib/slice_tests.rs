#[test]
#[should_panic]
pub fn range_out_of_slice_length() {
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[6..];
    println!("s:{s:?}");
}
#[test]
pub fn range_start_equal_slice_length_is_ok() {
    let v = vec![0, 1, 2, 3, 4];
    assert_eq!(&v[0..], [0, 1, 2, 3, 4]);
    assert_eq!(&v[1..], [1, 2, 3, 4]);
    assert_eq!(&v[2..], [2, 3, 4]);
    assert_eq!(&v[3..], [3, 4]);
    assert_eq!(&v[4..], [4]);
    assert_eq!(&v[v.len()..], []);
}
