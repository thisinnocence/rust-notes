use std::collections::{HashMap, HashSet};

fn string_demo() {
    let mut s = String::from("  rust stdlib 101  ");
    println!("raw={:?}", s);

    // 这里的 `s` 是可变绑定（`mut`，来自 mutable），所以允许重新赋值。
    // `trim()` 返回借用视图 `&str`，`to_string()` 会创建一个新的 `String`。
    // 新值赋给 `s` 后，旧的 `String` 在此处被 drop 并释放。
    s = s.trim().to_string();
    s.push_str(" guide");
    let replaced = s.replace("101", "one-zero-one");
    let words: Vec<&str> = replaced.split_whitespace().collect();

    println!("trim+push={}", s);
    println!("replaced={}", replaced);
    println!("words={:?}", words);
    println!("starts_with_rust={}", replaced.starts_with("rust"));
}

fn sort_demo() {
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    nums.sort();
    println!("sort={:?}", nums);

    let mut pairs = vec![("alice", 3), ("bob", 1), ("carol", 2)];
    pairs.sort_by_key(|x| x.1);
    println!("sort_by_key={:?}", pairs);

    let mut desc = vec![3, 7, 1, 9, 2];
    desc.sort_by(|a, b| b.cmp(a));
    println!("sort_desc={:?}", desc);
}

fn map_set_demo() {
    let mut m: HashMap<&str, i32> = HashMap::new();
    m.insert("threads", 8);
    m.insert("workers", 4);

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("epoll");
    set.insert("io_uring");
    set.insert("epoll");

    println!("map_threads={:?}", m.get("threads"));
    println!("set_len={}", set.len());
}

fn main() {
    string_demo();
    sort_demo();
    map_set_demo();
}
