struct View<'a> {
    s: &'a str,
}

fn pick_first<'a>(a: &'a str, _b: &'a str) -> &'a str {
    a
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
}
