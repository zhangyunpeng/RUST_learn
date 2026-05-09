pub fn run() {
    demo3();
}

fn demo() {
    let mut s = "hello ".to_string();
    let res = extend_string(&mut s, "world!");
    assert_eq!(res, "hello world!");
}

fn extend_string<'a>(origin: &'a mut String, data: &str) -> &'a str {
    origin.push_str(data);
    origin
}

fn demo2() {
    let arr = ["ab", "bcd", "ef"];
    assert_eq!(find_biggest_item(&arr), Some("bcd"));
}

fn find_biggest_item<'a>(s: &'a [&'a str]) -> Option<&'a str> {
    // s.iter().reduce(|max_length_str, current|
    //     if current.len() > max_length_str.len() {
    //         current
    //     } else {
    //         max_length_str
    //     }).copied()

    // s.iter().reduce(|max_length_str, current|
    //     if current.len() > max_length_str.len() {
    //         current
    //     } else {
    //         max_length_str
    //     }).map(|&v|v)

    s.iter().max_by_key(|v| v.len()).copied()

    // s.iter().max_by(|a,b|a.len().cmp(&b.len())).copied()
}

#[derive(Debug)]
struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}

impl<'a, 'b> MyStruct<'a, 'b> {
    fn get_data1(&self) -> &'a str {
        self.data1
    }

    fn set_data(&mut self, data1: &'a str, data2: &'b str) {
        self.data1 = data1;
        self.data2 = data2;
    }

    // fn get_longest<'c>(&'c self, s: &'c str) -> &str { // 返回值的生命周期也可以省略，默认会使用self的生命周期
    fn get_longest<'c>(&'c self, s: &'c str) -> &'c str {
        let longest = if self.data1.len() > self.data2.len() {
            self.data1
        } else {
            self.data2
        };
        if longest.len() > s.len() { longest } else { s }
    }
}

fn demo3() {
    // let data1 = "hello";
    // let data2 = "hello";
    // let my_struct = MyStruct {
    //     data1,
    //     data2,
    // };

    // println!("data1: {}", my_struct.data1);
    // println!("data2: {}", my_struct.data2);
    // let ret = fun(&my_struct);
    // println!("{}", ret);

    // let data1 = "hello";
    // let data2 = "hello";
    // let ret;
    // {
    //     let my_struct = MyStruct {
    //         data1,
    //         data2,
    //     };
    //     ret = my_struct.get_data1();
    // }
    // println!("data1: {}", ret);

    // let data1 = "hello";
    // let data2 = "hello";
    // let mut my_struct = MyStruct {
    //     data1,
    //     data2,
    // };
    // let s1 = "s1";
    // let s2 = "s2";
    // my_struct.set_data(s1, s2);
    // println!("{:?}", my_struct);

    let data1 = "a";
    let data2 = "ab";
    let my_struct = MyStruct { data1, data2 };
    let s = "abc";
    assert_eq!("abc", my_struct.get_longest(s));
}

fn fun<'a>(ms: &MyStruct<'a, '_>) -> &'a str {
    ms.data1
}
