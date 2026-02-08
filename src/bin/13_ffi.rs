#[unsafe(no_mangle)]
pub extern "C" fn add_u32(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    // 这里直接调用 C ABI 导出的函数。
    // 实际跨语言时，C 代码会通过同名符号调用它。
    let v = add_u32(40, 2);
    println!("ffi add result={v}");
}
