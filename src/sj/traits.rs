pub fn run() {
    demo1();
    demo2();
}

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}
fn demo1() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string(),
    };

    // 填空
    println!("{}", comp_sci_student_greeting(&student));
}

struct Pretty(String);

impl std::fmt::Display for Pretty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + " world")
    }
}

fn demo2() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
