use std::fmt;
use std::num::ParseIntError;

// 自定义业务错误类型：把“解析失败”和“校验失败”统一到一个错误枚举。
#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    NonPositive(i32),
    Odd(i32),
}

impl From<ParseIntError> for AppError {
    fn from(value: ParseIntError) -> Self {
        AppError::Parse(value)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "parse error: {e}"),
            AppError::NonPositive(n) => write!(f, "value must be > 0, got {n}"),
            AppError::Odd(n) => write!(f, "value must be even, got {n}"),
        }
    }
}

fn parse_i32(raw: &str) -> Result<i32, AppError> {
    // `?` 的行为：
    // - Ok(v) => 解包得到 v，继续执行
    // - Err(e) => 立即 return Err(e)（这里会经 From 转成 AppError）
    let n: i32 = raw.parse()?;
    Ok(n)
}

fn require_positive(n: i32) -> Result<i32, AppError> {
    if n > 0 {
        Ok(n)
    } else {
        Err(AppError::NonPositive(n))
    }
}

fn require_even(n: i32) -> Result<i32, AppError> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(AppError::Odd(n))
    }
}

fn parse_validate_and_double(raw: &str) -> Result<i32, AppError> {
    // 三个 `?` 串起来，形成“失败就早返回”的错误传播链。
    let n = parse_i32(raw)?;
    let n = require_positive(n)?;
    let n = require_even(n)?;
    Ok(n * 2)
}

fn main() {
    let inputs = ["21", "8", "-4", "x"];

    for raw in inputs {
        match parse_validate_and_double(raw) {
            Ok(v) => println!("input={raw}, doubled={v}"),
            Err(e) => println!("input={raw}, error={e}"),
        }
    }
}
