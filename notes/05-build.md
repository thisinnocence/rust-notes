# 05 - Build / 包管理 / 链接产物（系统视角）

本章目标：把 Rust 的构建系统放在你熟悉的 C/C++、Go、Node.js、Python 语境下看清楚。

## 1. Rust 构建的核心模型

Rust 默认使用 `cargo` 作为“包管理 + 构建编排 + 依赖解析”一体工具。

一个最小工程约定：

- `Cargo.toml`：包元数据、依赖、构建配置。
- `Cargo.lock`：依赖解析结果锁定（应用项目建议提交）。
- `src/main.rs`：二进制入口。
- `src/lib.rs`：库入口。
- `src/bin/*.rs`：多个独立可执行目标。
- `target/`：构建产物目录。

常用命令：

- `cargo check`：只做类型检查和语义分析，不产出最终可执行文件。
- `cargo build`：构建 debug 产物。
- `cargo build --release`：构建优化版产物。
- `cargo run --bin <name>`：构建并运行指定二进制。

## 2. 对比 C/C++ 构建体系

你关心的“为什么不需要复杂 C++ 构建工程”核心在这里：

| 维度 | C/C++ 常见痛点 | Rust 的改进点 |
| --- | --- | --- |
| 模块与组织 | 头文件依赖与二义性高 | 模块系统由语言和包管理统一，无头文件二义性 |
| 依赖与链接 | 链接顺序和依赖关系常需手工拼装 | 依赖图由 Cargo 统一解析，编译单元是 crate |
| 语义校验 | 很多问题暴露在较后阶段 | 语义检查更早，很多错误在编译期前置 |
| 工程脚手架 | 工具链碎片化，前期构建搭建成本高 | 默认目录约定更强，减少“先搭构建脚手架”成本 |

不是说 Rust 没复杂度，而是复杂度从“构建脚本拼装”转移到了“类型/所有权语义建模”。

## 3. 对比 Go / Node.js / Python

### Go

- `go mod` 和 Cargo 都是“官方一等公民”工具链。
- Go 产物默认单可执行文件体验好；Rust 也可做到单文件部署（取决于链接方式和目标）。
- Go 有较明显 runtime（调度器、GC、栈扩缩等）；Rust 无 GC runtime/VM。

### Node.js（npm/pnpm）

- Node 产物通常依赖运行时解释器（Node VM），部署时是“代码 + node_modules + runtime”。
- Rust 是 ahead-of-time 编译，部署通常是“最终二进制 + 少量系统动态库（若动态链接）”。
- `pnpm-lock.yaml` 对应 Rust 的 `Cargo.lock`，都用于可复现依赖版本。

### Python

- 常依赖解释器 + 虚拟环境 + site-packages。
- Rust 无虚拟环境概念，依赖由 Cargo 解析并编译进产物（或以动态库形式链接）。
- Python 包隔离偏环境级（venv）；Rust 隔离偏项目级（lockfile + target 目录）。

## 4. 版本隔离与可复现

Rust 的“版本隔离”主要靠：

- 每个项目自己的 `Cargo.lock`。
- 依赖解析遵循 SemVer 约束（`Cargo.toml`）+ 锁定结果（`Cargo.lock`）。
- 构建缓存位于全局 cargo 缓存和项目 `target`，但解析结果由项目锁文件固定。

可类比：

- Node: `package.json` + `pnpm-lock.yaml`
- Rust: `Cargo.toml` + `Cargo.lock`

## 5. 产物类型：`exe` / `so` / `a`

Rust 可以直接产出多种目标（crate type）：

- 可执行文件：`bin`（默认 `src/main.rs`）。
- Rust 静态库中间产物：`rlib`。
- 动态库（Rust ABI）：`dylib`。
- C ABI 动态库：`cdylib`（用于给 C/C++/Python 等调用）。
- C ABI 静态库：`staticlib`（类似 `.a`，用于外部链接）。

系统集成时常用：

- 给 C/C++ 调 Rust：`cdylib` 或 `staticlib` + `extern "C"` + `#[no_mangle]`。

## 6. 符号导出与可见性（你问的“符号模型”）

Rust 默认并不把所有函数都当作 C 可见符号导出。

- `pub`：是 Rust 模块可见性，不等于对外 ABI 导出。
- 跨 FFI 稳定导出通常要：
- `extern "C"`（调用约定）
- `#[no_mangle]`（禁用符号改名）
- 必要时配合链接脚本/导出控制（平台相关）

这点比 C/C++ “默认符号 + 链接器可见”更明确一些。

## 7. 运行时体量与底层依赖

你问的核心：Rust 最终二进制是否像 Go 那样带巨大 runtime？

结论先说：

- Rust 没有 Go 那种 GC + 调度器 runtime 体量。
- Rust 也不是“啥都没有”；会包含语言运行所需最小支持（如 panic/展开策略、分配器相关组件、`std` 功能）。
- 是否依赖 `libc` 取决于目标和链接方式：
- 常见 Linux + `std` 场景下会链接到 `libc`（动态或静态）。
- 也可以走 `musl` 静态链接，减少部署依赖。
- `no_std` 场景可以进一步去掉标准库，走更裸的系统/平台接口。

当前仓库实测（`target/debug/04_trait`）：

- 是 ELF 动态链接可执行文件。
- `ldd` 显示依赖 `libc.so.6` 与 `libgcc_s.so.1`。

## 8. 系统调用路径：会不会像 Go 那样绕 libc

默认 `std` 路径下，Rust 许多 OS 功能最终会通过 libc 封装进入内核。

- 你可用 `std`（最省心）。
- 也可用 `libc`/`nix`/`rustix` 更靠近系统调用层。
- 是否直 syscall、是否经 libc，与所用库和目标平台有关，不是语言强制唯一路径。

工程实践建议：

- 先用 `std` + 生态库建立功能。
- 性能/可控性要求高时，再下沉到 `rustix`/`libc`，并用基准测试验证收益。

## 9. 内核级 / Bare-Metal 能力：为什么 Rust 能做

你这点判断是对的：Rust 的确具备内核级和裸机开发能力，关键是它可以脱离 `std`。

核心开关：

- `#![no_std]`：不链接标准库，去掉对常规 OS 运行时设施的依赖。
- `#![no_main]`：不用默认入口，自己定义启动路径（常配合启动汇编/链接脚本）。

这意味着：

- 可以像 C/汇编一样，精确控制启动代码、内存布局、中断向量、链接地址。
- 能直接面向硬件/平台 ABI 做最小运行环境（runtime）构建。
- 很适合内核、bootloader、固件、RTOS 组件等场景。

和 C/汇编混合构建的常见模式：

- Rust + 启动汇编（`start.S`）+ 链接脚本（`linker.ld`）。
- Rust 导出 `extern "C"` 符号给 C 调用，或 C 导出符号给 Rust 调用。
- 构建层用 Cargo 驱动，必要时配 `build.rs`、`cc` crate 或外部链接器参数。

和 Go 的关键区别（系统级视角）：

- Go 常规模型强依赖自带 runtime（调度器、GC、栈管理），裸机/内核路线更受限。
- Rust 无强制 GC runtime，可按目标裁剪到极小运行支撑，贴近 C 的可控性。

补一句边界：

- “Rust 没有大 runtime”不等于“零运行支持”。`no_std` 场景仍需你显式处理 panic、分配器、启动流程等基础设施。

## 10. Bare-Metal 对比 C：系统程序员要额外注意什么

你可以把 Rust 裸机开发理解为“C 的控制力 + 更强的编译期约束”，但有一些新增注意点。

### 10.1 启动与入口责任（和 C 基本同级）

- 你仍要负责入口、栈、段初始化、时钟/中断早期初始化。
- 常见结构仍是：`start.S` + `linker.ld` + 语言入口函数。
- Rust 侧通常用 `#![no_main]` + 自定义入口符号对接启动代码。

### 10.2 panic / unwind 策略（Rust 特有显式项）

- `no_std` 必须提供 `#[panic_handler]`。
- 裸机通常使用 `panic = "abort"`，避免 unwind 依赖和体积增长。
- 和 C 相比：C 多数错误处理是约定返回码；Rust 会要求你明确 panic 行为边界。

### 10.3 堆分配不是默认可用

- 用到 `Box/Vec/String` 等堆对象前，要先提供全局分配器。
- 很多固件/内核早期阶段会刻意避免堆，只用静态区和栈。
- 这和 C 的 `malloc` 可用性问题本质相同，但 Rust 会在类型层面更早暴露需求。

### 10.4 ABI 与 FFI 边界要刻意管理

- 跨语言边界统一用 `extern "C"`，避免 Rust ABI 假设。
- 导出符号常配 `#[no_mangle]`，并明确符号可见性。
- `repr(C)` 用于与 C 共享结构体布局；否则布局不应假定兼容。

### 10.5 内存模型与 MMIO 访问纪律

- 做寄存器访问时要用 `read_volatile` / `write_volatile` 或等价封装。
- 中断共享数据需用原子/临界区，不要把 `static mut` 当“随便可写全局”。
- Rust 的借用规则能减少别名错误，但并不替代硬件同步原语。

### 10.6 链接与产物检查（和 C 一样要做）

- 检查 map 文件、节布局、符号表、重定位结果。
- 验证最终镜像格式（ELF/bin/hex）与烧录地址一致。
- 用 `objdump/readelf/nm` 做最终产物核查，流程和 C 项目一致。

### 10.7 常见“从 C 迁移到 Rust 裸机”的误区

- 误区：以为“没有 runtime”就完全没有语言支持成本。
- 现实：你要显式补齐 panic、分配器、入口、目标配置等基础设施。
- 误区：借用检查会自动解决并发与中断竞态。
- 现实：竞态控制仍靠架构原语（屏蔽中断、原子、锁、内存屏障）。

### 10.8 实战建议（按你背景）

- 先做一个 `no_std + no_main` 最小可启动样例（串口打印/死循环）。
- 再引入中断和 MMIO 封装，最后才引入堆分配。
- 保持“C 汇编启动层 + Rust 业务层”分层，这样迁移阻力最小。

## 11. 业界进展与常用库：Bare-Metal 现在到哪一步了

你的判断“Rust 在 bare-metal 已经大展宏图”可以成立，但更精确地说是：

- 在工具链、基础抽象、调试工具、部分垂直行业已进入可落地阶段。
- 在“全行业替代 C”层面仍是渐进迁移，常见形态是 Rust/C 混合栈。

### 11.1 可以认为比较成熟的基座

- 官方嵌入式社区与文档体系：Rust Embedded Working Group、Embedded Rust Book。
- 生态稳定基石：`embedded-hal` 1.0（驱动可移植 trait 标准）。
- Cortex-M 启动运行时：`cortex-m-rt`（最小启动/runtime）。
- 调试与烧录工具：`probe-rs`（CLI、DAP、VSCode、GDB 流程）。

### 11.2 工程上高频使用的库（按层分）

- 启动与目标层：
- `cortex-m-rt`、`riscv-rt`、芯片厂 HAL（例如 `esp-hal`）。
- 硬件抽象层：
- `embedded-hal`、`embedded-hal-async`、`embedded-hal-bus`。
- 并发与任务模型：
- `embassy`（async embedded 框架）、`rtic`（中断驱动并发框架）。
- 调试与日志：
- `probe-rs`、`defmt`（紧凑日志编码，常见于资源受限场景）。
- 无堆数据结构：
- `heapless`（定长容器，避免运行时堆依赖）。

### 11.3 业界落地信号（你关心“经验”）

- 功能安全方向：
- Ferrocene 提供面向安全关键场景的 Rust 工具链与合规资料（汽车/工业/医疗）。
- 嵌入式 OS 方向：
- Tock OS（Rust 编写的嵌入式操作系统）长期持续演进。
- 产业 MCU 生态：
- Espressif 官方持续推进 `esp-rs`，覆盖 `no_std` 与 `std` 两条开发路线。
- 深嵌入实践：
- Oxide 的 Hubris 展示了 Rust 在高可靠 MCU 系统中的任务隔离与故障恢复模型。

### 11.4 仍需谨慎的现实边界

- 芯片厂支持度不均：有的厂商 Rust HAL 完整，有的仍以社区维护为主。
- 周边工具链成熟度差异：量产链路（认证、诊断、产测）常需与现有 C 体系深度集成。
- 人才与代码审查模型：团队需要同时掌握 Rust unsafe 边界与传统底层调试方法。

### 11.5 给系统程序员的实操建议

- 优先从“增量替换”开始：
- 保留 C/汇编启动和 BSP，把新驱动/协议栈用 Rust 写。
- 优先选择“生态强势芯片”做首批项目（如 Cortex-M 主流板卡、ESP32-C3/C6）。
- 在项目早期就固定工具链版本（Rust toolchain + probe + linker）并冻结构建镜像。

## 12. 配套代码怎么读（`src/bin/05_build.rs`）

- 目的：把“构建系统概念”落到可运行代码，不是语法炫技。
- 覆盖点：
  - `cfg!(target_os/target_arch)`：条件编译认知入口。
  - `cfg!(debug_assertions)`：debug/release 差异入口。
  - `CARGO_PKG_*` 环境变量：Cargo 与构建元数据注入。
  - 注释里解释了 bin crate、默认链接路径与 no_std 分流。
- 运行命令：
  - `cargo run --bin 05_build`
- 先建立一套可重复的烧录/调试/回归流程，再扩大 Rust 代码占比。

## 12. 参考链接（便于你继续深挖）

- Embedded Working Group: [rust-embedded.org](https://rust-embedded.org/)
- Embedded Rust Book: [docs.rust-embedded.org/book](https://docs.rust-embedded.org/book/)
- `embedded-hal` v1.0 发布说明: [blog.rust-embedded.org/embedded-hal-v1](https://blog.rust-embedded.org/embedded-hal-v1/)
- `cortex-m-rt` docs: [docs.rs/cortex-m-rt](https://docs.rs/crate/cortex-m-rt/latest)
- Embassy: [github.com/embassy-rs/embassy](https://github.com/embassy-rs/embassy)
- RTIC: [docs.rs/rtic](https://docs.rs/rtic/latest/rtic/)
- probe-rs: [probe.rs/docs](https://probe.rs/docs/)
- Tock OS: [tockos.org](https://tockos.org/)
- Hubris: [hubris.oxide.computer](https://hubris.oxide.computer/)
- Espressif Rust docs: [docs.espressif.com/projects/rust](https://docs.espressif.com/projects/rust/)
- Ferrocene: [ferrocene.dev/en](https://ferrocene.dev/en)
- Linux kernel Rust docs: [docs.kernel.org/rust/index.html](https://docs.kernel.org/rust/index.html)
