# 00 - 环境与工程结构

你的背景：系统程序员，熟悉 C/C++、QEMU、Linux、高性能用户态程序。

本仓库目标：

- `src/` 放可运行 Rust 代码（小颗粒示例）。
- `notes/` 放与 C/C++ 背景对照的学习笔记。

第一步先保持最小可运行：

- `cargo run`
- `cargo check`

自动编译并运行全部示例：

- `./scripts/run-all.sh`

单个示例编译/运行（以 `01_basics.rs` 为例）：

- 只编译：`cargo build --bin 01_basics`
- 编译并运行：`cargo run --bin 01_basics`

Markdown 规范检查：

- `./scripts/lint-md.sh`

后续会按主题增加：

- 基础语法
- 所有权与借用
- 错误处理
- trait 与工程化
