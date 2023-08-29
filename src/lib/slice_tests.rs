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

///split at 看成是你想要取得多少元素會比較容易知道split後會有甚麼結果
/// split_at(1)代表你想要取得1個元素在前面
/// split_at(0)代表你想取0個元素在前面，其他剩下的元素都擺在後面
#[test]
pub fn split_at_element() {
    let source = [1, 2, 3, 4, 5, 6];
    {
        let (left, right) = source.split_at(0);
        assert_eq!(left, &[]);
        assert_eq!(right, &[1, 2, 3, 4, 5, 6]);
    }

    {
        let (left, right) = source.split_at(source.len());
        assert_eq!(left, &[1, 2, 3, 4, 5, 6]);
        assert_eq!(right, &[]);
    }
}
