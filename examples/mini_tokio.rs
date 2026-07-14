use learn::base::runtime::mini_tokio::{Delay, MiniTokio};
use std::time::Duration;

// #[tokio::main]
// async fn main() {
//     let d = Delay::new(Duration::from_millis(100));
//     let out = d.await;
//     println!("{:?}", out);
// }

fn main() {
    let mut mini = MiniTokio::new();
    mini.spawn(Delay::new(Duration::from_millis(100)));
    mini.spawn(Delay::new(Duration::from_millis(10)));
    mini.spawn(Delay::new(Duration::from_millis(1)));
    mini.run();
}
