pub fn run() {
    demo_fn_once();
    demo_fn_mut();
    demo_fn_mut2();
    demo_fn();
    demo_fn2();
    demo_fn3();
}

/*
 * 闭包自动实现 Copy 特征的规则是：只要闭包捕获的类型都实现了 Copy 特征，这个闭包会默认实现 Copy 特征
 */
fn demo_fn3() {
    // 拿走所有权
    let s = String::from("hello");
    let update_string = move |_| println!("{}", s);

    exec(update_string);
    // exec(update_string); 不能再用了

    // 可变引用
    let mut s = String::new();
    let mut update_string = |str| s.push_str(str);
    exec(update_string);
    // exec(update_string); 不能再用了
}

/*
 * 三种 Fn 的关系
 * 1. 所有的闭包斗自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
 * 2. 没有移除所捕获变量所有权的闭包自动实现了 FnMut 特征
 * 3. 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
 */

fn demo_fn2() {
    let s = String::new();
    let f = || println!("{}", s);

    demo_fn2_exec(f);
    demo_fn2_exec2(f);
    demo_fn2_exec3(f);
}

fn demo_fn2_exec<F: FnOnce()>(f: F) {
    f();
}

fn demo_fn2_exec2<F: FnMut()>(mut f: F) {
    f();
}

fn demo_fn2_exec3<F: Fn()>(f: F) {
    f();
}

/*
 * 闭包从捕获环境中移出了变量的所有权，闭包仅自动实现了 FnOnce
 */
fn demo_fn4() {
    let mut s = String::new();
    let update_string = |str| -> String {
        s.push_str(str);
        s
    };

    //
    // demo_fn4_exec(update_string);
}

fn demo_fn4_exec<'a, F>(mut f: F)
where
    F: FnMut(&'a str) -> String,
{
    f("hello");
}

/*
   Fn
*/
fn demo_fn() {
    let s = "hello".to_string();
    let print_str = |str| println!("{} {}", s, str);
    exec_fn(print_str);
    println!("{}", s);
}

fn exec_fn<F>(f: F)
where
    F: Fn(String) -> (),
{
    f("world".to_string());
}

/*
 * FnMut
 * 1. FnMut 的使用必须有 mut 接收
 *      a. demo_fn_mut 中， mut update_string 接收
 *      b. demo_fn_mut2 中， exec 参数 mut f 接收
 */
fn demo_fn_mut() {
    let mut s = String::new();
    // 闭包内部补货可变借用，需要将该闭包声明为可变类型
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{}", s);
}

fn demo_fn_mut2() {
    let mut s = String::new();
    let update_string = |str| s.push_str(str);
    exec(update_string);
    println!("{}", s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello");
}

/*
 * FnOnce 拿走被捕获变量的所有权
 * 1. fn_once1 调用时会转移所有权， 不能对已失去所有权的闭包变量进行二次调用
 * 2. fn_once2 F: Fn(usize)->bool + copy, 调用时使用的是拷贝，没有发生所有权的转移
 * 3. demo_fn_once2 添加move关键字，强制闭包捕获变量所有权。常用语闭包的生命周期大于捕获变量的生命周期时， 如：
 *    将闭包返回或移入其他线程
 */
fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool,
{
    println!("{}", func(3));
    // println!("{}", func(4)); // func 只能调用一次
}

fn fn_once2<F>(func: F)
where
    F: Fn(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn demo_fn_once() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());

    let x = vec![1, 2, 3];
    fn_once2(|z| z == x.len());
}

fn demo_fn_once2() {
    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
