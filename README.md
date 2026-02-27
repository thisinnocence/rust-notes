# rust-notes

一个面向系统程序员（C/C++ 背景）的 Rust 学习仓库。

项目采用“人类 + AI 结对学习”方式持续迭代：

- 开发者负责提出问题、验证理解、运行代码。
- AI 助手负责小颗粒示例、对照笔记、持续重构文档。
- 每次改动尽量小提交，方便回看学习轨迹。

## rust 资料

- <https://doc.rust-lang.org/stable/book/>
- <https://kaisery.github.io/trpl-zh-cn/title-page.html>
- <https://www.zhihu.com/question/400584073/answer/3665589601>
- <https://document.kirigaya.cn/docs/rust-tutorial/ch00-00-introduction.html>
- Rust for Rustaceans: Idiomatic Programming for Experienced Developers

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
| 18 | 常用标准库速览 | [`notes/18-stdlib.md`](notes/18-stdlib.md) | [`src/bin/18_stdlib.rs`](src/bin/18_stdlib.rs) |

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

## 学习建议

面向有 C/C++ 经验的系统程序员，建议采用“小步快跑 + 边界优先”的学习方式：

| 关注点 | 建议做法 |
| --- | --- |
| 学习节奏 | 每次只攻克 1-2 个核心概念（例如 ownership + borrow），避免一天铺太多主题。 |
| 编码策略 | 先写“拥有值”版本（`String`/`Vec`），再逐步引入借用与生命周期。 |
| 数据结构迁移 | 先掌握 `Option<Box<Node>>` 这类单所有权模型，再引入 `Rc`/`Arc`/`RefCell`/`Weak`。 |
| 重构心智 | 接受“编译期前置约束”：早期改动更硬，但长期运行期问题更少。 |
| 工程习惯 | 每次改动保持小提交，保证可回看、可回滚、可定位。 |
| AI 协作 | 让 AI 生成最小示例与对照注释，人类负责问题拆解、边界判断和最终验收。 |

实践上可遵循一句话：先保证正确边界，再追求抽象优雅和语法熟练度。资深程序员的推荐路线是：

| 阶段 | 目标 | 推荐动作 |
| --- | --- | --- |
| 第 1 阶段：语义建模 | 建立 Rust 核心心智 | 重点只学 ownership、borrow、lifetime、Result；每个概念配 1 个最小可运行示例。 |
| 第 2 阶段：真实切片 | 从“会语法”到“会落地” | 选一个小而真实的模块（配置解析/协议编解码/状态机）用 Rust 重写并可测试。 |
| 第 3 阶段：边界治理 | 能在工程里长期维护 | 建立 FFI/unsafe 边界规则；把“不变量”写进注释和测试。 |
| 第 4 阶段：性能与并发 | 把可用提升到可扩展 | 引入 profiling、benchmark、并发压测；区分“热路径优化”和“可维护性优化”。 |
| 第 5 阶段：抽象收敛 | 团队可复制的开发方式 | 把通用模式沉淀为 crate、模板、review checklist，减少个人风格差异。 |

配套习惯建议：

- 每次学习都绑定“一个可运行结果”（`cargo run`/`cargo test`）。
- 遇到生命周期报错先判断“返回值借用了谁”，再决定是否改为返回拥有值。
- 尽量避免早期堆叠复杂类型体操，优先让模型正确、可读、可验证。
- 保持小提交，确保每一步都能独立回看和回滚。

路线总则：

- 以“核心小项目”驱动学习，语法笔记只保留为项目服务的最小解释，避免大而散的语法铺开。
- 大而散的文档会让人迷失在细节里，失去对核心概念的把握，而且也串不起来，记不住没有意义。
- 每个概念都要配一个最小可运行示例，确保理解和应用的闭环。
