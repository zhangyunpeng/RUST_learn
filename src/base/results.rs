pub fn run() {
    demo2();
}

fn demo2() {
    let x = Some("a");
    assert_eq!(x.ok_or(0), Ok("a"));
    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));
}

fn demo1() {
    let s1 = "ab";
    let s2 = "cd";
    let add_res = add_string(s1, s2);
    match add_res {
        Ok(s) => println!("{}", s),
        Err(e) => println!("err: {}", e.display()),
    }

    let s1 = "ab";
    let s2 = "";
    let add_res = add_string(s1, s2);
    match add_res {
        Ok(s) => println!("{}", s),
        Err(e) => println!("err: {}", e.display()),
    };

    let s1 = "ab";
    let s2 = "cde";
    let add_res = add_string(s1, s2);
    match add_res {
        Ok(s) => println!("{}", s),
        Err(e) => println!("err: {}", e.display()),
    };
}

enum AddStringError {
    Empty,
    LengthMisMatch,
}

impl AddStringError {
    fn display(&self) -> String {
        match self {
            Self::Empty => "Empty string".to_string(),
            Self::LengthMisMatch => "String length dis match".to_string(),
        }
    }
}

fn add_string(s1: &str, s2: &str) -> Result<String, AddStringError> {
    if s1.is_empty() || s2.is_empty() {
        return Err(AddStringError::Empty);
    }
    if s1.len() != s2.len() {
        return Err(AddStringError::LengthMisMatch);
    }
    Ok(format!("{}{}", s1, s2))
}

fn demo() {
    let x: Result<i32, &str> = Ok(10);
    assert!(x.is_ok());
    let x: Result<i32, &str> = Err("err");
    assert!(x.is_err());

    let x: Result<i32, &str> = Ok(10);
    assert!(x.is_ok_and(|x| x > 1));
    assert!(!x.is_ok_and(|x| x > 10));
}
