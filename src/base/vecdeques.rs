use std::collections::VecDeque;

pub fn run() {
    demo();
}

fn demo() {
    let mut v = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    assert_eq!(v.pop_front(), Some(1));
    assert_eq!(v.pop_front(), Some(2));
    assert!(v.pop_front().is_none());

    let mut v1: VecDeque<i32> = [1, 2, 3].into();
    let mut v2: VecDeque<i32> = [4, 5].into();
    v1.append(&mut v2);
    assert_eq!(v1, [1, 2, 3, 4, 5]);
    assert_eq!(v2, []);

    let mut v = VecDeque::new();
    v.extend(1..5);
    v.retain(|x| x % 2 == 0);
    assert_eq!(v, [2, 4]);

    let mut v = VecDeque::new();
    v.extend(1..5);
    v.retain_mut(|x| {
        if &*x % 2 == 0 {
            *x *= 2;
            true
        } else {
            false
        }
    });
    assert_eq!(v, [4, 8]);
}
