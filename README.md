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

- AI 编程助手：Codex（GPT-5 系列能力）。
- 开发环境：VSCode Remote。
- 交互方式：命令行驱动 AI 完成代码与文档迭代。

## 当前目录

```text
.
├── Cargo.toml
├── Cargo.lock
├── README.md
├── notes/
│   ├── 00-setup.md
│   ├── 01-basics.md
│   ├── 02-ownership.md
│   ├── 03-result.md
│   ├── 04-trait.md
│   ├── 05-build.md
│   ├── 06-style.md
│   ├── 07-module.md
│   ├── 08-memory.md
│   ├── 09-concurrency.md
│   ├── 10-async.md
│   ├── 11-async-advanced.md
│   ├── 12-unsafe.md
│   ├── 13-ffi.md
│   ├── 14-lifetime.md
│   ├── 15-testing.md
│   ├── 16-perf.md
│   └── 17-error-handling.md
├── scripts/
│   └── lint-md.sh
└── src/
    ├── main.rs
    └── bin/
        ├── 01_basics.rs
        ├── 02_ownership.rs
        ├── 03_result.rs
        ├── 04_trait.rs
        ├── 05_build.rs
        ├── 06_style.rs
        ├── 07_module.rs
        ├── 08_memory.rs
        ├── 09_concurrency.rs
        ├── 10_async.rs
        ├── 11_async_advanced.rs
        ├── 12_unsafe_boundary.rs
        ├── 13_ffi.rs
        ├── 14_lifetime.rs
        ├── 15_testing.rs
        ├── 16_perf.rs
        └── 17_error.rs
```

## 学习路径（章节对应）

| 章节 | 主题 | 示例 |
| --- | --- | --- |
| 00 | 环境与工程结构 | - |
| 01 | 基础语法 | `01_basics.rs` |
| 02 | 所有权与借用 | `02_ownership.rs` |
| 03 | Result 与错误处理入门 | `03_result.rs` |
| 04 | Struct/Enum/Trait | `04_trait.rs` |
| 05 | Build + Ecosystem | `05_build.rs` |
| 06 | Style | `06_style.rs` |
| 07 | 模块系统 | `07_module.rs` |
| 08 | 内存模型与布局 | `08_memory.rs` |
| 09 | 并发 | `09_concurrency.rs` |
| 10 | 异步（基础） | `10_async.rs` |
| 11 | 异步（进阶） | `11_async_advanced.rs` |
| 12 | unsafe 边界 | `12_unsafe_boundary.rs` |
| 13 | FFI | `13_ffi.rs` |
| 14 | 生命周期 | `14_lifetime.rs` |
| 15 | 测试策略 | `15_testing.rs` |
| 16 | 性能工程 | `16_perf.rs` |
| 17 | 错误处理工程化 | `17_error.rs` |

## 常用命令

```bash
# 检查所有示例
cargo check --bins

# 运行某一章示例（示例：05）
cargo run --bin 05_build

# 运行测试（示例）
cargo test --bin 12_unsafe_boundary --bin 13_ffi

# Markdown 规范检查
./scripts/lint-md.sh
```

## 学习目标

- 用系统编程视角掌握 Rust 核心语义（所有权、错误模型、trait、模块）。
- 建立构建系统、包管理、生态治理的工程化认知。
- 逐步过渡到并发、异步、unsafe/FFI、性能与可维护性实践。
