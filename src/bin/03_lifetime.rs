struct View<'a> {
    s: &'a str,
}

fn pick_first<'a>(a: &'a str, _b: &'a str) -> &'a str {
    a
}

// 同一条线：`x` 和返回值都绑在 `'a` 上。
fn same_line<'a>(x: &'a str) -> &'a str {
    x
}

// 两条线：`x` 在 `'a`，`y` 在 `'b`，返回值明确绑到 `'a`（只能返回 x）。
fn two_lines_pick_x<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

// 省略生命周期的等价写法：一个输入引用 -> 输出默认继承该引用生命周期。
fn pick_first_elided(a: &str) -> &str {
    a
}

// 当借用关系太复杂时，返回拥有值通常更易维护。
fn choose_owned(a: &str, b: &str) -> String {
    if a.len() >= b.len() {
        a.to_string()
    } else {
        b.to_string()
    }
}

// 下面是“故意错误”的反例，保留为阅读注释，不参与编译：
//
// fn bad_return_ref<'a>() -> &'a str {
//     let s = String::from("temp");
//     &s
// }
//
// 这会报错：`s` 在函数结束时销毁，返回它的借用会悬垂。
// 结论：生命周期标注不能把“本来无效的借用”变成有效。

fn main() {
    let s1 = String::from("alpha");
    let s2 = String::from("beta-long");

    let r1 = pick_first(&s1, &s2);
    let r2 = pick_first_elided(&s1);
    println!("picked explicit={r1}, picked elided={r2}");

    let v = View { s: &s1 };
    println!("view={}", v.s);

    let owned = choose_owned(&s1, &s2);
    println!("owned={owned}");

    // 连线视角：返回引用跟着哪条生命周期线走。
    let r3 = same_line(&s2);
    let r4 = two_lines_pick_x(&s1, &s2);
    println!("same_line={r3}, two_lines_pick_x={r4}");
}
