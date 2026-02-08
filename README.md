# rust-notes

一个面向系统程序员（C/C++ 背景）的 Rust 学习仓库。

项目采用“人类 + AI 结对学习”方式持续迭代：

- 开发者负责提出问题、验证理解、运行代码。
- AI 助手负责小颗粒示例、对照笔记、持续重构文档。
- 每次改动尽量小提交，方便回看学习轨迹。

## AI 协作说明

当前仓库由 AI 协作生成与维护，核心目标：

- 每章都有可运行示例（`src/bin/*.rs`）。
- 每章都有系统化笔记（`notes/*.md`）。
- 重点做 C/C++ 向 Rust 的迁移式理解。

当前实践工具：

- AI 编程助手：Codex（模型：gpt-5.3-codex，日期：2026-02-08）。
- 开发环境：VSCode Remote。
- 交互方式：命令行驱动 AI 完成代码与文档迭代。

## 学习路径（章节对应）

| 章节 | 主题 | 笔记 | 示例 |
| --- | --- | --- | --- |
| 00 | 环境与工程结构 | [`notes/00-setup.md`](notes/00-setup.md) | - |
| 01 | 基础语法 | [`notes/01-basics.md`](notes/01-basics.md) | [`src/bin/01_basics.rs`](src/bin/01_basics.rs) |
| 02 | 所有权与借用 | [`notes/02-ownership.md`](notes/02-ownership.md) | [`src/bin/02_ownership.rs`](src/bin/02_ownership.rs) |
| 03 | 生命周期 | [`notes/03-lifetime.md`](notes/03-lifetime.md) | [`src/bin/03_lifetime.rs`](src/bin/03_lifetime.rs) |
| 04 | Result 与错误处理 | [`notes/04-result.md`](notes/04-result.md) | [`src/bin/04_result.rs`](src/bin/04_result.rs) |
| 05 | Struct/Enum/Trait | [`notes/05-trait.md`](notes/05-trait.md) | [`src/bin/05_trait.rs`](src/bin/05_trait.rs) |
| 06 | 构建与生态 | [`notes/06-build.md`](notes/06-build.md) | [`src/bin/06_build.rs`](src/bin/06_build.rs) |
| 07 | 代码风格 | [`notes/07-style.md`](notes/07-style.md) | [`src/bin/07_style.rs`](src/bin/07_style.rs) |
| 08 | 模块系统 | [`notes/08-module.md`](notes/08-module.md) | [`src/bin/08_module.rs`](src/bin/08_module.rs) |
| 09 | 内存模型与布局 | [`notes/09-memory.md`](notes/09-memory.md) | [`src/bin/09_memory.rs`](src/bin/09_memory.rs) |
| 10 | 并发编程 | [`notes/10-concurrency.md`](notes/10-concurrency.md) | [`src/bin/10_concurrency.rs`](src/bin/10_concurrency.rs) |
| 11 | 异步编程 | [`notes/11-async.md`](notes/11-async.md) | [`src/bin/11_async.rs`](src/bin/11_async.rs) |
| 12 | 异步进阶：Pin、取消与背压 | [`notes/12-async-advanced.md`](notes/12-async-advanced.md) | [`src/bin/12_async_advanced.rs`](src/bin/12_async_advanced.rs) |
| 13 | unsafe 边界 | [`notes/13-unsafe.md`](notes/13-unsafe.md) | [`src/bin/13_unsafe_boundary.rs`](src/bin/13_unsafe_boundary.rs) |
| 14 | FFI | [`notes/14-ffi.md`](notes/14-ffi.md) | [`src/bin/14_ffi.rs`](src/bin/14_ffi.rs) |
| 15 | 测试策略 | [`notes/15-testing.md`](notes/15-testing.md) | [`src/bin/15_testing.rs`](src/bin/15_testing.rs) |
| 16 | 性能分析与优化 | [`notes/16-perf.md`](notes/16-perf.md) | [`src/bin/16_perf.rs`](src/bin/16_perf.rs) |
| 17 | 错误处理工程化 | [`notes/17-error-handling.md`](notes/17-error-handling.md) | [`src/bin/17_error.rs`](src/bin/17_error.rs) |

## 常用命令

```bash
# 检查所有示例
cargo check --bins

# 运行某一章示例（示例：06）
cargo run --bin 06_build

# 运行测试（示例）
cargo test --bin 13_unsafe_boundary --bin 14_ffi

# Markdown 规范检查
./scripts/lint-md.sh
```

## 学习目标

- 用系统编程视角掌握 Rust 核心语义（所有权、错误模型、trait、模块）。
- 建立构建系统、包管理、生态治理的工程化认知。
- 逐步过渡到并发、异步、unsafe/FFI、性能与可维护性实践。
