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

## 学习路径（章节对应）

1. `00` 环境与工程结构
   - 笔记：[`notes/00-setup.md`](notes/00-setup.md)
2. `01` 基础语法
   - 笔记：[`notes/01-basics.md`](notes/01-basics.md)
   - 示例：[`src/bin/01_basics.rs`](src/bin/01_basics.rs)
3. `02` 所有权与借用
   - 笔记：[`notes/02-ownership.md`](notes/02-ownership.md)
   - 示例：[`src/bin/02_ownership.rs`](src/bin/02_ownership.rs)
4. `03` Result 与错误处理入门
   - 笔记：[`notes/03-result.md`](notes/03-result.md)
   - 示例：[`src/bin/03_result.rs`](src/bin/03_result.rs)
5. `04` Struct/Enum/Trait
   - 笔记：[`notes/04-trait.md`](notes/04-trait.md)
   - 示例：[`src/bin/04_trait.rs`](src/bin/04_trait.rs)
6. `05` Build + Ecosystem
   - 笔记：[`notes/05-build.md`](notes/05-build.md)
   - 示例：[`src/bin/05_build.rs`](src/bin/05_build.rs)
7. `06` Style
   - 笔记：[`notes/06-style.md`](notes/06-style.md)
   - 示例：[`src/bin/06_style.rs`](src/bin/06_style.rs)
8. `07` 模块系统
   - 笔记：[`notes/07-module.md`](notes/07-module.md)
   - 示例：[`src/bin/07_module.rs`](src/bin/07_module.rs)
9. `08` 内存模型与布局
   - 笔记：[`notes/08-memory.md`](notes/08-memory.md)
   - 示例：[`src/bin/08_memory.rs`](src/bin/08_memory.rs)
10. `09` 并发
    - 笔记：[`notes/09-concurrency.md`](notes/09-concurrency.md)
    - 示例：[`src/bin/09_concurrency.rs`](src/bin/09_concurrency.rs)
11. `10` 异步（基础）
    - 笔记：[`notes/10-async.md`](notes/10-async.md)
    - 示例：[`src/bin/10_async.rs`](src/bin/10_async.rs)
12. `11` 异步（进阶）
    - 笔记：[`notes/11-async-advanced.md`](notes/11-async-advanced.md)
    - 示例：[`src/bin/11_async_advanced.rs`](src/bin/11_async_advanced.rs)
13. `12` unsafe 边界
    - 笔记：[`notes/12-unsafe.md`](notes/12-unsafe.md)
    - 示例：[`src/bin/12_unsafe_boundary.rs`](src/bin/12_unsafe_boundary.rs)
14. `13` FFI
    - 笔记：[`notes/13-ffi.md`](notes/13-ffi.md)
    - 示例：[`src/bin/13_ffi.rs`](src/bin/13_ffi.rs)
15. `14` 生命周期
    - 笔记：[`notes/14-lifetime.md`](notes/14-lifetime.md)
    - 示例：[`src/bin/14_lifetime.rs`](src/bin/14_lifetime.rs)
16. `15` 测试策略
    - 笔记：[`notes/15-testing.md`](notes/15-testing.md)
    - 示例：[`src/bin/15_testing.rs`](src/bin/15_testing.rs)
17. `16` 性能工程
    - 笔记：[`notes/16-perf.md`](notes/16-perf.md)
    - 示例：[`src/bin/16_perf.rs`](src/bin/16_perf.rs)
18. `17` 错误处理工程化
    - 笔记：[`notes/17-error-handling.md`](notes/17-error-handling.md)
    - 示例：[`src/bin/17_error.rs`](src/bin/17_error.rs)

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
