# rust-notes

一个面向系统程序员（C/C++ 背景）的 Rust 学习仓库。

这个项目采用“人类 + AI 结对学习”方式推进：

- 我（开发者）负责提出问题、验证理解、运行代码。
- AI 助手负责小颗粒代码示例、对照笔记、持续迭代文档。
- 每次改动尽量小提交，方便回看学习轨迹。

## AI 协作说明

当前仓库的内容由 AI 协作生成和维护，重点是：

- 按章节输出可运行示例（`src/bin/*.rs`）。
- 按章节输出对照笔记（`notes/*.md`）。
- 面向 C/C++、Go、Java、Node.js、Python 背景做迁移解释。

配套 AI 工具与模型（当前实践）：

- AI 编程助手：Codex（GPT-5 系列能力）。
- 开发环境：VSCode Remote（本地查看、调试、阅读）。
- 交互方式：通过命令行驱动 AI 完成代码与文档迭代。

## 项目结构

```text
.
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   └── bin/
│       ├── 01_basics.rs
│       ├── 02_ownership.rs
│       ├── 03_result.rs
│       └── 04_trait.rs
├── notes/
│   ├── 00-setup.md
│   ├── 01-basics.md
│   ├── 02-ownership.md
│   ├── 03-result.md
│   ├── 04-trait.md
│   └── 05-build.md
└── scripts/
    └── lint-md.sh
```

## 快速开始

```bash
cargo check --bins
cargo run --bin 01_basics
cargo run --bin 02_ownership
cargo run --bin 03_result
cargo run --bin 04_trait
```

## 文档规范检查

```bash
./scripts/lint-md.sh
```

## 学习目标

- 用系统编程视角掌握 Rust 核心语义（所有权、借用、错误处理、trait）。
- 理解 Rust 构建系统与 C/C++、Go、Node.js、Python 的差异。
- 逐步过渡到 `no_std` / bare-metal / 内核级开发路径。
