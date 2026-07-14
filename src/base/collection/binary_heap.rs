use std::collections::BinaryHeap;

pub fn demo() {
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);
    heap.push(1);
    heap.push(5);
    heap.push(2);
    assert_eq!(heap.peek(), Some(&5));
    assert_eq!(heap.peek(), Some(&5));
    assert_eq!(heap.len(), 3);

    for x in &heap {
        println!("{}", x);
    }

    for x in heap.iter() {
        println!("{}", x);
    }

    // assert_eq!(heap.pop(), Some(5));
    // assert_eq!(heap.pop(), Some(2));
    // assert_eq!(heap.pop(), Some(1));
    // assert_eq!(heap.pop(), None);

    heap.clear();
    assert!(heap.is_empty());

    let heap = BinaryHeap::from(vec![5, 4, 3, 2, 1]);
    println!("{:?}", heap);

    let mut heap = BinaryHeap::with_capacity(10);
    heap.push(1);

    let mut heap = BinaryHeap::new();
    assert!(heap.peek_mut().is_none());
    heap.push(2);
    heap.push(3);
    heap.push(5);
    if let Some(mut val) = heap.peek_mut() {
        *val = 0;
    }
    assert_eq!(heap.peek(), Some(&3));

    let heap = BinaryHeap::from(vec![5, 4, 3, 2, 1]);
    assert_eq!(vec![1, 2, 3, 4, 5], heap.into_sorted_vec());

    let mut heap_a = BinaryHeap::from(vec![2, 1]);
    let mut heap_b = BinaryHeap::from(vec![3, 4]);
    heap_a.append(&mut heap_b);
    assert_eq!(vec![1, 2, 3, 4], heap_a.into_sorted_vec());
    assert!(heap_b.is_empty());

    let mut heap = BinaryHeap::from(vec![5, 4, 3, 2, 1]);
    heap.retain(|&x| x % 2 == 0);
    assert_eq!(vec![2, 4], heap.into_sorted_vec());

    let mut heap = BinaryHeap::new();
    heap.reserve_exact(100);
    assert_eq!(100, heap.capacity());
    heap.push(1);
    assert_eq!(100, heap.capacity());

    let mut heap = BinaryHeap::new();
    heap.reserve(100);
    assert!(heap.capacity() >= 100);
    heap.push(4);
    assert_eq!(100, heap.capacity());

    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    heap.try_reserve_exact(data.len()).unwrap();
    heap.extend(data.iter());
    assert_eq!(10, heap.len());
    assert_eq!(10, heap.capacity());

    let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(100);
    assert_eq!(100, heap.capacity());
    heap.shrink_to_fit();
    assert_eq!(0, heap.capacity());

    let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(100);
    assert_eq!(100, heap.capacity());
    heap.shrink_to(10);
    assert_eq!(10, heap.capacity());
}

use std::cmp::Reverse;
pub fn min_heap() {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(5));
    heap.push(Reverse(3));
    heap.push(Reverse(2));

    // for i in heap.iter()  {
    //     println!("{:?}", i);
    // }

    // assert_eq!(heap.pop(), Some(Reverse(2)));
    // assert_eq!(heap.pop(), Some(Reverse(3)));
    // assert_eq!(heap.pop(), Some(Reverse(5)));

    assert_eq!(heap.pop().unwrap().0, 2);
    assert_eq!(heap.pop().unwrap().0, 3);
    assert_eq!(heap.pop().unwrap().0, 5);
}
