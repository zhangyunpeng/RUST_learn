# 用大白话 + 分步例子讲 Pin，完全抛开专业术语

## 一、先搞懂最核心的灾难场景：自引用结构体

假设我们写一个结构体，里面存一段字符串，同时存一个指向自己字符串的引用：

```rust
struct SelfRef {
    buf: String,
    ptr: &str, // ptr 指向自己的 buf
}
```

正常构造它：

```rust
let mut s = SelfRef { buf: "hello".into(), ptr: "" };
s.ptr = &s.buf;
```

现在内存布局：
栈上 `s` 有两块数据，`buf` 在堆上，`ptr` 存的是 `buf` 的堆地址。

### 灾难来了：Rust 默认允许**移动值**

```rust
let s1 = s; // 把 s 移动给 s1
```

移动之后：
原来栈上 `s` 的内存失效，数据搬到 `s1` 的栈位置。
但结构体内部的 `ptr` 存的是**旧地址**，现在指向一块无效内存，变成野指针 → 内存安全崩溃。

这就是 async/await 会遇到的问题：
`async fn` 编译出来的 Future 是一个状态机，跨 `.await` 会保存对自身内部数据的引用，也就是**自引用结构体**。
一旦这个 Future 被移动，内部引用全部失效，程序直接崩。

**Pin 的唯一使命：阻止自引用类型被移动，杜绝野指针。**

## 二、Pin 到底是个啥？

```rust
#[repr(transparent)]
pub struct Pin<Ptr>(Ptr);
```

1. 它只是一层**零开销包装**，里面只放一个指针 / Box / 引用；
2. 它不包裹数据本身，包裹「指向数据的指针」；
3. 它靠**编译期类型检查**限制你移动里面指针指向的数据，没有运行时逻辑。

举两个直观区分：

- `Pin<Box<T>>`：包装 Box（指针），锁住堆上的 T 不让移动；
- `Pin<&mut T>`：包装可变引用，锁住栈上的 T 不让移动；

重点：`Pin` 管的是 `T`（被指向的数据），不是外面的 Box / 引用。

## 三、两个阵营：Unpin 和！Unpin

### 1. Unpin：随便动，Pin 管不住

绝大多数普通类型：`i32`、`String`、`Vec`、没有自引用的结构体，编译器自动给它们实现 `Unpin`。

规则：只要 T 是 Unpin，`Pin<任何指针<T>>` 形同虚设，你随时能拿出 `&mut T`、随便移动 T。

```rust
let mut num = 10;
let pinned = Pin::new(&mut num); // 合法，i32: Unpin
// 直接取出内部可变引用随便改、随便移动
let inner = pinned.get_mut();
```

### 2. !Unpin：不能随便动，Pin 锁死

自引用结构体、async 生成的 Future、标记 `PhantomPinned` 的类型，**没有自动实现 Unpin**。

规则：只要 `T: !Unpin`，编译器会锁死所有能取出裸 `&mut T`、移动 T 的 API：

- 不能调用 `pinned.get_mut()`；
- 不能把内部 Box / 引用拆出来；
- 不能把 T 挪到别的变量里。

```rust
// Future 是 !Unpin
let f: Pin<Box<dyn Future>> = Box::pin(async {});
// f.get_mut(); // 编译报错！不允许取出裸可变引用
```

## 四、两个构造函数，分清 safe 和 unsafe

### 1. Pin::new ()（安全）

仅当 `T: Unpin` 才能调用。
编译器知道这个类型不怕移动，包装 Pin 没有安全风险，不用 unsafe。

### 2. Pin::new\_unchecked (指针)（unsafe）

没有 Unpin 限制，你可以把指向 `!Unpin` 类型的指针包进 Pin。
但你要手动向编译器保证：**我保证永远不会移动这块数据**。
一旦你违反承诺，就会出现野指针，未定义行为。

我们常用的 `Box::pin(future)` 内部就是封装了 `Pin::new_unchecked`，帮你做安全保证：
Box 把 Future 放在堆上，堆地址永远不会变，所以可以安全包装。

## 五、为什么 Future.poll 必须是 self: Pin<&mut Self>？

如果 poll 长这样（假设）：

```rust
fn poll(&mut self, cx: &mut Context) -> Poll<()>
```

传入普通 `&mut Self`，调用方可以在 poll 期间把整个 Future 移走，内部自引用直接报废。

改成：

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<()>
```

1. 传入的是被 Pin 锁住的可变引用；
2. Future 是 `!Unpin`，无法从这个 Pin 引用里拿出完整的 Future 移动；
3. 整个 poll 过程中，Future 在内存里的地址固定不变，内部跨 await 的引用全部有效。

## 六、举个极简可运行例子，直观感受 Pin 的限制

### 案例 1：普通 Unpin 类型，Pin 随便拆

```rust
use std::pin::Pin;

fn main() {
    let mut v = vec![1,2,3];
    let mut pinned = Pin::new(&mut v);
    // 可以直接取出内部可变引用
    let inner = pinned.get_mut();
    inner.push(4);
}
```

### 案例 2：自引用！Unpin 类型，Pin 锁死，无法取出裸引用

```rust
use std::pin::{Pin, PhantomPinned};

// PhantomPinned 标记这个类型 !Unpin
struct Foo {
    _marker: PhantomPinned,
}

fn main() {
    let foo = Foo { _marker: PhantomPinned };
    let boxed = Box::pin(foo);
    let mut pinned: Pin<Box<Foo>> = boxed;

    // 下面这行编译报错：Foo 没有实现 Unpin，不能调用 get_mut
    // let inner = pinned.get_mut();
}
```

## 七、结合你之前 MiniTokio 代码解释

```rust
future: Pin<Box<dyn Future<Output = ()> + Send>>
```

1. `async { delay_ms(100).await }` 编译生成的 Future 内部有自引用，属于 `!Unpin`；
2. `Box::pin` 将 Future 分配到堆，堆地址固定，同时用 `Pin` 包装；
3. 因为被 Pin 包裹，我们无法把这个 Future 从 Box 里取出来移动；
4. 调用 `future.as_mut()` 只能得到 `Pin<&mut dyn Future>`，刚好满足 `poll` 方法的参数要求；
5. 轮询期间 Future 地址全程不变，`.await` 保存的内部引用不会变成野指针。

## 八、一句话总结 Pin

1. 问题根源：自引用类型一旦移动会产生野指针；
2. Pin 方案：用一层指针包装，在编译期阻止移动 `!Unpin` 类型；
3. Unpin 是豁免标签：普通无自引用类型不受 Pin 限制；
4. 只在处理 async Future、自引用结构体时需要，普通同步代码完全碰不到。