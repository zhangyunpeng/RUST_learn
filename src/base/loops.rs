

pub fn run() {
    demo_for();
}

fn demo_for() {
    let mut arr = [
        "hello".to_string(),
        "world".to_string(), 
    ];
    // for item in arr {
    //     println!("item: {}", item);
    // }

    // for item in arr.into_iter() {
    //     println!("item: {}", item);
    // }

    for item in &arr {
        println!("item: {}", item);
    }

    for item in arr.iter() {
        println!("item: {}", item);
    }

    for item in &mut arr{
        item.push_str("abc");
    }
    println!("arr: {:?}", arr);

    for item in arr.iter_mut() {
        item.push_str("def");
    }
    println!("arr: {:?}", arr);
}

// fn demo_loop() {
//     'outer: loop {
//         println!("outer");
//         loop {
//             println!("inner");
//             loop {
//                 println!("inner2");
//                 break 'outer;
//             }
//         }
//     }
//     println!("Exited outer");
// }

// fn demo_loop2() {
//     let result = 'outer: loop {
//         println!("outer");
//         loop {
//             println!("inner");
//             loop {
//                 println!("inner2");
//                 break 'outer 20;
//             }
//         }
//     };
//     println!("result: {}", result);
// }
