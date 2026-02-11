/// 导出给 C/C++ 调用的最小函数。
///
/// 关键点：
/// - `extern "C"`：使用 C ABI，便于跨语言互调。
/// - `#[no_mangle]`：禁用符号改名，导出稳定符号名 `rust_add`。
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

// 这个符号由 C++ 侧实现，Rust 这里声明后即可调用。
extern "C" {
    fn cpp_mul(a: i32, b: i32) -> i32;
}

/// C++ 调 Rust，再由 Rust 调回 C++（双向 FFI 最小演示）。
#[no_mangle]
pub extern "C" fn rust_mul_via_cpp(a: i32, b: i32) -> i32 {
    // Safety: `cpp_mul` 由同进程 C++ 程序提供，签名双方保持一致。
    unsafe { cpp_mul(a, b) }
}
