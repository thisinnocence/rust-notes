struct View<'a> {
    s: &'a str,
}

fn pick_first<'a>(a: &'a str, _b: &'a str) -> &'a str {
    a
}

fn main() {
    let s1 = String::from("alpha");
    let s2 = String::from("beta");

    let r = pick_first(&s1, &s2);
    println!("picked={r}");

    let v = View { s: &s1 };
    println!("view={}", v.s);
}
