use learn::base::runtime::{mini_tokio_v2::MiniTokio, self_futures::delay_v2::Delay};
use std::time::Duration;

fn main() {
    let mini = MiniTokio::default();
    let a = Delay::new(Duration::from_millis(1), "a");
    let b = Delay::new(Duration::from_millis(2), "b");
    let c = Delay::new(Duration::from_millis(3), "c");
    let d = Delay::new(Duration::from_millis(4), "d");
    mini.spawn(a);
    mini.spawn(b);
    mini.spawn(c);
    mini.spawn(d);
    mini.run();
}
