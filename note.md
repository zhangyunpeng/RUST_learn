## 打印相关宏
1. print!: 与 printn!相似，但是不会在输出末尾添加换行符。用于在同一行打印多个内容。
2. eprintln!：与 println!相似，但它会打印到标准错误流，而不是标准输出流。
3. format!：创建包含多个值的字符串
4. write!： 与 format！相似，但它写入的是缓存区，不是穿件一个新字符串
## 实用的cargo工具
1. rustfmt 格式化检查  
```
rustup component add rustfmt
cargo fmt
```
2. clippy 检测cargo check 无法识别的不友好写法
```
rustup component add clippy
cargo fmt
```
3. cargo fix 做rust版本升级很有用，自动更新错误的语法，非100%修正
## 字符串编辑 r
```
let text = r"";
// r####"内容"####;
let text = r###"sunshuine"###;
```
## ASCII 转换
```
let b: u8 = b'+';
let c: char = 43 as char;
```
## char
```
let infinity_symbol = '\u{221E}';
println!("symbol = {}, usv = {}", infinity_symbol, infinity_symbol as u32);
输出： symbol = ∞, usv = 8734
```
## array
```
let arr: [i32;2] = [1,2];
let arr = [0;1024];
for num in arr {
    println!("{}", num);
}
```

## 内存安全
1. Null pointer dereference：Rust 没有传统意义上的空指针，而使用 Option<T>枚举 (Some(T)或 None) 来明确表示“可能有值，可能无值”，强制调用者处理“无”的情况。
2. use after free：通过“生命周期”和“借用检查器”确保引用不会超过其被引用数据的生存期。这是编译时的静态检查。
3. Dangling pointer：是“悬垂指针”的直接后果。所有权系统确保：当值的所有者（例如变量）离开作用域时，其资源被自动且唯一地释放（Drop），之后任何试图通过旧引用访问的行为在编译时被阻止。
4. Double free：所有权唯一性保证了资源只有唯一的所有者能释放它。当所有权移动后，原变量不再有效，无法再次释放。资源在所有者离开作用域时自动释放一次。
5. Buffer overflows：标准库中的集合类型（如 Vec<T>, String）会在访问时进行边界检查（除非使用明确的不安全方法 get_unchecked）。数组的索引操作在运行时会检查是否越界，若越界则panic，而不是导致未定义行为。
6. Data race： 借用规则保证了：  
• 要么任意多个不可变引用​ (&T)。  
• 要么唯一一个可变引用​ (&mut T)。  
这从定义上消除了数据竞争的可能性。Send和 Synctrait 进一步约束了跨线程共享的安全性。  
7. 迭代器失效​：Rust 的迭代器大多是“借用的”，受到生命周期和借用规则的保护。如果在迭代过程中尝试修改容器（比如通过&mut），编译器会报错，因为它违反了“同时存在可变和不可变引用”的规则。
8. 未初始化内存​： Rust 强制变量必须在使用前初始化，编译器会进行检查。对于内存级别的操作，标准库提供了 MaybeUninit<T>等工具来明确、安全地处理未初始化的内存。
9. 整数溢出​： 在 Debug 模式下，整数溢出会导致 panic。在 Release 模式下，默认采用补码环绕（wrapping）行为，但程序员可通过 wrapping_*, checked_*, saturating_*, overflowing_*等方法明确指定所需的溢出处理逻辑，从而避免意外。

rust 不允许在运行时写入非法内存  

### panic
1. panic::set_hook
2. panic::catch_unwind
```
    let arr = [1,2,3];
    let result = panic::catch_unwind(||{
        for i in 0..10 {
            println!("{}", arr[i]);
        }
    });
    match result {
        Ok(_) => println!("No panic"),
        Err(_) => println!("Catch Panic"),
    }
```   

## 测试
1. ```#[cfg(test)]```
2. ```#[test]```
3. ```#[should_panic(expected = "reason")]```

## 运算符与特征
|运算符|特征|
|:---:|:---:|
|==|PartialEq|
|!=|PartialEq|
|>|PartialOrd|
|<|PartialOrd|
|>=|PartialOrd|
|<>=|PartialOrd|

## 位运算与特征
|运算符|特征|
|:---:|:---:|
|!|Not|
|&|BitAnd|
|```|```|BitOr|
|<<|Shl|
|>>|Shr|
|^|BitXor|

## 引用
1. 比较（== != > >= < <= ）两个引用时，对比的不是内存地址，而是引用指向的值。需要注意 类型不匹配时，无法比较： &i32与&i32可以比较，&i32与&&i32不可比较，可以转换为&i32与*&&i32比较 
2. 

## 循环
1. loop:  无限循环，直到碰到break
    可以为loop添加标签 ``` 'loop_name: {} ```, 可以使用break loop_name 跳到指定循环, 作为表达式时， 可以在break 后携带返回值。
2. for in: ``` for element in collection  ```, collection 可以是array range vec 或者是任何实现了 Iterator trait的类型, 也支持标签
3.  while: 
```
while condition {}
while let pattern = value {}
```

## 元组
1. 使用 ref 或 元组引用，避免部分所有权被转移

## 结构体
1. Tuple struct 例: ``` struct A(i32, String) ```
2. Named struct 例: ``` struct Person { name: String, age: u8} ```
3. Unit struct  例: ``` struct D() ```

结构体默认不实现 Copy Trait， 即使它的所有成员都实现了 Copy Trait, 需要通过 ```#[derive(Copy, Clone)] ```实现，实现Copy必须同时实现Clone, 同样只有所有成员都实现了 Copy， 才可以为结构体派生 Copy  
``` #[derive(Default)] ``` 为结构体派生默认值实现   
## 向量
### vs array
1. array 存储在栈上，不可修改其大小
2. vec 存储在堆上， 可动态调整大小
### funcs
drain() 排空指定范围 参数为Range  
extract_if 剔除制定范围中，满足条件的元素 ``` extract_if(.., |x|x%2==0) ``` 返回迭代器  
retain(&mut self, f: FnMut(&T)->bool) 保留指定元素  
retain_mut()  
split_at() split_at_mut() 按照索引分割， 区别在于后者返回可变引用  
split_at_unchecked split_at_mut_unchecked 非安全的按照索引切割， 不进行边界检查， 区别在于后者返回可变引用  
split() 返回一个迭代器，该迭代器遍历由匹配元素分割的子切片。 匹配的元素不包含在子切片中   
rsplit() 同split(), 迭代返回顺序从右开始  
splitn() 返回一个迭代器，该迭代器遍历由匹配 pred 的元素分隔的子切片，最多返回 n 个子切片。匹配的元素不包含在子切片中。如果还有剩余元素，则最后一个返回的子切片将包含切片的剩余部分。  
rsplitn() 同split(), 迭代返回顺序从右开始 
splice() 创建一个拼接迭代器，将向量中指定范围替换为给定的 replace_with迭代器，并返回被移除的元素  
append() 移除 other 中所有元素，追加到self  
extend() 扩展, 同上，但会转移 other的所有权

## VecDeque
一个使用可增长环形缓冲区实现的双端队列。  
该类型作为队列的“默认”用法是使用 push_back添加到队列，使用 pop_front从队列中移除。extend和 append以这种方式从尾部添加元素，而迭代 VecDeque则是从前到后进行。  




