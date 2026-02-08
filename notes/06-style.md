# 06 - 代码风格

这份文档单独讨论 coding style，不讲语法技巧，而讲工程治理：

- 为什么现代语言越来越倾向“工具统一风格”。
- 为什么 C/C++ 世界长期风格碎片化。
- 为什么 Go/Rust 在这件事上更容易快速达成一致。

## 1. 核心结论

- 风格统一不是“审美问题”，是协作吞吐问题。
- 大规模工程里，风格越分裂，review 噪音越高，重构成本越大。
- 现代语言的趋势是：把风格从“人治规范”变成“工具默认行为”。

## 2. C/C++ 为什么很难全局统一

C/C++ 并不是没有好规范，而是“规范太多且并行存在”：

- Google C++ Style Guide
- LLVM Coding Standards
- Qt Coding Style
- 各公司内部规范（命名、括号、注释、头文件组织）

典型碎片例子（同为 C++）：

- 命名风格并不一致：函数是 `PascalCase`、`camelCase`、还是 `snake_case`，不同体系不同。
- 规则优先级不同：有的强调“延续现有代码风格”，有的强调“统一到组织规范”。
- 工具一致性不足：clang-format 能解决格式，但解决不了所有命名和语义风格分歧。

这导致一个现实：

- 组织内部能统一。
- 跨组织、跨项目很难形成像 Go 那样的“生态级单一风格”。

## 3. Go 的风格哲学：把争论消灭在工具层

Go 社区做了一个很激进但成功的选择：

- `gofmt` 作为事实标准（de facto style）。
- Go 官方仓库强制 gofmt。
- “No knobs（尽量少配置）”成为文化共识。

行业评价里最经典的一句是：

- “gofmt 的风格不是任何人的最爱，但 gofmt 是所有人的最爱。”

工程收益非常直接：

- code review 更聚焦语义，不再纠缠排版。
- 新人上手快，跨团队阅读成本低。
- 大规模改动的 diff 更可控。

## 4. Rust 的风格哲学：统一默认 + 可演进稳定性

Rust 路径和 Go 类似，但又更强调“长期演进稳定”：

- Rust Style Guide 定义默认风格。
- `rustfmt` 使用该默认风格。
- 社区普遍推荐默认配置，减少项目间风格漂移。
- 引入 Style Editions（风格版本）来平衡“可演进”和“避免格式抖动”。

这套机制解决了一个长期问题：

- 风格可以进化，但不会让旧项目每次升级工具链都产生无意义大改 diff。

## 5. 业界对“风格工具化”的共同理念

从 Go、Rust 到 JS/Python，主流趋势高度一致：

- Prettier：明确强调“停止风格争论”，并限制新增格式选项。
- Black：主张“uncompromising formatter”，用统一输出换取协作效率。
- Linux Kernel Rust 指南：明确要求使用 `rustfmt`，减少 review 往返。

这背后的共识不是“某种括号风格更美”，而是：

- 统一且可自动化的风格，能显著降低协作摩擦。

## 6. 从系统工程角度看，统一风格的真实收益

对系统程序员背景而言，收益可量化在这些环节：

- 代码评审：减少样式争论，审查带宽留给语义、并发、边界条件。
- 重构：风格一致使机械性改动更安全，diff 信噪比更高。
- 维护：跨模块阅读速度更快，历史代码理解成本更低。
- AI 协作：模型更容易学习稳定模式，生成代码更容易一次合并。

## 7. 为什么说 Rust 在 style 维度“接近 Go 的优点”

Rust 没有像 Go 那样“绝对单一”的风格工具文化，但工程效果很接近：

- `rustfmt` + `clippy` + Cargo 的默认工作流让一致性高度自动化。
- 命名约定在社区层面高度稳定（`snake_case`、`UpperCamelCase`、`SCREAMING_SNAKE_CASE`）。
- 工具链反馈与 CI 集成成熟，风格治理成本较低。

## 8. 团队落地建议

可采用“最小可争议”策略：

- 统一 `rustfmt` 默认配置，尽量不加自定义项。
- CI 强制 `cargo fmt --check` 与 `cargo clippy`。
- 规则争议优先交给工具，不在 review 里人工反复争论。
- coding style 文档只保留“工具覆盖不到的语义约定”。

结论：

- 风格统一做得越自动化，团队越能把脑力花在真正重要的系统问题上。

## 9. 参考资料（官方与一手为主）

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

## 10. 配套代码怎么读（`src/bin/06_style.rs`）

- 这个文件只做一件事：把“命名/缩进/格式”放进可运行代码里直观看。
- 覆盖点：
  - 类型/枚举：`UpperCamelCase`
  - 函数/变量：`snake_case`
  - 常量：`SCREAMING_SNAKE_CASE`
  - `match` 与 `impl` 常见排布方式
- 运行命令：
  - `cargo run --bin 06_style`
