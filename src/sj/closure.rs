pub fn run() {
    demo();
    demo2();
    demo3();
}

fn demo3() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

fn demo() {
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(sum(11), 12);
}

fn demo2() {
    let mut c = Cacher::new(|v| v);
    c.query(1u32);
    println!("Cacher:{:?}", c.query(1u32));
    c.query(10u32);
    println!("Cacher:{:?}", c.query(1u32));
    println!("Cacher:{:?}", c.query(10u32));
}

struct Cacher<T, E: Copy>
where
    T: Fn(E) -> E,
{
    query: T,
    value: Option<E>,
}

impl<T, E: Copy> Cacher<T, E>
where
    T: Fn(E) -> E,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    fn query(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
