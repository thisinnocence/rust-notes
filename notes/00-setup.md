# 00 - 环境与工程结构

背景假设：读者具备系统编程经验，熟悉 C/C++、QEMU、Linux 与高性能用户态程序。

本仓库目标：

- `src/` 放可运行 Rust 代码（小颗粒示例）。
- `notes/` 放与 C/C++ 背景对照的学习笔记。
- `demos/` 放独立练习和实验代码（不影响主线章节代码）。

术语补充（Cargo 语境）：

- `package`：由根目录 `Cargo.toml` 描述的工程单元。
- `crate`：Rust 编译单元（`lib crate` 或 `bin crate`）。
- `src/bin/*.rs` 属于 Cargo 默认自动发现路径，一般不需要在 `Cargo.toml` 里写 `[[bin]]`。
- 当前仓库可理解为“一个 package，包含多个 bin crate”，即 `src/bin/*.rs` 下每个可执行文件都是一个 bin crate。
- 诸如 `demos/*.rs` 这类非默认路径文件，若要用 `cargo run --bin ...` 运行，需要在 `Cargo.toml` 里显式配置 `[[bin]]` 和 `path`。

不写 `Cargo.toml` 目标配置时，可把 Cargo 理解为“按约定解析目录”：

- `src/main.rs`：默认可执行目标（bin）。
- `src/lib.rs`：默认库目标（lib）。
- `src/bin/<name>.rs`：额外可执行目标，目标名默认是 `<name>`。
- 其他目录（如 `demos/`）不会自动当作可执行目标，除非显式写 `[[bin]]` 指向它。

自动编译并运行全部示例：

- `./scripts/run-all.sh`

说明：

- `run-all.sh` 默认只运行 `src/bin/*.rs` 的主线示例，不包含 `demos/`。

单个示例编译/运行（以 `01_basics.rs` 为例）：

- 只编译：`cargo build --bin 01_basics`
- 编译并运行：`cargo run --bin 01_basics`

Markdown 规范检查：

- `./scripts/lint-md.sh`

小结：

- 可先用 `run-all` 跑通全量示例，再按章节逐个深入学习对应 `notes` 与 `src/bin`。
