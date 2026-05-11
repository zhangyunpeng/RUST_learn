#[allow(unused)]
mod base;
#[allow(unused)]
mod closure;
#[allow(unused)]
mod closure_fn;
#[allow(unused)]
mod enums;
#[allow(unused)]
mod file_search;
#[allow(unused)]
mod hashmaps;
#[allow(unused)]
mod lifetime;
#[allow(unused)]
mod traits;
#[allow(unused)]
mod vecs;

pub fn run() {
    println!("Running sj start======");
    // enums::run();
    // base::run();
    // traits::run();
    // vecs::run();
    // hashmaps::run();
    // closure::run();
    closure_fn::run();
    println!("Running sj end======");
}
