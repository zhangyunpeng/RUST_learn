
pub fn run() {
    demo();
}

fn demo() {
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", find_val(&arr, 3));
    println!("{:?}", find_val(&arr, 6));
}


/*
    find_val 多版本
*/
// fn find_val(arr: &[i32], target: i32) -> Option<usize> {
//     for (index, value) in arr.iter().enumerate() {
//         if target.eq(value) {
//             return Some(index)
//         }
//     }
//     None
// }

// fn find_val(arr: &[i32], target: i32) -> Option<usize> {
//     arr.iter().position(|x| *x == target)
// }

// fn find_val(arr: &[i32], target: i32) -> Option<usize> {
//     arr.iter()
//         .enumerate()
//         .find(|(_, x)| **x == target)
//         .map(|(index, _)|index)
// }

fn find_val<T: PartialEq>(arr: &[T], target: T) -> Option<usize> {
    arr.iter().position(|x| *x == target)
}