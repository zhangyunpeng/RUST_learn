use std::collections::HashMap;

pub fn run() {
    demo();
    demo2();
    demo3();
    demo4();
}

fn demo4() {
    let mut c = Counter::new();
    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/*
 * 消费者适配器
 * 实际上方法调用了 next 消费元素
 * 1. collect 将迭代器收集为指定集合
 * 2. sum 将迭代器总元素求和
 */
fn demo2() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // println!("{:?}", v1_iter);
    println!("{:?}", v1);
}

/*
 * 迭代适配器
 * 是惰性的，不产生任何行为，需要消费者适配器收尾
 * map() 会对迭代器中的每个值进行操作
 * filter() 会对迭代器的值进行过滤
 * zip() 将两个迭代器压缩到一起， 形成 Iterator<Item = (ValueFromA, ValueFromB)> 新的迭代器
 */
fn demo3() {
    let arr = vec![1, 2, 3];
    let v2: Vec<_> = arr.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    let names = vec!["Bob", "Frank", "Ferris"];
    let ages = vec![18, 19, 20];
    let m: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", m);

    let arr = vec![1, 2, 3, 4];
    let arr2 = arr.into_iter().filter(|x| x % 2 == 0).collect::<Vec<_>>();
    println!("{:?}", arr2);
}

fn demo() {
    let arr = vec![1, 2];
    for i in arr {
        println!("{}", i);
    }
    // println!("{:?}", arr);

    let arr = vec![1, 2];
    for i in &arr {
        println!("{}", i);
    }
    println!("{:?}", arr);
    let arr = vec![1, 2];
    let mut arr_iter = arr.iter();
    assert_eq!(Some(&1), arr_iter.next());
    assert_eq!(Some(&2), arr_iter.next());
    assert_eq!(None, arr_iter.next());
    println!("{:?}", arr);

    let arr = vec![1, 2];
    match IntoIterator::into_iter(&arr) {
        mut iter => loop {
            match iter.next() {
                Some(x) => println!("{}", x),
                None => break,
            }
        },
    };

    let mut arr = vec![1, 2];
    for i in &mut arr {
        *i += 1;
    }
    println!("{:?}", arr);
    for i in arr.iter_mut() {
        *i *= 2;
    }
    println!("{:?}", arr);
}
