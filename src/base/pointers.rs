use std::{cell::RefCell, rc::Rc};

pub fn run() {
    demo_ref_cell();
}

fn demo_ref_cell() {
    let data = Rc::new(RefCell::new(100));
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);
    *owner1.borrow_mut() += 10;
    *owner2.borrow_mut() += 10;
    println!("{:?}", data);
}

fn demo_rc2() {
    let mut strong_ref_1 = Rc::new("hello".to_string());
    println!("strong: {:?}", Rc::strong_count(&strong_ref_1));
    println!("weak: {:?}", Rc::weak_count(&strong_ref_1));

    let rc_mut = Rc::get_mut(&mut strong_ref_1);
    if let Some(s) = rc_mut {
        *s += " world!";
    }
    println!("{:?}", strong_ref_1);
}
fn demo_rc() {
    let strong_ref_1 = Rc::new("hello".to_string());
    println!("strong: {:?}", Rc::strong_count(&strong_ref_1));
    println!("weak: {:?}", Rc::weak_count(&strong_ref_1));

    let strong_ref_2 = Rc::clone(&strong_ref_1);
    println!("strong: {:?}", Rc::strong_count(&strong_ref_2));
    println!("weak: {:?}", Rc::weak_count(&strong_ref_2));

    let _weak_ref_1 = Rc::downgrade(&strong_ref_2);
    let weak_ref_2 = Rc::downgrade(&strong_ref_2);
    println!("strong: {:?}", Rc::strong_count(&strong_ref_2));
    println!("weak: {:?}", Rc::weak_count(&strong_ref_2));

    let _strong_ref_3 = weak_ref_2.upgrade();
    println!("strong: {:?}", Rc::strong_count(&strong_ref_2));
    println!("weak: {:?}", Rc::weak_count(&strong_ref_2));
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn demo5() {
    let n2 = Node { val: 2, next: None };
    let n1 = Node {
        val: 1,
        next: Some(Box::new(n2)),
    };
    println!("{:?}", n1);
}

fn demo4() {
    let mut box_i32 = Box::new(10);
    let immutable_raw_ptr = &(*box_i32);
    println!("{:?}", *immutable_raw_ptr);
    let mutable_raw_ptr = &mut (*box_i32);
    *mutable_raw_ptr += 10;
    println!("{:?}", *mutable_raw_ptr);
}

fn demo3() {
    let v = vec![10; 5];
    let bv = v.into_boxed_slice();
    println!("{:?}", bv);
}

fn demo() {
    let mut number: i32 = 100;
    let raw: *mut i32 = &mut number as *mut i32;
    unsafe {
        *raw += 100;
    }
    assert_eq!(200, number);
}

fn demo1() {
    let v: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    v.iter().for_each(|shape| shape.draw());
}

fn demo2() {
    let box_int = Box::new(13);
    let raw_ptr_to_int = Box::into_raw(box_int);
    let box_int: Box<i32>;
    unsafe {
        box_int = Box::from_raw(raw_ptr_to_int);
    }
    println!("{:?}", box_int);
}

trait Shape {
    fn draw(&self);
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Circle radius: {}", self.radius);
    }
}

#[derive(Debug)]
struct Square {
    side: f64,
}

impl Shape for Square {
    fn draw(&self) {
        println!("Square side: {}", self.side);
    }
}
