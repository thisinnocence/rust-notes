//! 06_build: 通过可运行示例理解 Rust 构建系统关键点。
//!
//! 运行：
//! cargo run --bin 06_build
//!
//! 你可以配合这些命令观察构建行为：
//! 1) cargo check --bin 06_build
//! 2) cargo build --bin 06_build
//! 3) cargo build --release --bin 06_build
//! 4) file target/debug/06_build
//! 5) ldd target/debug/06_build    (Linux + 动态链接时可用)

use std::env;

/// 类型名使用 UpperCamelCase。
struct BuildInfo {
    profile: &'static str,
    target_os: &'static str,
    target_arch: &'static str,
}

/// 函数名使用 snake_case。
fn current_profile() -> &'static str {
    // debug_assertions 常用于区分 debug/release 逻辑。
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

/// cfg!(...) 在运行期得到 bool，但条件由编译期确定。
fn target_os_name() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "other"
    }
}

fn target_arch_name() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "riscv64") {
        "riscv64"
    } else {
        "other"
    }
}

fn build_info() -> BuildInfo {
    BuildInfo {
        profile: current_profile(),
        target_os: target_os_name(),
        target_arch: target_arch_name(),
    }
}

fn main() {
    let info = build_info();

    println!("profile={}", info.profile);
    println!("target_os={}", info.target_os);
    println!("target_arch={}", info.target_arch);

    // 这行展示 Cargo 在构建时注入的环境变量（若存在）。
    // 对比 C/C++：很多信息常由外部脚本拼接；Rust/Cargo 约定更集中。
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown".to_string());
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
    println!("package={pkg_name} v{pkg_version}");

    // 核心认知注释：
    // 1) 这个示例是 bin crate，直接产出可执行文件。
    // 2) 没有头文件与手工链接顺序；依赖和构建图由 Cargo 管理。
    // 3) 默认 std 程序在 Linux 常见会链接 libc（动态或静态，取决于目标和配置）。
    // 4) 若进入 no_std/bare-metal，会切到另一套启动与运行支持模型。
}
