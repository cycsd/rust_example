use rand::Rng;
#[test]
pub fn usize_saturating_add() {
    let mut u = usize::MIN;
    u = u.saturating_add_signed(-1);
    assert_eq!(u, 0);

    let mut max = usize::MAX;
    max = max.saturating_add(1);
    assert_eq!(max, 18446744073709551615)
}

#[test]
pub fn usize_wrapping_add() {
    let mut u = usize::MIN;
    u = u.wrapping_add_signed(-1);
    assert_eq!(u, usize::MAX);

    let mut max = usize::MAX;
    max = max.wrapping_add(1);
    assert_eq!(max, usize::MIN);

    let minus_one: usize = rand::thread_rng().gen_range(1..usize::MAX);

    assert_eq!(minus_one.wrapping_add(!0), minus_one - 1);
}
