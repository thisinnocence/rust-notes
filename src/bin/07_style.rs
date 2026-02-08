//! 07_style: 用最小示例展示 Rust 社区常见风格约定。
//!
//! 运行：
//! cargo run --bin 07_style

use std::time::Duration;

// 类型名：UpperCamelCase（大驼峰）。
struct RetryPolicy {
    max_retry: u32,
    backoff_ms: u64,
}

// 枚举名同样用 UpperCamelCase；变体也用 UpperCamelCase。
enum RunMode {
    Fast,
    Safe,
}

// 常量：SCREAMING_SNAKE_CASE（全大写下划线）。
const DEFAULT_TIMEOUT_MS: u64 = 250;

// 函数名：snake_case（小写下划线）。
fn calc_total_timeout(policy: &RetryPolicy) -> Duration {
    // 局部变量也用 snake_case。
    let total_ms = u64::from(policy.max_retry) * policy.backoff_ms + DEFAULT_TIMEOUT_MS;
    Duration::from_millis(total_ms)
}

// impl 块常见顺序：先构造/核心 API，再辅助方法。
impl RetryPolicy {
    fn new(max_retry: u32, backoff_ms: u64) -> Self {
        Self {
            max_retry,
            backoff_ms,
        }
    }

    fn describe(&self, mode: RunMode) -> String {
        let mode_str = match mode {
            RunMode::Fast => "fast",
            RunMode::Safe => "safe",
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

    // 缩进由 rustfmt 统一，默认 4 空格；行宽默认 100。
    println!("{}", policy.describe(RunMode::Fast));
    println!("{}", policy.describe(RunMode::Safe));
    println!("total_timeout_ms={}", timeout.as_millis());

    // 风格建议：
    // 1) 不手工对齐空格，交给 rustfmt。
    // 2) 导入按工具默认顺序，不做个性化微调。
    // 3) review 聚焦语义，不讨论排版细节。
}
