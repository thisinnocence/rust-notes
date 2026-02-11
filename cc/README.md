# cc - Minimal C++17 + Rust FFI Demo

这个目录演示一个最小双向混合工程：

- C++17 可执行程序调用 Rust 导出的函数（`rust_add`）。
- Rust 函数再反向调用 C++ 导出的函数（`cpp_mul`）。
- CMake 统一驱动 C++ 编译与 Cargo 构建 Rust 动态库。

## 目录

- `main.cpp`：C++ 入口；既调用 Rust，也提供 `cpp_mul` 给 Rust 用。
- `rustlib/`：Rust 库（`cdylib`）；导出函数并调用 C++ 符号。
- `CMakeLists.txt`：先构建 Rust，再链接 C++。

## 构建与运行

```bash
cmake -S cc -B cc/build
cmake --build cc/build
./cc/build/cpp_calls_rust
```

预期输出示例：

```text
cpp -> rust: 7 + 35 = 42
cpp -> rust -> cpp: 7 * 35 = 245
```

## 学习要点

- 跨语言互调最小条件：`extern "C"` + 稳定符号名（`#[no_mangle]`）。
- 双向互调时，链接阶段需保证符号可见（这里通过 `-Wl,--export-dynamic`）。
- Rust 作为库由 Cargo 构建，C++ 由 CMake 编译，工程可分层维护。
- 先跑通最小函数，再逐步扩展到结构体、错误码、内存所有权约定。
