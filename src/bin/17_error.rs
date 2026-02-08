use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum DomainError {
    Parse(ParseIntError),
    OutOfRange(i32),
}

impl From<ParseIntError> for DomainError {
    fn from(e: ParseIntError) -> Self {
        Self::Parse(e)
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::Parse(e) => write!(f, "parse error: {e}"),
            DomainError::OutOfRange(v) => write!(f, "value out of range: {v}"),
        }
    }
}

fn parse_port(raw: &str) -> Result<u16, DomainError> {
    let n: i32 = raw.parse()?;
    if (1..=65535).contains(&n) {
        Ok(n as u16)
    } else {
        Err(DomainError::OutOfRange(n))
    }
}

fn main() {
    for s in ["8080", "0", "abc"] {
        match parse_port(s) {
            Ok(p) => println!("ok port={p}"),
            Err(e) => println!("err input={s}: {e}"),
        }
    }
}
