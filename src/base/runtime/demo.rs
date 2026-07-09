use std::collections::VecDeque; //  双端队列 任务调度 FIFO
use std::future::Future; //  异步 Future trait
use std::pin::Pin; //  固定指针，异步核心，防止 Future 内存移动
use std::sync::{Arc, Mutex}; //  线程安全共享、互斥锁
use std::task::{Context, Poll, Wake, Waker}; //  轮询、唤醒相关API

// Task：可跨线程发送、静态生命周期、无返回值的异步任务
type Task = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// 全局共享任务队列（线程安全）
#[derive(Default)]
struct SharedQueue {
    queue: Mutex<VecDeque<Task>>,
}

/// 运行时主结构体
#[derive(Clone, Default)]
pub struct Runtime {
    shared: Arc<SharedQueue>,
}

impl Runtime {
    /// 生成绑定当前运行时的Waker
    fn make_waker(&self) -> Waker {
        let state = Arc::new(WakerState {
            queue: self.shared.clone(),
        });
        Waker::from(state)
    }

    /// 提交异步任务到调度队列
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.shared.queue.lock().unwrap().push_back(Box::pin(fut));
    }

    /// 循环poll所有就绪任务
    pub fn run(&self) {
        loop {
            // 加锁取出任务
            let mut guard = self.shared.queue.lock().unwrap();
            if guard.is_empty() {
                break;
            }
            let task = guard.pop_front().unwrap();
            // 立刻释放锁，避免长时间持有阻塞wake
            drop(guard);

            let waker = self.make_waker();
            let mut cx = Context::from_waker(&waker);
            let mut task = task;

            match task.as_mut().poll(&mut cx) {
                Poll::Ready(_) => {
                    // 任务执行完成，直接丢弃
                }
                Poll::Pending => {
                    // 未就绪，放回队列等待下次唤醒
                    self.shared.queue.lock().unwrap().push_back(task);
                }
            }
        }
    }
}

/// Waker 内部状态，持有全局任务队列Arc
struct WakerState {
    queue: Arc<SharedQueue>,
}

/// 实现标准 Wake trait，要求 self: Arc<Self>
impl Wake for WakerState {
    fn wake(self: Arc<Self>) {
        // 极简唤醒：本demo仅轮询，真实runtime会单独缓存唤醒任务
        // 如需优化：增加单独唤醒队列，wake时写入，run优先消费唤醒队列
    }
}

/// block_on：同步阻塞执行任意Future，线程安全
pub fn block_on<F, T>(fut: F) -> T
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let rt = Runtime::default();
    // 线程安全存储返回结果
    let output = Arc::new(Mutex::new(None));
    let output_clone = output.clone();

    rt.spawn(async move {
        let res = fut.await;
        *output_clone.lock().unwrap() = Some(res);
    });

    // 启动调度循环
    rt.run();

    // 取出结果并返回
    output.lock().unwrap().take().unwrap()
}

// ====================== 自定义异步Sleep Future（模拟IO） ======================
struct SleepFuture {
    remain: u64,
}

impl SleepFuture {
    fn new(ms: u64) -> Self {
        Self { remain: ms }
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remain == 0 {
            return Poll::Ready(());
        }
        self.remain -= 1;
        println!("Sleep pending, remain: {}ms", self.remain);
        Poll::Pending
    }
}

// 语法糖封装
pub async fn sleep(ms: u64) {
    SleepFuture::new(ms).await
}
