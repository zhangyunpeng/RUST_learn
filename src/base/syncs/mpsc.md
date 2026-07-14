## 一、基础定义区分

### 1. `channel()` — 无界异步通道

```
pub fn channel<T>() -> (Sender<T>, Receiver<T>)
```

- 缓冲：**逻辑无限缓冲区**，堆上动态扩容
- 发送行为：`tx.send()` **永不阻塞**，只要内存充足就能写入
- 发送端类型：`Sender<T>`
- 场景：消息量可控、不希望发送线程被阻塞

### 2. `sync_channel(bound)` — 有界同步通道

```
pub fn sync_channel<T>(bound: usize) -> (SyncSender<T>, Receiver<T>)
```

- 缓冲：固定容量 `bound`，预先分配有限缓冲区
- 发送行为：缓冲区满时 `tx.send()` **阻塞线程**，直到有消费者取出消息腾出空间
- 发送端类型：`SyncSender<T>`
- 特殊：`bound = 0` 会面通道，无任何缓冲，发送方必须等接收方同时就绪才完成传递

## 二、核心差异对照表

表格

| 特性 | channel () 无界通道 | sync\_channel (N) 有界同步通道 |
| --- | --- | --- |
| 发送端类型 | `Sender<T>` | `SyncSender<T>` |
| 缓冲区大小 | 无限，动态扩容 | 固定 N，上限可控 |
| send () 是否阻塞 | 永远不阻塞 | 缓冲满则阻塞 |
| 内存风险 | 消息堆积会 OOM 内存溢出 | 有上限，天然限流 |
| bound=0 模式 | 不支持 | 会面通道（rendezvous） |
| 多生产者 | 支持，可 clone tx | 支持，可 clone tx |
| 消费者 | 单消费者，Receiver 不可克隆 | 单消费者 |

## 三、底层行为细节

### 1. channel () 无界通道

1. 内部链表存储消息，发送时直接追加；
2. 无论多少消息，发送线程立刻返回，不会休眠；
3. 缺点：如果消费速度远低于生产速度，消息无限堆在堆内存，最终程序内存暴涨崩溃；
4. `Sender` 只实现 `Send`，不实现 `Sync`（但可 clone 多线程发送）。

示例：

```
use std::sync::mpsc::channel;
let (tx, rx) = channel();
// 疯狂发送不会阻塞
for i in 0..100000 {
    tx.send(i).unwrap();
}
```

### 2. sync\_channel (N) 有界通道

1. 固定长度队列，最多存 N 条未消费消息；
2. 队列已满时调用 `send()`，线程阻塞休眠，直到 `recv()` 取出一条；
3. 天然限流，防止内存爆炸；
4. `SyncSender` 同样支持多线程 clone 并发发送。

#### 特殊：sync\_channel (0) 会面通道

没有缓冲区，发送和接收必须同时阻塞等待对方：

```
let (tx, rx) = std::sync::mpsc::sync_channel(0);
// 单独发送会永久阻塞，必须配合接收线程
std::thread::spawn(move || {
    tx.send(100).unwrap();
});
rx.recv().unwrap();
```

## 四、断开逻辑（两者完全一致）

1. 所有发送端全部销毁 → `rx.recv()` 返回 `Err(Disconnected)`；
2. 接收端销毁 → 任意发送端 `send()` 返回 `Err(SendError)`；
3. 两者都不支持多消费者（Receiver 不能 clone），属于 MPSC（多生产者单消费者）。

## 五、适用场景选择

### 选 channel () 无界通道

- 消息数量少、消费速度稳定，不会堆积；
- 不允许发送线程阻塞，追求发送低延迟；
- 内部事件分发、轻量通知。

### 选 sync\_channel () 有界通道

- 生产速度可能远超消费，需要限流防 OOM；
- 需要背压机制：生产过快时让生产者停下来；
- 任务队列、工作池，限制最大待处理任务数；
- 需要 0 缓冲的一对一同步会面场景。

## 六、关键方法差异

1. `Sender<T>`（无界）
    - 只有 `send()`，无阻塞逻辑；
    - 无 `try_send` 专属优势，塞满不存在。
2. `SyncSender<T>`（有界）
    - `send()`：满则阻塞；
    - `try_send()`：不阻塞，缓冲区满直接返回 `Err(Full)`，适合非阻塞尝试发送。

```
let (tx, rx) = std::sync::mpsc::sync_channel(2);
tx.send(1).unwrap();
tx.send(2).unwrap();
// 缓冲已满，try_send 立即返回错误
assert!(tx.try_send(3).is_err());
```
