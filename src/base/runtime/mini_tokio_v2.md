# 完整代码解析：带标准 Waker 唤醒机制的多线程安全 MiniTokio

这份代码是**具备真实唤醒逻辑、多线程安全、基于 mpsc 通道调度**的极简异步运行时，修复了你前面两段代码的核心缺陷（noop 空唤醒、单线程独占、无脑忙轮询），完整对标 Tokio 底层调度模型：

- 使用 `mpsc` 多生产者单消费者通道做任务调度队列；
- 依靠 `ArcWake` 实现标准唤醒逻辑，任务阻塞后由 IO / 定时器主动唤醒重新入队，不再无限忙轮询；
- 全结构 `Arc+Mutex` 实现多线程安全，任意线程都可`spawn`提交任务；
- 区分「调度通道」「任务存储容器」「Waker 唤醒回调」三层结构。

## 一、导入包逐行说明

```
use futures_util::task::{self, ArcWake};
// ArcWake：标准唤醒trait，实现后可通过 task::waker(Arc<Self>) 生成合法Waker
// task::waker：把实现ArcWake的Arc包装成标准库Waker句柄

use std::pin::Pin;
// 固定指针，约束Future内存不可移动，poll API强制要求

use std::sync::{Arc, Mutex, mpsc};
// Arc：多线程共享所有权智能指针
// Mutex：互斥锁，保护任务内部Future，保证同一时间仅一个线程poll
// mpsc::channel：多生产者单消费者线程安全通道，作为全局调度队列

use std::task::{Context, Poll};
// Context：轮询上下文，承载Waker
// Poll：轮询结果枚举 Poll::Ready / Poll::Pending
```

## 二、顶层运行时结构体 MiniTokio

```
pub struct MiniTokio {
    scheduled: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}
```

### 字段作用

1. `sender`：通道发送端（多生产者）
   任意线程调用`spawn`、任意任务触发`wake`时，把`Arc<Task>`发送进调度队列；
2. `scheduled`：通道接收端（单消费者）
   `run()`循环阻塞从通道接收待执行任务，**无任务时自动阻塞休眠，不消耗 CPU**，彻底解决忙轮询问题。

### 关联实现

1. `new()` / `Default`：创建 mpsc 收发通道，组装运行时；
2. `spawn()`：对外提交异步任务入口；
3. `run()`：主线程调度循环，持续消费通道任务并执行 poll。

## 三、核心存储结构 TaskFuture

```
struct TaskFuture {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    poll: Poll<()>,
}
```

### 字段含义

1. `future`：类型擦除后的堆存储异步任务，`Pin<Box<>>`满足异步内存固定要求；
2. `poll`：缓存上一次 poll 的结果，标记任务是否已经完成。

### 方法逻辑

```
fn new(future: impl Future<Output = ()> + Send + 'static) -> Self {
    // 新建任务默认状态为Pending
    TaskFuture { future: Box::pin(future), poll: Poll::Pending }
}

fn poll(&mut self, cx: &mut Context<'_>) {
    // 只有任务还处于Pending状态才会执行poll；Ready则跳过，避免重复执行
    if self.poll.is_pending() {
        self.poll = self.future.as_mut().poll(cx);
    }
}
```

关键优化：任务一旦返回`Poll::Ready`，后续再拿到该任务时不会重复 poll，减少无效操作。

## 四、任务主体结构体 Task（唤醒核心载体）

```
struct Task {
    task_future: Mutex<TaskFuture>,
    executor: mpsc::Sender<Arc<Task>>,
}
```

### 字段说明

1. `task_future: Mutex<TaskFuture>`
    - Mutex 保证多线程竞争 poll 时互斥访问，同一时刻最多一个线程轮询该任务；
    - 注释说明：真实 Tokio 使用无锁设计，这里用 Mutex 简化教学；
2. `executor`：调度通道发送端克隆，唤醒时用来把自身重新发送到调度队列。

### 内部方法

#### 1. schedule：将当前任务重新送入调度队列

```
fn schedule(self: &Arc<Self>) {
    let _ = self.executor.send(self.clone());
}
```

克隆自身`Arc<Task>`，通过通道发送给运行时，标记为待执行任务；发送失败直接忽略。

#### 2. 实现 ArcWake trait（异步唤醒核心）

```
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}
```

`ArcWake`是适配`Arc`的唤醒标准接口：
当 Future 内部 IO / 定时器就绪时，调用`cx.waker().wake_by_ref()`，最终会走到这个方法，自动调用`schedule()`把任务重新入队等待轮询。
这是和前面两段代码最大的区别：**真正实现了按需唤醒，不再无脑循环轮询**。

#### 3. poll：单次推进任务逻辑

```
fn poll(self: &Arc<Self>) {
    // 基于当前Task的Arc构造绑定自身的Waker
    let waker = task::waker(self.clone());
    let mut cx = Context::from_waker(&waker);
    // 尝试加锁获取任务内部Future，unwrap简化错误处理
    let mut task_future = self.task_future.try_lock().unwrap();
    // 调用TaskFuture的poll方法推进异步任务
    task_future.poll(&mut cx);
}
```

流程：生成专属 Waker → 创建轮询上下文 → 锁住任务数据 → 执行 poll。

#### 4. spawn：静态构造任务并首次入队

```
fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task>>)
where
    F: Future<Output = ()> + Send + 'static,
{
    // 包裹TaskFuture，装入Mutex，用Arc包装实现多线程共享
    let task = Arc::new(Task{
        task_future: Mutex::new(TaskFuture::new(future)),
        executor: sender.clone(),
    });
    // 首次发送到调度通道，等待run循环消费
    let _ = sender.send(task);
}
```

## 五、MiniTokio 对外 API 完整逻辑

### 1. spawn 提交任务

```
pub fn spawn<F>(&self, f: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    Task::spawn(f, &self.sender);
}
```

外部调用入口，把用户传入的 Future 交给 Task 静态方法构造任务并送入调度通道。

### 2. run 调度主循环（运行时核心）

```
pub fn run(&self) {
    // recv() 阻塞等待通道内有任务，无任务时线程休眠，零CPU占用
    while let Ok(task) = self.scheduled.recv() {
        task.poll()
    }
}
```

执行流程：

1. `scheduled.recv()` 阻塞，没有待执行任务时线程挂起；
2. 通道收到`Arc<Task>`，调用`task.poll()`轮询一次任务；
3. 任务 poll 两种结果：
    - `Poll::Ready`：任务完成，后续再次拿到该任务会直接跳过 poll，不会重新入队；
    - `Poll::Pending`：任务阻塞，Future 内部会持有 Waker，IO / 定时器就绪后自动调用 wake，重新发送到通道等待下一次执行；
4. 通道关闭时循环退出。

## 六、整体完整执行流程示例

```
// 使用示例
fn main() {
    let rt = MiniTokio::default();
    rt.spawn(async {
        // 模拟sleep，内部会保存Waker，倒计时结束调用wake唤醒
        sleep(100).await;
        println!("done");
    });
    rt.run();
}
```

1. `rt.spawn` 创建`Arc<Task>`，发送到 mpsc 通道；
2. `run()` 的`recv()`拿到任务，执行`task.poll()`；
3. 执行`sleep.await`，poll 返回`Poll::Pending`，Future 内部保存当前任务的 Waker；
4. 运行时循环再次调用`recv()`，通道无任务，线程阻塞休眠；
5. sleep 倒计时结束，调用`wake_by_ref()`，触发`Task::schedule()`，把任务重新发送进通道；
6. `recv()`收到唤醒后的任务，再次 poll sleep，返回`Poll::Ready`，任务执行完毕；
7. 通道无新任务，线程持续阻塞。

## 七、对比前两版 MiniTokio 的核心升级点

1. **彻底消除忙轮询**
   使用 mpsc 阻塞接收，无任务时线程休眠；依靠 Wake 按需唤醒，只在任务就绪时才重新调度。
2. **真正线程安全**
   mpsc 多生产者通道 + Arc 共享任务，任意线程都可以调用`spawn`提交任务，前两版只能单线程使用。
3. **标准合规 Waker 唤醒**
   实现`ArcWake`，遵循 Rust 异步标准唤醒模型，兼容所有标准 async/await、IO、定时器 Future；前一版是 noop 空唤醒，完全失效。
4. **状态缓存优化**
   `TaskFuture`缓存 poll 结果，完成的任务不会重复执行 poll，减少开销。
5. **调度与任务解耦**
   运行时 MiniTokio 只负责收发通道，任务自身持有唤醒逻辑，结构分层清晰，贴近真实 Tokio 架构。

## 八、代码存在的局限（教学 Demo，不可生产）

1. 使用`Mutex`保护任务 Future，真实 Tokio 采用无锁队列，锁竞争会降低并发性能；
2. 单消费者调度循环，仅单线程执行 poll，无法多线程并行执行任务；
3. 无内置定时器 / IO 驱动，需要外部提供能触发 wake 的 Future（如 sleep）才能演示唤醒；
4. 错误处理极简：`try_lock().unwrap()`、通道发送失败直接忽略，无任务取消、崩溃恢复逻辑；
5. 通道关闭后 run 直接退出，缺少优雅停机处理。
