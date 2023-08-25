#![allow(dead_code, unused_variables, unused)]
use std::{
    iter::{Filter, Map},
    slice::Iter,
};

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

/// # Compile Fail
/// this code would compile fail, because you don't return the same type
///
/// a: Filter
/// b: Map
///
///```compile_fail
///     let source = vec![1,2,3,4,5,6];

///     (0..source.len())
///         .fold(source.into_iter().filter(|_|true),|src,index|{
///             match index%2 {
///                 0=>src.filter(|n|n%2==0),
///                 _=>src.map(|n|n+1),
///             }
///     });
///```
/// you can use .collect() parse iterator to vec
///
/// and you can have the same type Vec<_>
///
/// but it will be a early loading and cause inefficiently caculate
///
#[test]
fn how_to_return_different_iterator_in_closure() {
    let mut source = vec![1, 2, 3, 4, 5, 6];
    let len = source.len();
    let iter: Box<dyn Iterator<Item = &mut i32>> = Box::new(source.iter_mut());
    let res = (0..len)
        .fold(iter, |src, index| match index % 2 {
            0 => Box::new(src.filter(|n| **n % 2 == 0)),
            _ => Box::new(src.map(|n| {
                *n += 2;
                n
            })),
        })
        .map(|&mut n| n)
        .collect::<Vec<_>>();
    assert_eq!(res, vec![8, 10, 12]);
}

///Either
enum Or<L, R> {
    Left(L),
    Right(R),
}

///# Compile Fail
///```compile_fail,E0308
///let source = [1, 2, 3, 4, 5];
///let mut or = Or::Right(source.into_iter().map(|n| n + 2));
///or = Or::Left(source.into_iter().filter(|n| n % 2 == 0));
///
///let res = match or {
///    Or::Left(iter) | Or::Right(iter) =>{
/// //Fn not the same type(a:|n|n+2, b:|n|n%2==0)
///        iter.map(|n|n+4)
///    },
///}
/// ```
#[test]
fn return_different_iterator_by_enum() {
    let source = [1, 2, 3, 4, 5];

    let mut or = Or::Right(source.into_iter().map(|n| n + 2));
    or = Or::Left(source.into_iter().filter(|n| n % 2 == 0));

    let res: Vec<i32> = match or {
        Or::Left(iter) => iter.map(|n| n + 1).collect(),
        Or::Right(iter) => iter.map(|n| n + 4).collect(),
    };
    assert_eq!(
        res,
        source
            .into_iter()
            .filter(|n| n % 2 == 0)
            .map(|n| n + 1)
            .collect::<Vec<_>>()
    );
}
