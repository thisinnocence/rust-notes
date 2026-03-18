//! Day 3 最小练习
//!
//! 运行:
//! cargo run --bin day-03 -- Tom 20
//! cargo run --bin day-03 -- Tom xx
//! cargo run --bin day-03
//!
//! 目标:
//! - 看懂 `Result<T, E>` 的基本返回方式
//! - 看懂 `?` 怎么把错误往上抛
//! - 看懂“自定义错误类型 + 分层报错”的最小结构
//! - 不追求复杂 CLI，只追求能顺着代码读下来

use std::env;

const APP_NAME: &str = "day-03-errors";

#[derive(Debug)]
struct CliInput {
    name: String,
    age: u8,
}

#[derive(Debug)]
enum CliError {
    MissingName,
    MissingAge,
    InvalidAge,
}

fn parse_name(raw: Option<&String>) -> Result<String, CliError> {
    match raw {
        // TODO 1:
        // 把这一行自己补成 `Ok(name.clone())`
        Some(name) => Ok(name.clone()),
        None => Err(CliError::MissingName),
    }
}

fn parse_age(raw: Option<&String>) -> Result<u8, CliError> {
    // TODO 3:
    let raw = raw.ok_or(CliError::MissingAge)?;
    // 把字符串解析成 u8
    // 提示: `raw.parse::<u8>().map_err(|_| CliError::InvalidAge)`
    raw.parse::<u8>().map_err(|_| CliError::InvalidAge)
}

// 这一层只负责“把原始输入解析成结构化数据”。
// 出错就返回 Result::Err，不在这里打印。
fn parse_cli(args: &[String]) -> Result<CliInput, CliError> {
    let name = parse_name(args.get(1))?;
    let age = parse_age(args.get(2))?;

    // TODO 4:
    // 把这里自己补成 `Ok(CliInput { name, age })`
    Ok(CliInput { name, age })
}

fn age_group(age: u8) -> &'static str {
    if age >= 18 { "adult" } else { "minor" }
}

// 这一层只负责业务输出。
// 它不关心参数是怎么来的，只接收已经解析好的数据。
fn build_message(input: &CliInput) -> String {
    format!(
        "hello, {}! age={}, group={}",
        input.name,
        input.age,
        age_group(input.age)
    )
}

// 分层关系:
// main -> run_app -> parse_cli -> parse_name/parse_age
//
// `?` 的作用:
// - 如果下层返回 Ok(...)，继续往下执行
// - 如果下层返回 Err(...)，立刻把错误返回给上一层
fn run_app(args: &[String]) -> Result<(), CliError> {
    let input = parse_cli(args)?;
    let message = build_message(&input);
    println!("{message}");
    Ok(())
}

fn print_error(err: &CliError) {
    match err {
        CliError::MissingName => {
            println!("error=缺少 name 参数");
            println!("tip: cargo run --bin day-03 -- Tom 20");
        }
        CliError::MissingAge => {
            println!("error=缺少 age 参数");
            println!("tip: cargo run --bin day-03 -- Tom 20");
        }
        CliError::InvalidAge => {
            println!("error=age 解析失败，必须是 u8");
            println!("tip: cargo run --bin day-03 -- Tom 20");
        }
    }
}

fn print_day3_checklist() {
    println!();
    println!("== Day 3 checklist ==");
    println!("[ ] 我能解释 `Result<T, E>` 里的 T 和 E");
    println!("[ ] 我能解释 `?` 为什么会提前返回");
    println!("[ ] 我能看懂自定义错误类型为什么常写成 enum");
    println!("[ ] 我能说清楚 parse_cli 和 run_app 的分层职责");
    println!("[ ] 我能自己改一个新的错误分支并跑通");
}

fn main() {
    println!("app={APP_NAME}");

    let args: Vec<String> = env::args().collect();
    println!("args={args:?}");

    match run_app(&args) {
        Ok(()) => println!("status=ok"),
        Err(err) => print_error(&err),
    }

    print_day3_checklist();
}
