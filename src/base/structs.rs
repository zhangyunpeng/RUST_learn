pub fn run() {
    demo1();
}

#[derive(Debug, Clone, Default)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Wife(Person, Person);

fn demo1() {
    let mut p = Person {
        name: "sun".to_string(),
        age: 18,
    };

    // #[derive(Debug)] 派生 Debug实现
    println!("{:?}", p);

    p.age = 20;
    println!("{:?}", p);

    let name = p.name;
    println!("{}", name);
    // println!("{:?}", p); // err p.name的所有权 被转移给了name
    p.name = "sun2".to_string();
    println!("{:?}", p);

    let p = Person::default();
    let p1 = Person {
        name: "name".to_string(),
        ..Default::default()
    };
    println!("{:?}", p);
    println!("{:?}", p1);

    let w = Wife(p, p1);
    println!("{:?}", w);
}
