# 07 - 代码风格

统一的代码风格不是审美问题，而是协作效率问题。风格越自动化，review 噪音越低，跨团队阅读与重构成本越可控。
现代语言普遍都有默认风格和配套工具，Rust 也不例外。

## 0. Rust 默认风格

Rust 社区的默认风格由 Style Guide + `rustfmt` 落地，常用基线如下：

- 命名：函数、变量、模块、文件名用 `snake_case`。
- 命名：类型（`struct`/`enum`/`trait`）用 `UpperCamelCase`。
- 命名：常量和 `static` 用 `SCREAMING_SNAKE_CASE`。
- 行宽：`rustfmt` 默认最大行宽为 `100`。
- 缩进：4 空格，不使用 tab。
- 排版：多行参数/字段通常保留尾随逗号，减少 diff 抖动。
- 工具：本地执行 `cargo fmt`，CI 使用 `cargo fmt --check`。

## 1. 核心结论

- 风格统一本质是工程吞吐问题，不是审美问题。
- 项目越大，风格不一致带来的 review 和维护成本越高。
- 现代语言的主流方向是“工具统一风格”，而不是“人工争论风格”。

## 2. C/C++ 为什么很难全局统一

C/C++ 不是没有规范，而是规范长期并行：Google、LLVM、Qt、公司内部规范都在使用。

典型分歧点：

- 命名不统一：`PascalCase`、`camelCase`、`snake_case` 并存。
- 规则优先级不同：有的强调“沿用旧风格”，有的强调“统一新规范”。
- 工具覆盖有限：`clang-format` 能统一格式，但不能统一全部命名和语义约定。

结果是：组织内通常能统一，跨组织/跨项目很难形成单一生态标准。

## 3. Go 的风格哲学：把争论消灭在工具层

Go 的核心策略非常明确：

- `gofmt` 作为事实标准。
- 官方仓库和社区默认强依赖 `gofmt`。
- 尽量少配置（No knobs），减少风格分叉。

工程收益：

- review 更聚焦语义。
- 跨团队阅读成本更低。
- 大规模改动的 diff 更稳定。

## 4. Rust 的风格哲学：统一默认 + 可演进稳定性

Rust 路径和 Go 相近，但更强调“可演进而不抖动”：

- Style Guide 定义默认规则。
- `rustfmt` 落地默认规则。
- 社区普遍推荐少定制，减少项目间漂移。
- 引入 Style Editions，降低工具升级带来的无意义大 diff。

## 5. 业界对“风格工具化”的共同理念

从 Go、Rust 到 JS/Python，趋势一致：

- Prettier：减少风格选项，停止无效争论（JS/TS 前端生态常用格式化工具，强调少配置统一输出）。
- Black：用统一输出换取协作效率（Python 生态常用格式化工具，自称 “uncompromising formatter”）。
- Linux Kernel Rust 指南：明确要求使用 `rustfmt`。

共识只有一个：风格统一且自动化，协作成本才会下降。

## 6. 从系统工程角度看统一风格的收益

- 代码评审：把注意力留给语义、并发、边界条件。
- 重构：机械性改动更安全，diff 信噪比更高。
- 维护：跨模块阅读更快，历史代码理解成本更低。
- AI 协作：代码模式更稳定，生成结果更容易合并。

## 7. 为什么说 Rust 在 style 维度“接近 Go 的优点”

Rust 没有 Go 那么“单一文化”，但工程效果接近：

- `rustfmt` + `clippy` + Cargo 默认流程，让一致性高度自动化。
- 社区命名约定长期稳定（`snake_case`、`UpperCamelCase`、`SCREAMING_SNAKE_CASE`）。
- CI 集成成熟，风格治理成本可控。

## 8. 团队落地建议

- 使用 `rustfmt` 默认配置，尽量不加自定义。
- CI 固化 `cargo fmt --check` 与 `cargo clippy`。
- 风格争议优先交给工具，不在 review 里反复讨论。
- 风格文档只保留“工具覆盖不到的语义规则”。

结论：风格统一做得越自动化，团队越能把精力投入真正的系统问题。

## 9. 参考资料

- Rust Style Guide: <https://doc.rust-lang.org/style-guide/>
- Rust Style Editions: <https://doc.rust-lang.org/stable/style-guide/editions.html>
- Rust API Guidelines（命名约定）: <https://rust-lang.github.io/api-guidelines/naming.html>
- Linux Kernel Rust Coding Guidelines: <https://docs.kernel.org/rust/coding-guidelines.html>
- gofmt 命令文档（Go 官方）: <https://go.dev/src/cmd/gofmt/doc.go>
- gofmt 文化演进（Go 官方演讲）: <https://go.dev/talks/2015/gofmt-en.slide>
- Go Code Review Comments: <https://go.dev/wiki/CodeReviewComments>
- Google C++ Style Guide: <https://google.github.io/styleguide/cppguide.html>
- LLVM Coding Standards: <https://llvm.org/docs/CodingStandards.html>
- Qt Coding Style: <https://wiki.qt.io/Qt_Coding_Style>
- Prettier Option Philosophy: <https://prettier.io/docs/next/option-philosophy>
- Black（The uncompromising formatter）: <https://github.com/psf/black>

## 10. 配套代码

对应示例：[`../src/bin/07_style.rs`](../src/bin/07_style.rs)。

- 该示例演示 Rust 常见命名与排版约定。
- 覆盖点：类型/枚举 `UpperCamelCase`、函数/变量 `snake_case`、常量 `SCREAMING_SNAKE_CASE`。
- 运行命令：`cargo run --bin 07_style`。
