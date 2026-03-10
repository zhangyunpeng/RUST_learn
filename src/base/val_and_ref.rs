
// 自动解引用
pub fn demo_deref() {
    let v = 42;
    let ref1 = &v;
    let ref2 = &ref1;
    println!("val : {}", v);
    println!("val : {}", ref1);
    println!("val : {}", ref2);
    println!("val : {}", *ref1);
    println!("val : {}", **ref2);


    // 比较（== != > >= < <= ）两个引用时，对比的不是内存地址，而是引用指向的值
    // 需要注意 类型不匹配时，无法比较： &i32与&i32可以比较，&i32与&&i32不可比较，可以转换为&i32与*&&i32比较 
    let v = 20;
    let v_ref = &v;
    let v1 = 20;
    let v1_ref = &v1;
    if v_ref == v1_ref {
        println!("{} == {}", v_ref, v1_ref);
    }

}


pub fn demo() {
    let arr = [1,2,3,4,5];
    // 传值， [i32;5]存储在栈上，传值时,进行copy，后续arr还可以继续使用
    println!("arr greatest: {}", find_greatest_val_by_value(arr));
    println!("arr: {:?}", arr);
    println!("arr greatest: {}", find_greatest_val_by_ref(&arr));
    println!("arr: {:?}", arr);

}

fn find_greatest_val_by_value(arr: [i32; 5]) -> i32 {
    // let mut val = 0;
    // for item in arr {
    //     val = val.max(item);
    // }
    // val
    arr.into_iter().max().unwrap_or(0)
}

fn find_greatest_val_by_ref(arr: &[i32; 5]) -> i32 {
    // let mut val = 0;
    // for item in arr {
    //     val = val.max(*item);
    // }
    // val
    *arr.iter().max().unwrap_or(&0)
}