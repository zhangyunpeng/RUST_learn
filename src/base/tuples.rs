
pub fn run() {
    demo2();
}

// 元组值的移动
fn demo2() {
    let the_date = (
        "Monday".to_string(),
        25, 
        "June".to_string(),
        "2026".to_string(),
    );

    // match the_date {
    //     (d, ..) if d == "Sunday" => println!("It's sunday"),
    //     _ => println!("ohter")
    // }
    // the_date.0 已经被转移所有权
    // println!("date: {:?}", the_date);

    match the_date {
        // ref 关键字
        (ref d, ..) if d == "Sunday" => println!("It's sunday"),
        _ => println!("other")
    }

     match &the_date {
        // ref 关键字
        (d, ..) if d == "Sunday" => println!("It's sunday"),
        _ => println!("other")
    }
    println!("date: {:?}", the_date);


}

fn demo() {
    let t: (i32, String, bool) = (1, "hello".to_string(), true);
    println!("{:?}", t);
    let t = ((1, 2, 3), (4, 5, 6));
    println!("{:?}", t);

    let rcvd_data = (5, "hello", 10);
    match rcvd_data {
        (a, b, c) if a > 0 && c < 20 => {
            println!("Valid data");
        },
        _ => println!("Invalid data"),
    }

    match rcvd_data {
        (_, "hello", _) => {
            println!("hello data");
        },
        _ => println!("no hello data"),
    }

    match rcvd_data {
        (_, _, _c @ 1..100) => {
            println!("valid c data");
        },
        _ => println!("invalid c data"),
    }

    match rcvd_data {
        (_a @ 1..100, ..) => {
            println!("valid a data");
        },
        _ => println!("invalid a data"),
    }
}
