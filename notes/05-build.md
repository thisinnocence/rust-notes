# 05 - 构建与生态

本章把两件常被分开讲的事合在一起：

- 构建系统如何影响工程效率与可维护性。
- 包管理与生态如何决定语言是否能长期落地。

对系统程序员来说，这两者本质上是同一个问题：能不能稳定交付、稳定演进。

## 1. Rust 的工程基础模型

Rust 以 Cargo 统一“包管理 + 构建 + 工作流”：

- `Cargo.toml`：依赖、版本、构建元数据。
- `Cargo.lock`：锁定解析结果，保证复现构建。
- `src/main.rs` / `src/lib.rs` / `src/bin/*.rs`：目标入口约定。
- `target/`：构建产物与缓存。

常用命令：

- `cargo check`：语义检查，不产最终可执行文件。
- `cargo build`：debug 构建。
- `cargo build --release`：优化构建。
- `cargo run --bin <name>`：构建并运行指定目标。

## 2. 对比 C/C++：复杂度转移而不是消失

| 维度 | C/C++ 常见成本 | Rust/Cargo 改进 |
| --- | --- | --- |
| 模块组织 | 头文件依赖与可见性问题 | 语言模块 + crate 统一模型 |
| 依赖链接 | 链接顺序、脚本、工具链组合复杂 | 依赖图由 Cargo 统一解析 |
| 语义发现 | 许多错误暴露较晚 | 更多错误前置到编译期 |
| 工程脚手架 | Make/CMake/Bazel 等割裂 | 默认约定强，冷启动快 |

要点：Rust 不是“没有复杂度”，而是把复杂度从“构建拼装”前移到“类型与语义建模”。

## 3. 对比 Go / Node.js / Python

| 语言 | 运行模型 | 构建与依赖模型 | 对系统工程的意义 |
| --- | --- | --- | --- |
| Go | 有明显 runtime（调度/GC） | `go mod` 一体化较好 | 服务端效率高，裸机/强约束场景受限 |
| Node.js | 解释执行 + VM | npm/pnpm + lockfile | 生态强，部署依赖 runtime |
| Python | 解释执行 + 虚拟环境 | pip/venv/lock 方案多样 | 迭代快，性能热点需下沉 |
| Rust | AOT 编译，无 GC/VM runtime | Cargo + crates.io + lockfile | 低层可控 + 工程化统一 |

## 4. 产物与链接认知

Rust 常见产物类型：

- `bin`：可执行文件。
- `rlib`：Rust 静态库中间产物。
- `dylib`：Rust ABI 动态库。
- `cdylib`：C ABI 动态库（跨语言集成常用）。
- `staticlib`：C ABI 静态库（类似 `.a`）。

符号导出边界：

- `pub` 仅是 Rust 模块可见性，不等于 C ABI 导出。
- 对外稳定导出通常要 `extern "C"` + `#[no_mangle]` + `#[repr(C)]`（结构体）。

## 5. 运行时体量与系统调用路径

核心判断：

- Rust 没有 Go 那种“较大统一 runtime（GC + 调度器）”。
- 但 Rust 也不是“零运行支持”；`panic`、分配器、启动路径仍是运行基础设施。
- 常见 Linux `std` 程序会通过 libc 进入 syscall（具体取决于目标与库选型）。

可选路径：

- `std`：开发效率高，适合大多数用户态程序。
- `libc`/`rustix` 下沉：更接近系统调用边界。
- `no_std`：面向裸机/内核等极简环境。

## 6. `no_std` 与 Bare-Metal：为什么 Rust 可做系统底层

核心开关：

- `#![no_std]`：不链接标准库。
- `#![no_main]`：不用默认入口，自定义启动流程。

和 C/汇编混合常见模式：

- `start.S` + `linker.ld` + Rust 入口。
- C/Rust 通过 `extern "C"` 做 ABI 边界协作。

## 7. 裸机相对 C 的新增注意点

- 必须显式提供 `#[panic_handler]`（通常配 `panic = "abort"`）。
- 需要明确分配器策略（不使用堆时不引入堆类型）。
- MMIO 与中断共享状态仍要遵守 volatile/原子/临界区规则。
- 产物验证（`readelf`/`objdump`/map 文件）仍是硬要求。

结论：Rust 提供的是更强编译期约束，不会替代底层工程责任。

## 8. 为什么“包管理”决定生态生命力

可持续生态必须同时满足：

- 依赖可发现（仓库与索引机制）。
- 依赖可复现（lockfile）。
- 依赖可治理（版本与安全更新可控）。

这也是 C/C++ 历史上最容易分裂的地方：语言强、但工程基础设施长期不统一。

## 9. Rust 生态闭环的工程价值

Rust 的优势不是单点，而是组合：

- Cargo（构建与依赖）
- crates.io（分发）
- `rustfmt` / `clippy`（风格与静态检查）
- `test` / `bench`（验证）

这让“代码资产复利”更容易发生：可复用、可升级、可治理。

## 10. Rust 在系统领域的落地现状

| 方向 | 现状判断 |
| --- | --- |
| CLI 工具 | 已是 Rust 的成熟优势场景之一 |
| Bare-metal/嵌入式 | 基座可用，芯片生态成熟度不均 |
| Linux/QEMU/虚拟化周边 | 持续推进，典型是渐进引入而非一次替换 |
| Python/JS 基建 | “上层语言不变，性能热点下沉 Rust”已常态化 |

## 11. 为什么很多迁移选 Rust 而不是 Go 或 C++ 重写

| 方案 | 优势 | 常见边界 |
| --- | --- | --- |
| 继续 C/C++ | 性能与既有资产延续 | 默认内存安全约束弱，治理成本高 |
| 改 Go | 开发效率高、工程约定统一 | runtime 模型不总适配低层强约束 |
| 改 Rust | 低层可控 + 内存安全 + 工程化统一 | 前期学习与建模成本更高 |

现实通常是混合路线：旧系统保留稳定部分，新增或高风险模块先 Rust 化。

## 12. AI 协作时代的附加收益

Rust 对 AI 协作开发的价值在于：

- 约束显式：类型与所有权让意图更可检查。
- 反馈密集：编译器能快速暴露边界错误。
- 流程统一：Cargo/format/lint/test 易接入自动化流水线。

不是“AI 自动写完”，而是“AI 产出更容易被工具链收敛到可交付代码”。

## 13. 配套代码

对应示例：[`../src/bin/05_build.rs`](../src/bin/05_build.rs)。

- 用最小运行代码把构建信息可视化：
  - `cfg!(target_os/target_arch)`：目标平台分支。
  - `cfg!(debug_assertions)`：debug/release 差异。
  - `CARGO_PKG_*`：Cargo 注入元数据。
- 文件里注释解释了 bin crate、链接边界与 no_std 分流认知。
- 运行：
  - `cargo run --bin 05_build`

## 14. 参考链接

- Linux kernel Rust docs: <https://docs.kernel.org/rust/index.html>
- QEMU Rust docs: <https://www.qemu.org/docs/master/devel/rust.html>
- Embedded Rust Book: <https://docs.rust-embedded.org/book/>
- `embedded-hal` 1.0: <https://blog.rust-embedded.org/embedded-hal-v1/>
- probe-rs: <https://probe.rs/docs/>
- Ubuntu 氧化讨论: <https://discourse.ubuntu.com/t/carefully-but-purposefully-oxidising-ubuntu/56995>
- uv: <https://github.com/astral-sh/uv>
- pydantic v2: <https://pydantic.dev/articles/pydantic-v2>
- SWC: <https://swc.rs/>
