fn takes_ownership(s: String) {
    println!("owned: {s}");
}

fn borrow_len(s: &str) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // move, s1 不再可用

    takes_ownership(s2);

    let mut msg = String::from("hello");
    let len = borrow_len(&msg);
    println!("msg={msg}, len={len}");

    append_world(&mut msg);
    println!("after mut borrow: {msg}");

    // 规则提示：
    // 1) 同一时刻，要么多个不可变借用，要么一个可变借用。
    // 2) 引用必须始终有效，编译期检查。
}
