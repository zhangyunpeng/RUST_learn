pub fn run() {
    demo_append();
}

fn demo_append() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];
    assert_eq!(v1.len(), 3);
    assert_eq!(v1.capacity(), 3);
    assert_eq!(v2.len(), 3);
    assert_eq!(v2.capacity(), 3);

    v1.append(&mut v2);
    assert_eq!(v1.len(), 6);
    assert_eq!(v1.capacity(), 6);
    assert_eq!(v2.len(), 0);
    assert_eq!(v2.capacity(), 3);
}

fn demo_splice() {
    let mut v = vec![1, 2, 3, 4, 5];
    let new = [6, 7, 8];
    let u = v.splice(1..3, new).collect::<Vec<i32>>();
    assert_eq!(v, [1, 6, 7, 8, 4, 5]);
    assert_eq!(u, [2, 3]);
}

fn demo_split() {
    let numbers = [1, 2, 3, 4, 5, 7];
    let mut iters = numbers.split(|x| x % 3 == 0);
    assert_eq!(iters.next().unwrap(), [1, 2]);
    assert_eq!(iters.next().unwrap(), [4, 5, 7]);
    assert!(iters.next().is_none());

    let numbers = [1, 2, 3, 4, 5, 7];
    let mut iters = numbers.rsplit(|x| x % 3 == 0);
    assert_eq!(iters.next().unwrap(), [4, 5, 7]);
    assert_eq!(iters.next().unwrap(), [1, 2]);
    assert!(iters.next().is_none());

    let numbers = [1, 2, 3, 4, 5, 6, 7];
    let mut iters = numbers.splitn(2, |x| x % 3 == 0);
    assert_eq!(iters.next().unwrap(), [1, 2]);
    assert_eq!(iters.next().unwrap(), [4, 5, 6, 7]);
    assert!(iters.next().is_none());

    let numbers = [1, 2, 3, 4, 5, 6, 7];
    let mut iters = numbers.rsplitn(2, |x| x % 3 == 0);
    assert_eq!(iters.next().unwrap(), [7]);
    assert_eq!(iters.next().unwrap(), [1, 2, 3, 4, 5]);
    assert!(iters.next().is_none());

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let first_three = numbers.split_off(3);
    assert_eq!(first_three, [4, 5, 6]);
    assert_eq!(numbers, [1, 2, 3]);

    let numbers = [1, 2, 3, 4, 5];
    let (left, right) = numbers.split_at(2);
    println!("left: {:?}, right: {:?}", left, right);

    let mut numbers = [1, 2, 3, 4, 5];
    let (left, right) = numbers.split_at_mut(2);
    left[0] += 1;
    right[0] += 1;
    assert_eq!(left, [2, 2]);
    assert_eq!(right, [4, 4, 5]);

    let numbers = [1, 2, 3, 4, 5];
    unsafe {
        let (left, right) = numbers.split_at_unchecked(2);
        assert_eq!(left, [1, 2]);
        assert_eq!(right, [3, 4, 5]);
    }

    let mut numbers = [1, 2, 3, 4, 5];
    unsafe {
        let (left, right) = numbers.split_at_mut_unchecked(2);
        left[0] += 1;
        right[0] += 1;
        assert_eq!(left, [2, 2]);
        assert_eq!(right, [4, 4, 5]);
    }
}

fn demo_retain() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.retain(|x| x % 2 == 0);
    println!("numbers: {:?}", numbers);

    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.retain_mut(|x| {
        if *x <= 3 {
            *x += 1;
            true
        } else {
            false
        }
    });
    println!("numbers: {:?}", numbers);
}

fn demo2() {
    let mut number = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let evens = number.extract_if(.., |x| &*x % 2 == 0).collect::<Vec<_>>();
    let odds = number;
    println!("evens: {:?}", evens);
    println!("odds: {:?}", odds);
}

fn demo1() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("{:?}, cap: {}", v, v.capacity());
    let _ = v.drain(1..=3);
    println!("{:?}, cap: {}", v, v.capacity());

    let mut v = vec![1, 2];
    v.extend([4, 5, 6]);
    println!("{:?}", v);

    for item in v.drain(..) {
        println!("{}", item);
    }
    println!("{:?}", v);
}
