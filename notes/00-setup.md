# 00 - 环境与工程结构

背景假设：读者具备系统编程经验，熟悉 C/C++、QEMU、Linux 与高性能用户态程序。

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

一句话总结：

- 先用 `run-all` 跑通全量示例，再按章节逐个深入学习对应 `notes` 与 `src/bin`。
