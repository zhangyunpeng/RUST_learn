use futures_util::task::ArcWake;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

// ===================== MiniTokio Runtime 不变 =====================
fn main() {
    let mini = MiniTokio::new();
    for _ in 0..2 {
        mini.spawn(async {
            delay_ms(100).await;
        });
    }
    mini.run();
}

struct MiniTokio {
    scheduled: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}

struct TaskFuture {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    poll: Poll<()>,
}

struct Task {
    task_future: Mutex<TaskFuture>,
    executor: mpsc::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl TaskFuture {
    fn new(future: impl Future<Output = ()> + Send + 'static) -> Self {
        Self {
            future: Box::pin(future),
            poll: Poll::Pending,
        }
    }

    fn poll(&mut self, cx: &mut Context<'_>) {
        if self.poll.is_pending() {
            self.poll = self.future.as_mut().poll(cx);
        }
    }
}

impl Task {
    fn poll(self: Arc<Self>) {
        let waker = futures_util::task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut task_future = self.task_future.try_lock().unwrap();
        task_future.poll(&mut cx);
    }

    fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Self {
            task_future: Mutex::new(TaskFuture::new(future)),
            executor: sender.clone(),
        });
        let _ = sender.send(task);
    }
}

impl MiniTokio {
    fn new() -> Self {
        let (s, r) = mpsc::channel();
        Self {
            scheduled: r,
            sender: s,
        }
    }

    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

// ===================== 全局定时器（修复核心） =====================
struct TimerEntry {
    deadline: Instant,
    waker: Waker,
}

impl PartialEq for TimerEntry {
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline
    }
}
impl Eq for TimerEntry {}
impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TimerEntry {
    // Reverse 实现小根堆，最早到期先出堆
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.deadline).cmp(&Reverse(other.deadline))
    }
}

#[derive(Clone)]
struct GlobalTimer {
    inner: Arc<Mutex<BinaryHeap<TimerEntry>>>,
    cond: Arc<Condvar>,
}

impl GlobalTimer {
    fn new() -> Self {
        let heap = Arc::new(Mutex::new(BinaryHeap::<TimerEntry>::new()));
        let cond = Arc::new(Condvar::new());
        let heap_clone = heap.clone();
        let cond_clone = cond.clone();

        // 仅启动1个全局计时线程，全程复用，不再无限spawn
        std::thread::spawn(move || {
            loop {
                let mut queue = heap_clone.lock().unwrap();
                let now = Instant::now();

                // 唤醒所有到期任务
                while let Some(top) = queue.peek() {
                    if top.deadline <= now {
                        let entry = queue.pop().unwrap();
                        entry.waker.wake();
                    } else {
                        let wait = top.deadline - now;
                        queue = cond_clone.wait_timeout(queue, wait).unwrap().0;
                        break;
                    }
                }
            }
        });

        Self { inner: heap, cond }
    }

    fn add_delay(&self, deadline: Instant, waker: Waker) {
        self.inner
            .lock()
            .unwrap()
            .push(TimerEntry { deadline, waker });
        self.cond.notify_one();
    }
}

// 全局单例定时器
static GLOBAL_TIMER: std::sync::OnceLock<GlobalTimer> = std::sync::OnceLock::new();
fn get_timer() -> &'static GlobalTimer {
    GLOBAL_TIMER.get_or_init(GlobalTimer::new)
}

struct Delay {
    deadline: Instant,
}

impl Future for Delay {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let now = Instant::now();
        if now >= self.deadline {
            println!("ready");
            return Poll::Ready(());
        }
        // 只注册waker，不新建线程
        get_timer().add_delay(self.deadline, cx.waker().clone());
        Poll::Pending
    }
}

// 辅助延迟函数
async fn delay_ms(ms: u64) {
    Delay {
        deadline: Instant::now() + Duration::from_millis(ms),
    }
    .await
}
