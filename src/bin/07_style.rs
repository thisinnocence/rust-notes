//! 07_style 用最小示例展示 Rust 社区常见风格约定
//!
//! 运行
//! cargo run --bin 07_style
//! 风格格式化
//! cargo fmt
//! 风格检查, CI 常用
//! cargo fmt --check

use std::time::Duration;

// 类型名使用 UpperCamelCase, 即大驼峰
struct RetryPolicy {
    // 结构体字段名使用 snake_case
    max_retry: u32,
    backoff_ms: u64,
}

// 枚举名同样使用 UpperCamelCase, 变体 variant 也使用 UpperCamelCase
// 变体 variant 就是 enum 里的每个选项名, 这里是 Fast 和 SafeMode
enum RunMode {
    Fast,
    SafeMode, // 多单词时仍使用 UpperCamelCase
}

// 常量名使用 SCREAMING_SNAKE_CASE, 即全大写下划线
const DEFAULT_TIMEOUT_MS: u64 = 250;

// 函数名使用 snake_case, 即小写下划线
fn calc_total_timeout(policy: &RetryPolicy) -> Duration {
    // 局部变量也使用 snake_case
    let total_ms = u64::from(policy.max_retry) * policy.backoff_ms + DEFAULT_TIMEOUT_MS;
    Duration::from_millis(total_ms)
}

// impl 块常见顺序是先构造和核心 API, 再辅助方法
impl RetryPolicy {
    fn new(max_retry: u32, backoff_ms: u64) -> Self {
        Self {
            max_retry,
            backoff_ms,
        }
    }

    fn describe(&self, mode: RunMode) -> String {
        // match 的每个分支 arm 结尾都用逗号, 即使最后一个 arm 也保留逗号
        let mode_str = match mode {
            RunMode::Fast => "fast",
            RunMode::SafeMode => "safe",
        };
        format!(
            "mode={mode_str}, max_retry={}, backoff_ms={}",
            self.max_retry, self.backoff_ms
        )
    }
}

fn main() {
    let policy = RetryPolicy::new(4, 30);
    let timeout = calc_total_timeout(&policy);

    // 缩进由 rustfmt 统一, 默认 4 空格, 行宽默认 100
    println!("{}", policy.describe(RunMode::Fast));
    println!("{}", policy.describe(RunMode::SafeMode));
    println!("total_timeout_ms={}", timeout.as_millis());

    // 风格建议
    // 1 不手工对齐空格, 交给 rustfmt
    // 2 导入按工具默认顺序, 不做个性化微调
    // 3 review 聚焦语义, 不讨论排版细节
}
