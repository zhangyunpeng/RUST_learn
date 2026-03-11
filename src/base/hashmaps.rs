use std::collections::HashMap;


pub fn run() {
    demo2();
}

fn demo2() {
    let s = "this is a sample text, this text just a sample";
    println!("{:?}", count_word(s));
}

fn count_word(s: &str) -> HashMap<&str, usize> {
    // let mut s = s.to_string();
    // s.retain(|x|!x.is_ascii_punctuation());
    // let words = s.split(" ");
    // let mut h = HashMap::new();
    // for word in words {
    //     h.entry(word).and_modify(|num|*num+=1).or_insert(1);
    // }
    // h.len()

    let mut h = HashMap::new();

    s.split_whitespace()
        .map(|word|word.trim_matches(|c: char|c.is_ascii_punctuation()))
        .filter(|word|!word.is_empty())
        .for_each(|word|{
            h.entry(word).and_modify(|n|*n += 1).or_insert(1);
        });
    h
}

fn demo() {
    let mut h = HashMap::new();
    h.insert("k1".to_string(), "v1".to_string());
    h.insert("k2".to_string(), "v2".to_string());
    h.insert("k3".to_string(), "v3".to_string());
    assert!(h.contains_key("k1"));
    h.remove("k1");
    assert_eq!(h.get("k2"), Some(&"v2".to_string()));
    assert_eq!("v2", h["k2"]);

    for (k, v) in &h {
        println!("{} -> {}", k, v);
    }

    for k in h.keys() {
        println!("{}", k);
    }

    for v in h.values() {
        println!("{}", v);
    }

    let val = h.entry("k1".to_string()).or_insert("v1".to_string());
    *val = "v11".to_string();
    println!("{:?}", h);

    h.entry("k2".to_string())
        .and_modify(|v|*v="v22".to_string())
        .or_insert("v2".to_string());
    h.entry("k4".to_string())
        .and_modify(|v|*v="v44".to_string())
        .or_insert("v4".to_string());
    println!("{:?}", h);
    
}