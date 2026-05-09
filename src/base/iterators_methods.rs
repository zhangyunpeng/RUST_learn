use std::collections::HashMap;

/// 迭代器适配器 map() fliter() skip() take() enumerate() zip() ...
/// 消耗性适配器 collect() sum() count() max() ...
pub fn run() {
    demo2();
}

fn demo1() {
    let mut numbers = [1, 2, 3];
    for item in &mut numbers {
        *item *= *item;
    }
    println!("{:?}", &numbers);

    let mut numbers = [1, 2, 3];
    numbers.iter_mut().for_each(|number| *number *= *number);
    println!("{:?}", &numbers);

    let numbers: [i32; 3] = [1, 2, 3];
    numbers
        .iter()
        .map(|&x| x * 2)
        .filter(|&x| x > 5)
        .for_each(|x| println!("{}", x));

    let number_strings = numbers
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", number_strings);

    let fruit_basket: Vec<(&str, i32)> = vec![("apple", 1), ("banana", 3), ("origin", 3)];
    let fruit_map = fruit_basket
        .into_iter()
        .filter(|(_, n)| *n > 2)
        .collect::<HashMap<&str, i32>>();
    println!("{:?}", fruit_map);

    let chars = vec!['r', 'u', 's', 't'];
    let word: String = chars.into_iter().collect();
    println!("{}", word);
}

fn demo2() {
    let transactions = [
        TransAction {
            amount: 100.0,
            desc: "100_desc".to_string(),
        },
        TransAction {
            amount: 90.0,
            desc: "90_desc".to_string(),
        },
        TransAction {
            amount: 80.0,
            desc: "80_desc".to_string(),
        },
    ];
    // let summay: (f64, usize) = transactions.into_iter().collect();
    // println!("{:?}", summay);
    let trans_map: HashMap<String, f64> = transactions.into_iter().collect();
    println!("{:?}", trans_map);

    let numbers = [1, 2, 3, 4];
    let evens = numbers
        .into_iter()
        .filter(|&number| number % 2 == 0)
        .collect::<Vec<i32>>();
    println!("{:?}", evens);
}

impl FromIterator<TransAction> for HashMap<String, f64> {
    fn from_iter<T: IntoIterator<Item = TransAction>>(iter: T) -> Self {
        let mut m = HashMap::new();
        for item in iter {
            m.insert(item.desc, item.amount);
        }
        m
    }
}

impl FromIterator<TransAction> for (f64, usize) {
    fn from_iter<T: IntoIterator<Item = TransAction>>(iter: T) -> Self {
        let mut total_amount: f64 = 0.0;
        let mut total: usize = 0;
        for item in iter {
            total_amount += item.amount;
            total += 1;
        }
        (total_amount, total)
    }
}

#[derive(Debug)]
struct TransAction {
    amount: f64,
    desc: String,
}
