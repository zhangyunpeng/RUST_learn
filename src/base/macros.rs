macro_rules! hello {
    () => {
        println!("hello world!");
    };
    ($name:literal) => {
        println!("hello {}", $name);
    };
    // ($name1:literal, $name2:literal) => {
    //     println!("hello, first name:{}, second name: {}", $name1, $name2);
    // };
    ($($name:literal), +) => {
        $(
            println!("hello {}", $name);
        )+
    };

    ($($name:expr), +) => {
        $(
            println!("hello {}", $name);
        )+
    };
}

pub fn run() {
    hello!("a");
    hello!("zhangyunpeng", "sunshine");
    hello!(1+2, 3+4);
}


