use std::collections::HashMap;

pub fn run() {
    demo();
    demo_search();
    demo_update();
}

fn demo() {
    let team_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<String, i32> = team_list.into_iter().collect();
    println!("{:?}", teams_map);
}

fn demo_search() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 20);
    let team = "Blue".to_string();
    assert_eq!(scores.get(&team), Some(&10));

    let score = scores.get("Blue").copied().unwrap_or(1000);
    assert_eq!(score, 10);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn demo_update() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));
    assert_eq!(scores.get("Blue"), Some(&20));

    let score = scores.entry("Yellow").or_insert(30);
    assert_eq!(*score, 30);
    let score = scores.entry("Yellow").or_insert(20);
    assert_eq!(*score, 30);

    let text = "hello world wonderful world";
    let mut m = HashMap::new();
    for word in text.split_whitespace() {
        let count = m.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", m);

    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // 使用两种方法实现 team_map2
    // 提示:其中一种方法是使用 `collect` 方法
    let teams_map2 = teams.into_iter().collect();

    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}
