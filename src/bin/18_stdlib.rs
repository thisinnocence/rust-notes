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
    // `Vec<T>` 是标准库容器类型，`vec![]` 是创建 `Vec` 的宏（不是函数）。
    // 这里会推断为 `Vec<i32>`。
    let mut nums = vec![5, 2, 9, 1, 5, 6];
    // `sort()` 是原地排序（in-place），会直接重排 `nums` 内部元素。
    nums.sort();
    println!("sort={:?}", nums);

    // 这里会推断为 `Vec<(&str, i32)>`，即“元素是二元组”的向量。
    let mut pairs = vec![("alice", 3), ("bob", 1), ("carol", 2)];
    pairs.sort_by_key(|x| x.1);
    println!("sort_by_key={:?}", pairs);

    let mut desc = vec![3, 7, 1, 9, 2];
    desc.sort_by(|a, b| b.cmp(a));
    println!("sort_desc={:?}", desc);
}

fn map_set_demo() {
    // 这里显式写了类型，便于学习阅读。
    // 实战里也可省略为 `let mut m = HashMap::new();`，
    // 编译器会根据后续 insert 推断出 `HashMap<&str, i32>`（体验近似 C++ auto）。
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
