# async/await 完整原理

## 一、async/await 底层核心原理

### 1. async 函数本质： 编译成 Future trait 状态机

Rust 中 async fn foo() -> T 不是普通函数，编译器会做语法糖脱糖:
```rust
async fn foo(x: i32) -> i32 {
    let a = bar().await;
    a + x
}
```
等价于生成一个匿名结构体，实现 std::future::Future trait:
1. 结构体保存所有跨 await 点的局部变量（状态）;
2. 每个 await 对应一个状态分支，形成状态机；
3. Future 只有一个核心方法：``` poll(&mut self, cx: &mut Context<'_>) -> Poll<T> ```

Poll 返回值两种情况
1. Poll::Ready(val): 任务完成，返回结果；
2. Poll::Pending： 任务未就绪，让出执行权，等待唤醒

### 2. Context & Waker：任务唤醒机制
Context 封装 Waker， 作用：
1. Future 返回 Pending 时，告诉运行时 ```[```我现在没数据，别轮询我了```]```
2. 当 IO 定时器就绪时，调用 ``` waker.wake() ```，把任务放回调度队列，下次被poll

### 3. await 的脱糖逻辑
fut.await 等价于循环 poll：
```rust
loop {
    match Future::poll(fut, cx) {
        Poll::Ready(res) => break res,
        Poll::Pending => return Poll::Pending,
    }   
}
```
只要子 Future 未就绪， 外层 Future 直接返回 Pending，保存当前状态，挂起。

### 4. 运行时（Runtime）职责
async Future 本身不会自动执行，必须有 Runtime 驱动：
1. 任务队列： 保存带唤醒的 Future
2. 调度器：循环取出任务调用 poll
3. IO 定时器驱动（复杂 runtime）：底层事件循环（epoll/io_uring）,就绪后唤醒 Waker
4. 多线程调度（tokio/async-std）：多线程执行任务，避免阻塞

### 5. 关键特性总结
1. 无栈协程：async Future 是用户状态机，不依赖操作系统线程栈，内存开销极小
2. 零成本抽象：同步代码无额外开销，仅 await 点产生状态存储
3. 单线程也能并发：基于事件驱动，不用多线程即可同时处理大量 IO 任务

