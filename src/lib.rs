#[cfg(test)]
mod tests {
    #[test]
    fn how_to_get_mut_in_vec() {
        let mut v = [1, 2, 3, 4, 5];
        let target = v.iter_mut().find(|n| **n == 2);
        target.map(|t| *t = 6);
        assert_eq!(v, [1,6,3,4,5]);
    }
}
