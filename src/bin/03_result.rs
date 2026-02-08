use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

fn main() {
    let inputs = ["21", "x", "7"];

    for raw in inputs {
        match parse_and_double(raw) {
            Ok(v) => println!("input={raw}, doubled={v}"),
            Err(e) => println!("input={raw}, error={e}"),
        }
    }
}
