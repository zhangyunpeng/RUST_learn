use learn::base::runtime::mini_tokio::MiniTokio;
use learn::base::runtime::self_futures::delay::Delay;
use std::time::Duration;

// #[tokio::main]
// async fn main() {
//     let d = Delay::new(Duration::from_millis(100));
//     let out = d.await;
//     println!("{:?}", out);
// }

fn main() {
    let mut mini = MiniTokio::default();
    mini.spawn(Delay::new(Duration::from_millis(100)));
    mini.spawn(Delay::new(Duration::from_millis(10)));
    mini.spawn(Delay::new(Duration::from_millis(1)));
    mini.run();
}
