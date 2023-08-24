#[test]
fn how_to_find_mut_in_vec() {
    let mut v = [1, 2, 3, 4, 5];
    let target = v.iter_mut().find(|n| **n == 2);
    target.map(|t| *t = 6);
    assert_eq!(v, [1, 6, 3, 4, 5]);
}

#[test]
fn traverse_monad_iter() {
    let v = vec![0, 1, 2, 3, 4, 5];
    let res: Option<Vec<_>> = v
        .into_iter()
        .map(|n: i32| n.checked_add(i32::MAX))
        .collect();
    assert_eq!(res, None);
}
#[test]

fn traverse_iter() {
    let v = vec![0, 1, 2, 3, 4, 5];
    let res: Vec<_> = v
        .into_iter()
        .filter_map(|n: i32| n.checked_add(i32::MAX))
        .collect();
    assert_eq!(res, vec![i32::MAX]);
}

///
///```
///     let source = vec![1,2,3,4,5,6];

/// (0..source.len()).fold(source.into_iter().filter(|_|true),|src,index|{
///    match index%2 {
///        0=>src.filter(|n|n%2==0).collect(),
///        _=>src.map(|n|n+1),
///         }
/// });
/// assert_eq!(true,false);
///```
/// you can use .collect() parse iterator to vec
/// 
/// but it will be a early loading and cause inefficiently caculate
#[test]
fn how_to_return_different_iterator_in_closure() {
    let source = vec![1, 2, 3, 4, 5, 6];
    let len = source.len();
    let iter: Box<dyn Iterator<Item = i32>> = Box::new(source.into_iter());
    let res = (0..len)
        .fold(iter, |src, index| match index % 2 {
            0 => Box::new(src.filter(|n| n % 2 == 0)),
            _ => Box::new(src.map(|n| n + 2)),
        })
        .collect::<Vec<_>>();
    assert_eq!(res, vec![3]);
}
