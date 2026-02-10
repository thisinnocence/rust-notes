# 06 - 构建与生态

本章把两件常被分开讲的事合在一起：

- 构建系统如何影响工程效率与可维护性。
- 包管理与生态如何决定语言是否能长期落地。

对系统程序员来说，这两者本质上是同一个问题：能不能稳定交付、稳定演进。

## 1. 先从 `rustc` 开始：编译器的最小工作流

先把 Cargo 放一边，Rust 的底层构建事实是：最终都要落到 `rustc`。

`rustc` 做的事很聚焦：

- 输入一个 crate（可执行或库）及其编译参数。
- 完成语法/类型检查、单态化、优化、代码生成、链接。
- 产出二进制、库或中间产物（如汇编、IR、元数据）。

这里先把 `crate` 说清楚（后面会反复出现）：

- 一句话：`crate` 是 Rust 的“编译与发布基本单元”，可以是可执行程序，也可以是库。
- 类比 C：接近“一个库/一个程序 + 一组源码 + 构建边界”，但 Rust 不依赖 `.h` 头文件体系。
- 类比 Go：接近 module + package 的组合体里“可独立构建发布”的那层。
- 类比 Python：比单个 package 更偏“可编译产物单元”，不是纯解释期导入单元。

只按 `rustc` 理解 `crate root`（先不引入 Cargo）：

- `crate` 是一次编译的整体单元，`crate root` 是这次编译的根文件。
- 没有专门语法标记“我是 root”；是否为 root 取决于你把哪个文件交给 `rustc`。
- 被 root 通过 `mod` 纳入的文件是模块文件，不是 root。
- 同一个文件在不同命令下身份会变：`rustc main.rs` 时 `main.rs` 是 root；`rustc util.rs` 时 `util.rs` 也可成为另一轮编译的 root。

和物理文件/目录的关系要分清：

- crate 的边界是“crate root 文件 + 由 `mod` 展开的模块树”，不是“某个目录”本身。
- 同一目录可以有多个 crate（例如 `src/main.rs` 和 `src/bin/*.rs`）。
- 同一 crate 也不要求把所有源码都塞在一个目录层级里（虽然后续维护上通常建议按约定目录组织）。
- 没被该 crate 的模块树引用到的 `.rs` 文件，不会自动参与这个 crate 的编译。

Rust 选择 `crate` 作为核心边界，是为了把“模块可见性、类型检查、依赖解析、链接产物”放到同一层统一建模，减少 C/C++ 那种“源码组织一套、构建系统另一套”的割裂。

这也带来一个直接结果：Rust 自身代码基本不需要 C/C++ 那套头文件机制。接口由模块可见性（`pub`）和实际定义本身表达，不再靠 `.h` 做声明同步；但如果做 C ABI 互操作，仍然会在边界处维护/生成头文件给 C 侧使用。

最小示例（单文件）：

```bash
rustc hello.rs
./hello
```

在本仓库里可直接体验：

```bash
rustc src/bin/01_basics.rs -o /tmp/01_basics
/tmp/01_basics
```

常用参数（先建立手感）：

- `-o app`：指定输出文件名。
- `--edition 2021`：指定语言版本。
- `-C opt-level=0/2/3`：优化等级（`cargo build --release` 本质会传更高优化）。
- `-g`：保留调试信息。
- `--emit=asm` / `--emit=llvm-ir`：观察编译中间产物。
- `--target <triple>`：交叉编译目标（如 `x86_64-unknown-linux-gnu`）。

如果只用 `rustc`，你确实可以像 `gcc + make` 一样自己组织与链接，只是模型不完全相同：

- 同一个 crate 内部：靠 `mod`/`use` 组织源码，不需要 C 的头文件声明分离。
- 跨 crate 依赖：先编译库，再在主程序里用 `--extern` 指定依赖；`-L` 指定库搜索路径。
- 链接 C 库：仍可用 `-L native=...` + `-l static=...` / `-l dylib=...`。
- 用 Makefile：完全可行，但依赖图、增量、profile、测试/lint 流程都要你自己维护。

最小类比（手工两步）：

```bash
# 1) 先编译库 crate
rustc --crate-type=rlib lib.rs -o build/libmylib.rlib

# 2) 再编译可执行并链接这个库
rustc main.rs --extern mylib=build/libmylib.rlib -L dependency=build -o build/app
```

和 `gcc -I` 的核心差异：

- Rust 没有“头文件搜索路径”这层；类型/函数签名在 crate 元数据里，不靠 `.h` 暴露。
- `-L` 在 Rust 里主要是“库搜索路径”，不是“源码接口搜索路径”。
- crate 内模块文件通常由 crate root 统一编译，不是每个 `.rs` 都像 `.c` 那样先各自产 `.o`。
  这里的 crate root 指“根文件”，不是入口函数：
  - `bin crate` 通常是 `src/main.rs`（或 `[[bin]].path` 指定文件）。
  - `lib crate` 通常是 `src/lib.rs`（或 `[lib].path` 指定文件）。
  - `fn main()` 只是可执行程序入口；crate root 是模块树与编译单元的根。

当工程变成“多 crate + 第三方依赖 + 多目标构建”时，纯 `rustc` 也能做，但要手工处理：

- 依赖下载与版本解析。
- `--extern` / `-L` 依赖传递与链接参数。
- 增量缓存、构建目录组织、profile 区分。
- 测试、格式化、lint、发布等外围流程。

这就是过渡点：`rustc` 解决“编译”，Cargo 解决“工程”。

再补一句常见误解：`rustc` 不完全等同于“`gcc` 必须把所有源码文件逐个列出来”。

- C/GCC 常见模型是“每个 `.c` 一个编译单元”，先产 `*.o`，再统一链接。
- Rust/rustc 常见模型是“每个 crate 一个编译单元入口（crate root）”，模块文件通过 `mod` 声明并入同一 crate 编译。
- 也就是说，同 crate 内你通常只指定 root 文件，不需要把每个 `.rs` 都写在命令行上。
- 跨 crate 仍要显式声明依赖（如 `--extern`、`-L`）；最终由 `rustc` 驱动链接器产出库或可执行文件。

本质差异是编译边界不同：C 以“源文件”做默认边界，Rust 以“crate”做默认边界。

## 2. Rust 的工程基础模型

为什么 Rust 能自然长出 Cargo，而 C/C++ 很难出现“单一事实标准”，底层在于可统一性不同：

- Rust 从语言层就有明确的包边界（crate）、可见性规则（`pub`）和依赖声明入口，工具可以稳定读取同一套语义。
- Rust 不依赖头文件文本拼接模型，接口和实现在同一语义系统里，减少“声明与实现漂移”带来的构建不确定性。
- crate 元数据可机器化描述（名称、版本、特性、依赖图），天然适合统一做解析、锁定、缓存和复现构建。
- C/C++ 历史包袱更重：源码边界、构建系统、包管理、ABI 约定长期分裂（Make/CMake/Bazel + vcpkg/conan/系统包管理并存），很难收敛到单一工具链入口。

所以不是“C/C++ 做不到包管理”，而是“语言与历史生态没有给出一个像 crate 这样天然统一的工程边界”，导致难以形成 Cargo 这种默认共识。

这和 Go 的确很像，但不完全一样：

- 相似点：都把“语言语义 + 构建/依赖工作流”做了强绑定，降低工程冷启动成本。
- Go 的核心边界更偏 `module/package`；Rust 的核心边界是 `crate`（可执行与库统一建模更强）。
- Go 在服务端工程路径更直接；Rust 额外覆盖 `no_std`/FFI/底层库产物（`staticlib`/`cdylib`）这类系统场景。
- 两者都比 C/C++ 更容易形成统一包管理，但 Rust 的设计目标对“跨层交付（应用到系统库）”约束更细。

为什么这些特质会让很多 Rust 项目“可以不写 Makefile/CMake”：

- 依赖解析、构建顺序、profile（debug/release）、目标产物约定都被 Cargo 内建统一，避免每个项目重复造构建脚本。
- 语言级模块/可见性与构建边界一致，工具不需要额外维护一套“源码组织到构建图”的映射层。
- 锁文件 + 语义化版本 + 可缓存构建目录，让“可复现 + 增量构建 + CI 稳定性”成为默认能力，而不是手工拼装能力。
- 因为默认路径足够强，生态可以围绕同一入口沉淀（`build/test/fmt/lint/publish`），包管理自然变强且可组合。

注意这不是绝对“抛弃” Make/CMake：当项目涉及复杂 C/C++/汇编混编、特殊链接脚本或跨平台系统集成时，仍可能引入它们；但在纯 Rust 或以 Rust 为主的工程里，Cargo 通常已覆盖主流程。

即使在 Linux，工程实践里通常也是 `cargo` 作为日常入口（`build/run/test`），`rustc` 作为底层编译器被 Cargo 调用；只有极简或高度定制链路才会直接手写 `rustc`/Makefile。

Rust 以 Cargo 统一“包管理 + 构建 + 工作流”，并把 `rustc` 包装成可复用流水线。

Cargo 诞生时间线:

- 2014-11-20：Cargo 与 crates.io 对外宣布可用（Rust 官方博客发布 `Cargo: Rust's community crate host`）。
- 2015-05-15：Rust 1.0 正式发布，Cargo/crates.io 随 1.0 进入官方稳定叙事，成为默认工程工作流。
- 到今天（2026-02-10），官方安装路径（`rustup`）仍默认同时安装 `rustc` 与 `cargo`，延续这套标准入口。

Cargo 的核心目录与文件：

- `Cargo.toml`：依赖、版本、构建元数据。
- `Cargo.lock`：锁定解析结果，保证复现构建。
- `src/main.rs` / `src/lib.rs` / `src/bin/*.rs`：目标入口约定。
- `target/`：构建产物与缓存。

常用命令：

- `cargo check`：语义检查，不产最终可执行文件。
- `cargo build`：debug 构建。
- `cargo build --release`：优化构建。
- `cargo run --bin <name>`：构建并运行指定目标。

关于上面的目录与文件，再补一个关键认知：

- `Cargo.toml` 是强约束，没有它 Cargo 无法识别包。
- `src/main.rs` / `src/lib.rs` / `src/bin/*.rs` 是默认约定，不是硬编码死规则。
- `target/` 也是默认输出目录，可通过 `CARGO_TARGET_DIR` 或 `.cargo/config.toml` 改。
- `Cargo.lock` 是锁文件：应用项目通常提交；库项目是否提交取决于团队策略。

例如可在 `Cargo.toml` 显式覆盖默认入口路径：

```toml
[lib]
path = "rust/core.rs"

[[bin]]
name = "tool"
path = "app/main.rs"
```

## 3. 对比 C/C++：复杂度转移而不是消失

| 维度 | C/C++ 常见成本 | Rust/Cargo 改进 |
| --- | --- | --- |
| 模块组织 | 头文件依赖与可见性问题 | 语言模块 + crate 统一模型 |
| 依赖链接 | 链接顺序、脚本、工具链组合复杂 | 依赖图由 Cargo 统一解析 |
| 语义发现 | 许多错误暴露较晚 | 更多错误前置到编译期 |
| 工程脚手架 | Make/CMake/Bazel 等割裂 | 默认约定强，冷启动快 |

要点：Rust 不是“没有复杂度”，而是把复杂度从“构建拼装”前移到“类型与语义建模”。

## 4. 对比 Go / Node.js / Python

| 语言 | 运行模型 | 构建与依赖模型 | 对系统工程的意义 |
| --- | --- | --- | --- |
| Go | 有明显 runtime（调度/GC） | `go mod` 一体化较好 | 服务端效率高，裸机/强约束场景受限 |
| Node.js | 解释执行 + VM | npm/pnpm + lockfile | 生态强，部署依赖 runtime |
| Python | 解释执行 + 虚拟环境 | pip/venv/lock 方案多样 | 迭代快，性能热点需下沉 |
| Rust | AOT 编译，无 GC/VM runtime | Cargo + crates.io + lockfile | 低层可控 + 工程化统一 |

## 5. 产物与链接认知

Rust 常见产物类型：

- `bin`：可执行文件。
- `rlib`：Rust 静态库中间产物。
- `dylib`：Rust ABI 动态库。
- `cdylib`：C ABI 动态库（跨语言集成常用）。
- `staticlib`：C ABI 静态库（类似 `.a`）。

符号导出边界：

- `pub` 仅是 Rust 模块可见性，不等于 C ABI 导出。
- 对外稳定导出通常要 `extern "C"` + `#[no_mangle]` + `#[repr(C)]`（结构体）。

## 6. 运行时体量与系统调用路径

核心判断：

- Rust 没有 Go 那种“较大统一 runtime（GC + 调度器）”。
- 但 Rust 也不是“零运行支持”；`panic`、分配器、启动路径仍是运行基础设施。
- 常见 Linux `std` 程序会通过 libc 进入 syscall（具体取决于目标与库选型）。

可选路径：

- `std`：开发效率高，适合大多数用户态程序。
- `libc`/`rustix` 下沉：更接近系统调用边界。
- `no_std`：面向裸机/内核等极简环境。

## 7. `no_std` 与 Bare-Metal：为什么 Rust 可做系统底层

核心开关：

- `#![no_std]`：不链接标准库。
- `#![no_main]`：不用默认入口，自定义启动流程。

和 C/汇编混合常见模式：

- `start.S` + `linker.ld` + Rust 入口。
- C/Rust 通过 `extern "C"` 做 ABI 边界协作。

## 8. 裸机相对 C 的新增注意点

- 必须显式提供 `#[panic_handler]`（通常配 `panic = "abort"`）。
- 需要明确分配器策略（不使用堆时不引入堆类型）。
- MMIO 与中断共享状态仍要遵守 volatile/原子/临界区规则。
- 产物验证（`readelf`/`objdump`/map 文件）仍是硬要求。

结论：Rust 提供的是更强编译期约束，不会替代底层工程责任。

## 9. 为什么“包管理”决定生态生命力

可持续生态必须同时满足：

- 依赖可发现（仓库与索引机制）。
- 依赖可复现（lockfile）。
- 依赖可治理（版本与安全更新可控）。

这也是 C/C++ 历史上最容易分裂的地方：语言强、但工程基础设施长期不统一。

## 10. Rust 生态闭环的工程价值

Rust 的优势不是单点，而是组合：

- Cargo（构建与依赖）
- crates.io（分发）
- `rustfmt` / `clippy`（风格与静态检查）
- `test` / `bench`（验证）

这让“代码资产复利”更容易发生：可复用、可升级、可治理。

## 11. Rust 在系统领域的落地现状

| 方向 | 现状判断 |
| --- | --- |
| CLI 工具 | 已是 Rust 的成熟优势场景之一 |
| Bare-metal/嵌入式 | 基座可用，芯片生态成熟度不均 |
| Linux/QEMU/虚拟化周边 | 持续推进，典型是渐进引入而非一次替换 |
| Python/JS 基建 | “上层语言不变，性能热点下沉 Rust”已常态化 |

## 12. 为什么很多迁移选 Rust 而不是 Go 或 C++ 重写

| 方案 | 优势 | 常见边界 |
| --- | --- | --- |
| 继续 C/C++ | 性能与既有资产延续 | 默认内存安全约束弱，治理成本高 |
| 改 Go | 开发效率高、工程约定统一 | runtime 模型不总适配低层强约束 |
| 改 Rust | 低层可控 + 内存安全 + 工程化统一 | 前期学习与建模成本更高 |

现实通常是混合路线：旧系统保留稳定部分，新增或高风险模块先 Rust 化。

## 13. AI 协作时代的附加收益

Rust 对 AI 协作开发的价值在于：

- 约束显式：类型与所有权让意图更可检查。
- 反馈密集：编译器能快速暴露边界错误。
- 流程统一：Cargo/format/lint/test 易接入自动化流水线。

不是“AI 自动写完”，而是“AI 产出更容易被工具链收敛到可交付代码”。

## 14. 配套代码

对应示例：[`../src/bin/06_build.rs`](../src/bin/06_build.rs)。

- 用最小运行代码把构建信息可视化：
  - `cfg!(target_os/target_arch)`：目标平台分支。
  - `cfg!(debug_assertions)`：debug/release 差异。
  - `CARGO_PKG_*`：Cargo 注入元数据。
- 文件里注释解释了 bin crate、链接边界与 no_std 分流认知。
- 运行：
  - `cargo run --bin 06_build`

## 15. 参考链接

- Linux kernel Rust docs: <https://docs.kernel.org/rust/index.html>
- QEMU Rust docs: <https://www.qemu.org/docs/master/devel/rust.html>
- Embedded Rust Book: <https://docs.rust-embedded.org/book/>
- `embedded-hal` 1.0: <https://blog.rust-embedded.org/embedded-hal-v1/>
- probe-rs: <https://probe.rs/docs/>
- Ubuntu: <https://discourse.ubuntu.com/t/carefully-but-purposefully-oxidising-ubuntu/56995>
- uv: <https://github.com/astral-sh/uv>
- pydantic v2: <https://pydantic.dev/articles/pydantic-v2>
- SWC: <https://swc.rs/>
