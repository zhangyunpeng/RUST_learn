use std::cmp::Reverse;

pub fn demo() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.sort_by_key(|&num| (num > 3, Reverse(num)));
    assert_eq!(v, vec![3, 2, 1, 5, 4]);
}
