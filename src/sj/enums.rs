use crate::sj::enums::List::Cons;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Self::Nil
    }

    fn prepend(self, elem: i32) -> Self {
        Self::Cons(elem, Box::new(self))
    }

    fn len(&self) -> i32 {
        match *self {
            Self::Cons(_, ref tail) => 1 + tail.len(),
            Self::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Self::Nil => "Nil".to_string(),
            Self::Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
        }
    }
}

pub fn run() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());
}