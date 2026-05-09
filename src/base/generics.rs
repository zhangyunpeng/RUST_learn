pub fn run() {
    demo1();
}

fn demo1() {
    let arr = [1, 2, 5, 6, 4, 3];
    println!("{:?}", find_max(&arr));

    let arr = [1.10, 2.01, 5.0, 6.321, 4.0, 3.09];
    println!("{:?}", find_max(&arr));
}

fn find_max<T: PartialOrd>(arr: &[T]) -> Option<&T> {
    // if arr.is_empty() {
    //     return None;
    // }
    // let mut max = &arr[0];
    // arr.iter().for_each(|v|if v >= max {max = v});
    // Some(max)
    arr.iter().reduce(|max, v| if v > max { v } else { max })
}
