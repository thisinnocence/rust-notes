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

- C/C++ 常见痛点：头文件依赖、宏展开、副本编译单元、链接顺序、ABI 差异、工具链碎片（Make/CMake/Bazel 等并存）。
- Rust 的改进点：
- 模块系统由语言和包管理统一；无头文件二义性。
- 依赖图由 Cargo 统一解析，编译单元是 crate，不靠手工拼链接顺序。
- 语义检查更早，很多错误在编译期前置。
- 默认约定目录结构，减少“先搭构建脚手架”成本。

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
