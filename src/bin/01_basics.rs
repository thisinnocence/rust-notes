fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sign_text(v: i32) -> &'static str {
    match v {
        x if x > 0 => "positive",
        0 => "zero",
        _ => "negative",
    }
}

fn main() {
    let language = "rust";
    let mut score = 95;
    score += 1;

    let sum = add(3, 5);
    println!("language={language}, score={score}, sum={sum}");

    for i in 0..3 {
        println!("i={i}, sign={}", sign_text(i - 1));
    }

    let condition = score > 90;
    let level = if condition { "A" } else { "B" };
    println!("level={level}");
}
