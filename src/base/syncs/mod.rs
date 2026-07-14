mod arc;

mod condvar;
mod mpsc;

pub fn run() {
    // arc::demo();
    // condvar::demo_wait();
    // condvar::demo_wait_while();
    // condvar::demo_wait_timeout();
    mpsc::demo2();
}
