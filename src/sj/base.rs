use std::fmt::Display;
use std::ops::Sub;

pub fn run() {
    int_overflow();
    float_demo();
    array_demo();
}

/*
 * 整型溢出
 * 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
 * 如果使用 checked_* 方法时发生溢出，则返回 None 值
 * 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
 * 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值
 */
fn int_overflow() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    assert_eq!(b, 19);

    let a: u8 = 255;
    let b = a.checked_add(20);
    assert_eq!(b, Option::None);

    let a: u8 = 255;
    let res = a.overflowing_add(20);
    assert_eq!((19, true), res);

    let a: u8 = 255;
    let b = a.saturating_add(20);
    assert_eq!(b, 255);
}

/*
 * 浮点数
 *
 */
fn float_demo() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2); // panic
}

fn array_demo() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    println!(
        "The value of the element at index {} is: {}",
        index, a[index]
    );

    let a = [0; 5];
    println!("{:?}", a);

    let a: [String; 5] = std::array::from_fn(|_| "hello".to_string());
    println!("{:?}", a);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn create_and_print<T>()
where
    T: From<i32> + Display,
{
    let a: T = 100.into();
    println!("{}", a);
}
