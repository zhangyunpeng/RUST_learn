use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

pub struct Delay {
    name: String,
    when: Instant,
    // 已创建计时线程则为 Some， 否则为 None
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Delay {
    pub fn new(dur: Duration, s: &str) -> Self {
        Delay {
            name: s.to_string(),
            when: Instant::now() + dur,
            waker: None,
        }
    }
}

impl Future for Delay {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Ready: {}", &self.name);
            return Poll::Ready(());
        }

        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            // 首次poll，创建计时线程并保存Waker
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            thread::spawn(move || {
                let now = Instant::now();
                if when > now {
                    thread::sleep(when - now);
                }
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }
        Poll::Pending
    }
}
