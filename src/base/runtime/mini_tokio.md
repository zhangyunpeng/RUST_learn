# 完整代码逐模块深度解析

这份代码是**极简版 Tokio 异步运行时**，完整实现三大核心组件：

1. 单线程异步调度器 `MiniTokio`（基于 mpsc 任务队列 + `ArcWake` 唤醒机制）
2. 全局单例定时器（独立后台计时线程 + BinaryHeap 最小堆管理延时任务）
3. `Delay` 延时 Future（异步睡眠，不阻塞运行时线程）

整体架构：用户提交异步任务 → 运行时循环 poll 任务 → 任务执行到`delay_ms`时注册定时器 waker → 定时器线程检测到时间到期唤醒任务 → 运行时重新 poll 完成任务。

## 一、前置依赖与导入说明

```
use std::cmp::Reverse;
use std::collections::BinaryHeap; // 定时器最小堆
use std::pin::Pin; // 处理自引用Future
use std::sync::{Arc, Condvar, Mutex, mpsc}; // 线程同步、任务通道
use std::task::{Context, Poll, Waker}; // Rust异步核心抽象
use std::time::{Duration, Instant}; // 时间戳
use futures_util::task::ArcWake; // 把Arc包装成Waker唤醒器
```

关键概念提前铺垫：

- `Poll<T>`：Future 轮询结果，`Pending`未完成、`Ready(T)`完成
- `Waker`：唤醒器，当 Future 可继续执行时通知运行时重新 poll
- `ArcWake`：trait，允许把`Arc<Self>`转换成`Waker`，实现任务自唤醒
- `Pin<Box<dyn Future>>`：堆上存储任意 Future，禁止移动（自引用安全）
- `BinaryHeap`：默认大根堆，配合`Reverse`实现**时间最小堆**（最早到期先执行）
- `Condvar`：条件变量，定时器线程无到期任务时阻塞等待，避免空轮询

# 二、MiniTokio 异步运行时模块（调度核心）

## 2.1 顶层入口 main

```
fn main() {
    let mini = MiniTokio::new();
    // 提交2个延时100ms的异步任务
    for _ in 0..2 {
        mini.spawn(async {
            delay_ms(100).await;
        });
    }
    mini.run(); // 启动调度循环
}
```

流程：创建运行时 → 生成 2 个延时 Future → 推入调度队列 → 阻塞运行时循环处理所有任务。

## 2.2 MiniTokio 运行时结构体

```
struct MiniTokio {
    scheduled: mpsc::Receiver<Arc<Task>>, // 接收待poll任务
    sender: mpsc::Sender<Arc<Task>>,      // 提交新任务/唤醒后重入任务
}
```

- `mpsc`多生产者单消费者通道：
    - `spawn`新建任务发送到通道
    - 任务被唤醒时再次 send 入通道，等待重新 poll
    - `run`循环 recv 取出任务执行

### 构造与 spawn

```
impl MiniTokio {
    fn new() -> Self {
        let (s, r) = mpsc::channel();
        Self { scheduled: r, sender: s }
    }

    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }
```

`spawn`对外暴露接口，转发给`Task`创建任务并投递到通道。

### 调度主循环 run

```
    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}
```

单线程调度循环：阻塞等待通道任务，拿到后调用`task.poll()`轮询 Future。

>
> 限制：当前是**单线程运行时**，所有任务都在主线程 poll，无多工作线程。

## 2.3 Task & TaskFuture：封装异步任务载体

### TaskFuture：存储 Future 与轮询状态

```
struct TaskFuture {
    future: Pin<Box<dyn Future<Output = ()> + Send>>, // 堆上任意异步任务
    poll: Poll<()>, // 缓存当前轮询状态
}
```

- `Pin<Box<dyn Future>>`：
    1. `Box`：把 Future 放到堆，生命周期脱离栈
    2. `Pin`：禁止移动，防止 async/await 自引用结构体悬空
    3. `dyn Future`：支持任意异步闭包 / 函数
- `poll`字段缓存轮询结果，避免重复 poll 已完成的 Future

#### 方法实现

```
impl TaskFuture {
    fn new(future: impl Future<Output = ()> + Send + 'static) -> Self {
        Self {
            future: Box::pin(future),
            poll: Poll::Pending, // 初始未完成
        }
    }

    fn poll(&mut self, cx: &mut Context<'_>) {
        // 仅当任务未完成时才轮询
        if self.poll.is_pending() {
            self.poll = self.future.as_mut().poll(cx);
        }
    }
}
```

核心优化：如果 Future 已经返回`Ready`，不再重复调用`poll`，避免无效操作。

### Task：带同步锁、调度通道的任务句柄

```
struct Task {
    task_future: Mutex<TaskFuture>, // 多线程同步保护Future状态
    executor: mpsc::Sender<Arc<Task>>, // 唤醒时投递回调度队列
}
```

- `Mutex`：定时器线程、运行时主线程都会访问任务状态，必须互斥锁同步
- `executor`：保存通道发送端，唤醒时把自身发回调度队列等待轮询

#### 调度方法 schedule

```
impl Task {
    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }
}
```

把`Arc<Task>`克隆一份发送到任务通道，运行时主线程会收到并重新 poll。发送失败直接忽略（通道关闭场景）。

#### 实现 ArcWake：转换 Waker 唤醒器

```
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}
```

`ArcWake`是 futures 工具库 trait，作用：
只要类型实现该 trait，就能通过`futures_util::task::waker(Arc<Task>)`生成标准`Waker`。
当定时器调用`waker.wake()`时，底层触发`wake_by_ref`，调用`schedule()`把任务重新送入调度队列。

#### poll 轮询任务逻辑

```
impl Task {
    fn poll(self: Arc<Self>) {
        // 1. 根据当前Task Arc生成Waker
        let waker = futures_util::task::waker(self.clone());
        // 2. 构造轮询上下文Context（携带唤醒器）
        let mut cx = Context::from_waker(&waker);
        // 3. 获取任务锁，访问Future
        let mut task_future = self.task_future.try_lock().unwrap();
        // 4. 执行Future轮询
        task_future.poll(&mut cx);
    }
```

`Context`是 Future 轮询的环境，核心只有`Waker`，Future 阻塞时会保存这个 waker，外部就绪后调用 wake 触发重调度。

#### 静态方法 spawn 创建任务

```
    fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Self {
            task_future: Mutex::new(TaskFuture::new(future)),
            executor: sender.clone(),
        });
        // 新建任务立即投递到调度队列，第一次poll
        let _ = sender.send(task);
    }
}
```

新建`Arc<Task>`，立刻发送到 mpsc 通道，运行时主线程马上取出执行第一次 poll。

# 三、全局定时器模块（延时 await 核心）

## 3.1 TimerEntry：堆中存储的延时条目

```
struct TimerEntry {
    deadline: Instant, // 到期时间戳
    waker: Waker,      // 时间到后需要唤醒的任务唤醒器
}
```

每条延时对应一个到期时间 + 任务 waker，定时器线程到期后调用`waker.wake()`。

### 实现堆排序 Trait（最小堆）

`BinaryHeap`默认是大根堆，我们需要**最早到期优先弹出**，因此用`Reverse`反转排序：

```
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.deadline).cmp(&Reverse(other.deadline))
    }
}
```

- `Reverse(Instant)`反转大小关系：更小的时间戳会被判定为更大，堆顶是最小时间（最早到期）
- 完整实现`Ord/Eq`是`BinaryHeap`的强制要求。

## 3.2 GlobalTimer 全局定时器

```
#[derive(Clone)]
struct GlobalTimer {
    inner: Arc<Mutex<BinaryHeap<TimerEntry>>>, // 延时任务堆，多线程共享
    cond: Arc<Condvar>,                       // 条件变量，阻塞计时线程
}
```

- `Arc<Mutex<BinaryHeap>>`：多线程安全的延时任务队列
- `Condvar`：计时线程无到期任务时阻塞，有新延时任务加入时唤醒线程重新检查堆

### new ()：启动后台计时线程（单全局线程）

```
impl GlobalTimer {
    fn new() -> Self {
        let heap = Arc::new(Mutex::new(BinaryHeap::<TimerEntry>::new()));
        let cond = Arc::new(Condvar::new());
        let heap_clone = heap.clone();
        let cond_clone = cond.clone();

        // 全局唯一计时线程，永久循环
        std::thread::spawn(move || loop {
            let mut queue = heap_clone.lock().unwrap();
            let now = Instant::now();

            // 循环弹出所有已到期任务
            while let Some(top) = queue.peek() {
                if top.deadline <= now {
                    let entry = queue.pop().unwrap();
                    entry.waker.wake(); // 唤醒对应的异步任务
                } else {
                    // 最近任务还没到时间，阻塞等待剩余时长
                    let wait = top.deadline - now;
                    // wait_timeout：阻塞，超时/被notify后解锁堆
                    queue = cond_clone.wait_timeout(queue, wait).unwrap().0;
                    break;
                }
            }
        });

        Self { inner: heap, cond }
    }
```

计时线程核心逻辑：

1. 锁定堆，获取当前时间
2. 循环检查堆顶：如果到期，弹出并执行`waker.wake()`唤醒任务
3. 堆顶未到期：计算等待时长，`wait_timeout`阻塞线程释放锁
    - 阻塞期间其他线程可以添加延时任务
    - 添加任务时调用`notify_one()`唤醒线程重新检查堆
4. 无限循环，全程只启动**一个后台线程**，不会随 delay 无限创建线程

### add\_delay：注册延时任务

```
    fn add_delay(&self, deadline: Instant, waker: Waker) {
        self.inner.lock().unwrap().push(TimerEntry { deadline, waker });
        self.cond.notify_one(); // 唤醒计时线程重新扫描堆
    }
}
```

当`Delay` Future 被 poll 且未到期时，调用此方法把当前任务 waker 存入堆，并通知计时线程。

## 3.3 全局单例定时器 OnceLock

```
static GLOBAL_TIMER: std::sync::OnceLock<GlobalTimer> = std::sync::OnceLock::new();
fn get_timer() -> &'static GlobalTimer {
    GLOBAL_TIMER.get_or_init(GlobalTimer::new)
}
```

`OnceLock`静态单例：全局仅初始化一次定时器，保证整个程序只有一条计时线程，无重复创建开销。

## 3.4 Delay 延时 Future（async/.await 底层载体）

```
struct Delay {
    deadline: Instant, // 目标到期时间
}

impl Future for Delay {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let now = Instant::now();
        if now >= self.deadline {
            println!("ready");
            return Poll::Ready(()); // 时间到，Future完成
        }
        // 未到期：把当前任务waker注册到全局定时器堆
        get_timer().add_delay(self.deadline, cx.waker().clone());
        Poll::Pending // 返回未完成，运行时暂停该任务
    }
}
```

Future 标准实现逻辑：

1. 检查当前时间是否超过截止时间，是则返回`Ready`完成
2. 未到期：复制当前上下文的 waker，交给定时器保管
3. 返回`Pending`，运行时会暂停该任务，去处理其他就绪任务

关键点：**不会阻塞运行时主线程**，只是注册 waker，运行时可以并发处理其他任务。

## 3.5 工具函数 delay\_ms

```
async fn delay_ms(ms: u64) {
    Delay {
        deadline: Instant::now() + Duration::from_millis(ms),
    }.await
}
```

语法糖，创建`Delay`实例并 await，对外提供友好的毫秒延时接口。

# 四、完整执行流程（以 main 两个任务为例）

1. `mini.spawn`创建 2 个`Arc<Task>`，发送到 mpsc 通道
2. `mini.run()`循环 recv，取出第一个 Task 执行`task.poll()`
3. 生成 Waker+Context，poll 任务内部`delay_ms(100).await`
4. 进入`Delay::poll`，当前时间未到截止时间，将 waker 存入全局定时器堆，返回`Pending`
5. 运行时循环取出第二个 Task，重复步骤 3-4，第二个 waker 入堆
6. 定时器后台线程锁定堆，堆顶是 100ms 后到期的任务，阻塞等待 100ms
7. 100ms 后线程唤醒，弹出第一个 TimerEntry，调用`waker.wake()`
8. `wake`触发`Task::schedule`，把第一个任务重新发送到 mpsc 调度通道
9. 运行时主线程收到任务，再次执行`task.poll()`
10. 再次 poll`Delay`，当前时间≥deadline，返回`Ready`，任务完成
11. 定时器线程继续处理第二个到期任务，重复 7-10，第二个任务完成
12. mpsc 通道无更多任务，`recv()`返回 Err，run 循环退出，程序结束

# 五、代码核心亮点与设计思想

## 亮点 1：标准 Rust 异步模型严格对齐 Tokio 底层

完全遵循 Rust 官方`Future/Poll/Waker`异步规范，没有黑魔法：

- 运行时 = 任务调度循环 + mpsc 任务队列
- 阻塞 IO / 延时 = 注册 waker 到后台线程，不阻塞工作线程
- 唤醒机制 = `ArcWake` + `Waker`，任务就绪后重新入队 poll

## 亮点 2：定时器高效设计

1. 全局单一线程，不会产生线程爆炸
2. BinaryHeap 最小堆，只处理最早到期任务
3. Condvar 阻塞等待，无 CPU 空轮询，低功耗
4. 复用 waker 机制，完全融入 Rust 异步生态

## 亮点 3：内存安全、线程安全

- 跨线程共享资源全部`Arc`+ 同步原语（Mutex/Condvar/mpsc）
- Future 使用`Pin<Box>`保证自引用安全
- 所有 Future 约束`Send`，支持多线程环境扩展（当前运行时单线程，可轻松扩展多工作线程）

# 六、代码缺陷与可优化点

## 1. 运行时单线程，无法利用多核

`run()`是单循环，所有任务串行 poll，CPU 密集任务会阻塞整个运行时。
优化方向：多工作线程池，mpsc 任务分发到多个 worker。

## 2. TimerEntry 无任务取消机制

如果任务被丢弃（如提前返回），堆中残留无效 waker，到期唤醒后 poll 发现任务已完成，造成少量无效开销。
优化：增加唯一标识，poll 时过滤过期任务，或使用弱引用 waker。

## 3. Mutex 粗粒度锁

定时器堆全局一把锁，大量延时任务并发添加时会锁竞争。
优化：分层堆、分片定时器。

## 4. 无任务调度优先级、无 IO 驱动

仅支持 sleep 延时，不支持文件 / 网络异步 IO，真实 Tokio 搭配 epoll/io\_uring。

## 5. mpsc 通道发送忽略错误

`let _ = sender.send(...)`，运行时关闭后无日志提示，生产环境应处理错误。

## 6. TaskFuture 一旦 Pending 就永久缓存状态

任务完成后不会从队列清理，仅跳过 poll，长期大量任务会占用内存。

# 七、关键概念对应 Tokio 源码

1. `MiniTokio` ≈ tokio::runtime::Runtime 基础调度层
2. `Task` ≈ tokio::task::JoinHandle / 内部 Task 结构体
3. `GlobalTimer` ≈ tokio::time::driver 全局时间驱动
4. `Delay` ≈ tokio::time::Sleep
5. `ArcWake + Waker` ≈ Tokio 内部唤醒机制
6. BinaryHeap 定时器堆 ≈ Tokio 时间轮（简化版最小堆实现）

代码中各个模块的具体功能是什么？

代码中使用了哪些设计模式？

如何使用这段代码？