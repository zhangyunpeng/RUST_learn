use std::marker::PhantomPinned;
use std::pin::Pin;

// PhantomPinned 标记这个类型 !Unpin
struct Foo {
    _marker: PhantomPinned,
}

pub fn demo() {
    // 普通 Unpin 类型，Pin 随便拆
    let mut v = vec![1, 2, 3];
    let pinned = Pin::new(&mut v);
    let inner = pinned.get_mut();
    inner.push(4);
    assert_eq!(v, vec![1, 2, 3, 4]);

    //  自引用！Unpin 类型，Pin 锁死，无法取出裸引用
    let f = Foo {
        _marker: PhantomPinned,
    };
    let boxed = Box::pin(f);
    let _pinned = boxed;
    // 下面这行编译报错：Foo 没有实现 Unpin，不能调用 get_mut
    // let inner = pinned.get_mut();
}
