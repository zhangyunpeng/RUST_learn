use std::{f64::consts::PI, fmt::Display, ops::Mul};



pub fn run() {
    demo2();
}

fn demo() {
    let dog = Dog {
        weight: 30,
        age: 5,
        name: "阿彪".to_string(),
    };
    println!("{}", dog);
    println!("{:15}", dog);
}

struct Dog {
    weight: u8,
    age: u8,
    name: String,
}

impl Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut w: usize = 8;
        if let Some(width) = f.width() {
            w = width
        } 
        writeln!(f, "{:#^w$}", "Dog details", w = w * 2 + 1)?;
        writeln!(f, "{:<w$}:{:<w$}", "Weight", self.weight)?;
        writeln!(f, "{:<w$}:{:<w$}", "Age", self.age)?;
        writeln!(f, "{:<w$}:{:<w$}", "Name", self.name)?;
        Ok(())
    }
}

trait Shape {
    type Item;
    fn area(&self) -> Self::Item;
}

struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Shape for Rectangle<T> 
    where T: Mul<Output = T> + Copy
{
    type Item = T;
    fn area(&self) -> Self::Item {
        self.width * self.height
    }
}

struct Circle<T> {
    radius: T
}

impl<T> Shape for Circle<T>
    where T: Mul<Output = T> +  Copy + From<f32> + From<f64>
{
    type Item = T;
    fn area(&self) -> Self::Item {
        let pi: T = PI.into();
        pi * self.radius * self.radius
    }
}

fn exec_area<T>(s: &dyn Shape<Item = T>) -> T {
    s.area()
}

fn demo2() {
    let r = Rectangle{
        width: 10,
        height: 20,
    };
    println!("{:?}", r.area());
    let c = Circle {
        radius: 2.0,
    };
    println!("{:?}", c.area());
    println!("{:?}", exec_area(&r));
    println!("{:?}", exec_area(&c));

    
}

