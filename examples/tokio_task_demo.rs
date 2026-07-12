// use tokio::task;

// #[tokio::main]
// async fn main() {
//     let v = vec![1, 2, 3];
//     // 需要move转移v所有权
//     task::spawn(async move {
//         println!("{:?}", v);
//     });
// }

use std::rc::Rc;
use tokio::task::yield_now;
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // 作用域让 rc 在 await 执行前销毁
        {
            let r = Rc::new("hello");
            println!("r: {}", r);
        }
        // rc 不再使用，任务挂起时不会保存该变量
        yield_now().await;
    });
}

// 错误示例
// #[tokio::main]
// async fn main() {
//     tokio::spawn(async {
//         let r = Rc::new("hello");
//         // await 之后仍会使用 rc，变量会被存入任务状态
//         yield_now().await;
//         println!("r: {}", r);
//     });
// }
