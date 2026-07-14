use std::sync::mpsc;
use std::thread;

pub fn demo() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });
    for i in rx.iter() {
        println!("Got: {}", i);
    }
}

pub fn demo2() {
    let (tx, rx) = mpsc::sync_channel(0);
    thread::spawn(move || {
        tx.send(1).unwrap();
    });
    assert_eq!(rx.recv().unwrap(), 1);
}
