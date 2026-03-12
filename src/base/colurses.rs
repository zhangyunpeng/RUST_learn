pub fn run() {
    demo6();
}

fn demo() {
    let c = |x: i32| x + 1;
    assert_eq!(3, c(2));

    let add = |x: i32, y: i32| x + y;
    assert_eq!(5, add(3, 2));

    assert_eq!(5, do_add(3, 2, &add));
}

fn do_add(x: i32, y: i32, add: &dyn Fn(i32, i32) -> i32) -> i32 {
    add(x, y)
}

fn demo2() {
    let mut x = 10;
    let mut print = || {
        x += 1;
    };
    print();
    print();
    assert_eq!(12, x);

    let mut x = 10;
    let mut print = move || {
        x += 1;
        println!("{}", x);
    };
    print();
    assert_eq!(10, x);

    let x = "10".to_string();
    let print = move || {
        println!("{}", x);
        x
    };
    print();
    // print(); // Err x 所有权已经转移走, 不能多次调用
    // assert_eq!(10, x); // Err x is move
}

fn demo3() {
    let x = 5;
    let mut c = || x * 2;
    assert_eq!(10, c());
    assert_eq!(10, call_fn(c));
    assert_eq!(10, call_fn_mut(c));
    assert_eq!(10, call_fn_mut(&mut c));
    assert_eq!(10, call_fn_once(c));
    assert_eq!(10, call_fn_once(c));
}

fn call_fn<F: Fn() -> i32>(f: F) -> i32 {
    f()
}

fn call_fn_mut<F: FnMut() -> i32>(mut f: F) -> i32 {
    f()
}

fn call_fn_once<F: FnOnce() -> i32>(f: F) -> i32 {
    f()
}

fn demo4() {
    let multiply = |x| x * x;
    assert_eq!(100, apply(multiply, 10));

    let y = 2;
    let multiply = |x| x * y;
    assert_eq!(20, apply_fn(multiply, 10));

    let mut y = 2;
    let mut multiply = |xx| {
        y *= xx;
        y
    };
    assert_eq!(20, apply_fn_mut(&mut multiply, 10));

    let mut s = "a".to_string();
    let c = |xxx: String| {
        s += xxx.as_str();
        s
    };
    assert_eq!("abc", apply_fn_once(c, "bc".to_string()));
    // assert_eq!("abc", apply_fn_once(c, "bc".to_string())); // Err c 是FnOnce只能调用一次
}

fn apply(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg)
}

fn apply_fn<F: Fn(i32) -> i32>(f: F, arg: i32) -> i32 {
    f(arg)
}

fn apply_fn_mut<F: FnMut(i32) -> i32>(mut f: F, arg: i32) -> i32 {
    f(arg)
}

fn apply_fn_once<F: FnOnce(String) -> String>(f: F, arg: String) -> String {
    f(arg)
}

fn demo5() {
    let y = 10;
    let c = Box::new(move |x| x * y);
    let my_struct = MyStruct { val: 10, action: c };
    assert_eq!(20, (my_struct.action)(2));
}

struct MyStruct {
    val: i32,
    action: Box<dyn Fn(i32) -> i32>,
}

fn demo6() {
    let mut sub_btn = Button::new(
        |button_data| {
            if let ButtonData::Count(sub_count) = button_data {
                *sub_count += 1;
                println!("subscrible!! Total subscription {}", sub_count);
            }
        },
        ButtonData::Count(0),
    );
    sub_btn.click();
    sub_btn.click();
    sub_btn.click();

    let mut send_btn = Button::new(
        |button_data| {
            if let ButtonData::Message(msg) = button_data {
                println!("Your subscrible message {}", msg);
            }
        },
        ButtonData::Message(String::new()),
    );

    send_btn.set_message("Hi, please call me".to_string());
    send_btn.click();
}

enum ButtonData {
    Count(i32),
    Message(String),
}

struct Button<F: Fn(&mut ButtonData)> {
    button_data: ButtonData,
    handler: F,
}

impl<F> Button<F>
where
    F: Fn(&mut ButtonData),
{
    fn new(handler: F, data: ButtonData) -> Self {
        Self {
            button_data: data,
            handler,
        }
    }

    fn click(&mut self) {
        (self.handler)(&mut self.button_data);
    }

    fn set_message(&mut self, message: String) {
        self.button_data = ButtonData::Message(message);
    }
}
