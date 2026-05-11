pub fn run() {
    demo();
    // demo2();
    demo_sort();
    demo_sort2();
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn demo_sort2() {
    let mut peple = vec![
        Person::new("c".to_string(), 20),
        Person::new("a".to_string(), 15),
        Person::new("b".to_string(), 16),
    ];
    peple.sort_unstable();
    println!("{:?}", peple);
}

fn demo_sort() {
    // let mut v = vec![1.0, 5.6, 2.3, 15f32];
    // v.sort_unstable();
    // assert_eq!(v, [1.0, 2.3, 5.6, 15f32]);

    let mut v = vec![1.0, 5.6, 2.3, 15f32];
    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(v, [1.0, 2.3, 5.6, 15f32]);
}

fn demo() {
    let mut v = Vec::new();
    v.push(1);

    let mut v = vec![1, 2, 3];
    v.push(4);
    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 1;
    }

    let v = vec![0; 3];
    let v_from = Vec::from([0; 3]);
    assert_eq!(v, v_from);

    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());
    v.reserve(100);
    println!(
        "Vector（reserve） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );
    v.shrink_to_fit();
    println!(
        "Vector（shrink_to_fit） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    let mut v = vec![1, 2];
    assert!(!v.is_empty());
    v.insert(2, 3);
    assert_eq!(v.remove(1), 2);
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v.pop(), None);
    v.clear();
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn show_addr(addr: IpAddr) {
    println!("{:?}", addr);
}
fn demo2() {
    let v = vec![
        IpAddr::V4(String::from("127.0.0.1")),
        IpAddr::V6(String::from("::1")),
    ];
    for p in v {
        show_addr(p);
    }
}
