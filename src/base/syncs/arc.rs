use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use std::thread;
use std::time::Duration;

struct Gadget {
    me: Weak<Gadget>,
}
pub fn demo() {
    let mut data = Arc::new(vec![1, 2, 3]);
    Arc::make_mut(&mut data).push(4);
    assert_eq!(*data, vec![1, 2, 3, 4]);

    // let five = Arc::new(5);
    // for _i in 0..2 {
    //     let val = five.clone();
    //     thread::spawn(move || {
    //         println!("{:?}", val);
    //     });
    // }

    let val = Arc::new(AtomicUsize::new(5));
    for _ in 0..5 {
        let val = val.clone();
        thread::spawn(move || {
            let v = val.fetch_add(1, Ordering::Relaxed);
            println!("{:?}", v);
        });
    }
    thread::sleep(Duration::from_secs(2));
    println!("ato {:?}", val.load(Ordering::Relaxed));
}

impl Gadget {
    fn new() -> Arc<Gadget> {
        Arc::new_cyclic(|me| Gadget { me: me.clone() })
    }

    fn me(&self) -> Arc<Self> {
        self.me.upgrade().unwrap()
    }
}
