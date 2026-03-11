use std::{
    fs,
    io::{self, Write},
    num::IntErrorKind,
};

pub fn run() {
    demo2();
}

fn demo() {
    let res = rename("log.txt", "io.txt");
    if let Err(e) = res {
        println!("{}", e);
        println!("err kind: {}", e.kind());
    }
}

fn rename(from: &str, to: &str) -> io::Result<()> {
    fs::rename(from, to)?;
    Ok(())
}

fn demo2() {
    print!("Please enter your number: ");
    io::stdout().flush().unwrap();
    let res = get_user_input();
    if let Err(e) = res {
        println!("err: {}", e);
        return;
    }

    let input = res.unwrap();
    let res = parse_integer_from_str_v2(&input);
    match res {
        Ok(num) => println!("Your number: {}", num),
        Err(_) => println!("Your input : {} no number", &input),
    }
}

fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn parse_integer_from_str(s: &str) -> Result<i32, String> {
    let res = s.parse::<i32>();
    match res {
        Ok(num) => Ok(num),
        Err(e) => match e.kind() {
            IntErrorKind::Empty => Err("String Empty".to_string()),
            _ => Err(format!("{:?}", e)),
        },
    }
}

fn parse_integer_from_str_v2(s: &str) -> io::Result<i32> {
    // let res = s.parse::<i32>();
    // match res {
    //     Ok(num) => Ok(num),
    //     Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "invalidate"))
    // }
    s.parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))
}
