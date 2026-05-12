use std::str::FromStr;
use std::num::ParseIntError;

pub fn run() {
    demo_as();
    demo_try_into();
    demo1();
    demo2();
    demo3();
    demo4();
    demo5();
}


fn demo5() {
    use std::str::FromStr;
    let parsed: i32 = "5".parse().unwrap();
    let parsed2 = "10".parse::<i32>().unwrap();
    let parsed3 = i32::from_str("20").unwrap();
    assert_eq!(parsed + parsed2 + parsed3, 35);
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|x|x.trim()).collect();
        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;
        Ok(Point { x, y })
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn demo4() {
    let origin = Point { x: 0, y: 0 };
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    let p = "(3, 4)".parse::<Point>();
    assert_eq!(p.unwrap(), Point{ x: 3, y: 4} );

    println!("Success!")
}

fn demo3() {
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let i3: u32 = 'a'.into();
}

#[allow(overflowing_literals)]
fn demo1() {
    assert_eq!(u8::MAX, 255);
    let v = 1000 as u8;
    println!("{:?}", v);
}

fn demo2() {
    let arr: [u64; 13] = [0;13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        println!("{}", std::mem::size_of_val(&*b));
    }
}

/*
 * 点操作符
 * 在调用时，会发生很多魔法般的类型转换， 如：自动引用、自动解引用、强制类型转换知道类型能匹配等
 * 假设有一个方法 foo，它有一个接收器（接收器是 self、&self、&mut self），如果调用 value.foo()，编译器在调用 foo 之
 * 前，需要决定使用哪个 Self 类型来调用。现在假设 value 拥有类型 T。
 * 1， 首先， 检查是否可以直接调用 T:foo(value), 称之为值调用
 * 2. 如果上一步无法完成，编译器会尝试增加自动引用，例如会尝试一下调用：
 *    <&T>::foo(value) 和 <&mut T>::foo(value), 称之为引用方法调用
 * 3. 如果上一步依然不工作，编译器会试着解引用T，然后再进行尝试。这里使用了 Deref 特征。若 T: Deref<Target = U>( T
 *    可以被解引用为 U ），那么编译器会使用 U 类型进行尝试， 称之为解引用方法调用
 * 4. 若 T 不能被解引用， 且 T 是一个定长类型（在编译器类型长度已知）， 编译器会尝试将 T 从定长类型转为不定长类型， 如
 *    将 [i32;2] 转为 [i32]
 * 5. 如果上述都不行， 编译失败
 * 具体示例如下， 代码如下：
 *    let array: Rc<Box<[T;3]>> = ...;
 *    let first_entry = array[0];
 * array 数组的底层数据隐藏在了重重封锁之后，那么编译器如何使用 array[0] 这种数组原生访问语法通过重重封锁，准确的访问到数
 * 组中的第一个元素？
 * 1. 首先， array[0] 是 Index 特征的语法糖：编译器会将 array[0] 转换为 array.index(0) 调用，当然在调用之前，编译
 *    器会先检查 array 是否实现了 Index 特征
 * 2. 接着， 编译器检查 Rc<Box[T;3]> 是否实现 Index 特征， 结果是否， 不仅如此， &Rc<Box[T;3]> 与
 *    &mut Rc<Box[T;3]> 也没有实现
 * 3. 此时继续对 Box<[T;3]> 进行上面的操作：  Box<[T;3]>, &Box<[T;3]> 和 &mut Box<[T;3]> 都没有实现 Index 特
 *    征， 所有编译器开始对 Box<[T;3]> 进行解引用， 得到 [T;3]
 * 4. [T;3] 以及它的各种引用都没有实现 Index 索引（实际上数组切片才可以通过索引访问）,因此他不能再解引用，编译器只能将定
 *    长转为不定长，因此 [T;3] 被转换称 [T] ，也就是数组切片，它实现了 Index 特征，因此最终我们可以通过 index 访问到
 *    对应的元素
 */
fn demo_dot() {

}

/*
 * 首先检查值调用， value 类型是 &T， 同时 clone 方法签名是 &T : fn clone(&T) -> T， 因此可以进行值方法调用，再加
 * 上编译器知道了 T 实现了 Clone， 因此 cloned 的类型是 T
 */
fn do_stuff<T: Clone>(value: &T) {
    let _cloned = value.clone();
}

/*
 * 首先 T 没有实现 Clone， 也就无法调用 T 的 clone方法， 此时 T 变成 &T， 在这种情况下， clone的方法签名如下：
 * fn clone(&&T) -> &T, 编译器发现 &T 实现了 Clone， 因此推出 cloned 也是 &T 类型
 */
fn do_stuff2<T>(value: &T) {
    let _cloned = value.clone();
}


/*
 * 隐式类型转换
 * &i32 可以隐式转换作为 Trait，&mut i32 不可以隐式转换作为 Trait
 */
fn demo() {
    let t: &i32 = &5;
    foo(t);

    // let t: &mut i32 = &mut 100;
    // foo(t);
}

trait Trait {}
fn foo<X: Trait>(t: X) {}

impl<'a> Trait for &'a i32 {}


/*
 * std::conver::TryInto 转换
 * 类型转换上拥有完全的控制，而不依赖内置转换
 */
fn demo_try_into() {
    use std::convert::TryInto;
    let b: i16 = 1500;
    let b: u8 = b.try_into().unwrap_or_else(|e| {
        println!("{}", e);
        0
    });
    println!("{}", b);
}

/*
 * as 类型转换不具备传递性
 * e as u1 as u2 合法，也不能认为 e as u2 合法
 */
fn demo_as() {
    let a = 3.1 as u8;
    let b = 100_u8 as u8;
    let c = 'a' as u8;
    println!("a = {}, b = {}, c = {}", a, b, c);

    let mut values = vec![1, 2, 3];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values, vec![1, 3, 3]);
}