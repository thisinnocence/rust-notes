/// SAFETY TEMPLATE:
/// 1) 前提：调用方必须满足什么条件？
/// 2) 保证：当前函数内部如何验证这些条件？
/// 3) 结果：为什么这次 unsafe 操作不会产生 UB？
unsafe fn read_u32_unaligned(ptr: *const u8) -> u32 {
    let p = ptr as *const u32;
    // SAFETY:
    // 1) 调用方保证 ptr 至少有 4 字节可读。
    // 2) 使用 read_unaligned 允许未对齐地址。
    unsafe { std::ptr::read_unaligned(p) }
}

fn read_u32_le_at(buf: &[u8], offset: usize) -> Option<u32> {
    if offset.checked_add(4)? > buf.len() {
        return None;
    }

    let p = buf.as_ptr().wrapping_add(offset);

    // SAFETY:
    // 1) 长度检查保证 [offset, offset+4) 在 buf 内。
    // 2) read_u32_unaligned 允许未对齐地址。
    let v = unsafe { read_u32_unaligned(p) };
    Some(u32::from_le(v))
}

fn main() {
    let data = [0x78, 0x56, 0x34, 0x12, 0xEF, 0xBE, 0xAD, 0xDE];

    let a = read_u32_le_at(&data, 0).expect("offset 0 should be valid");
    let b = read_u32_le_at(&data, 4).expect("offset 4 should be valid");
    let c = read_u32_le_at(&data, 6);

    println!("a=0x{a:08x}, b=0x{b:08x}, c={c:?}");
}

#[cfg(test)]
mod tests {
    use super::read_u32_le_at;

    #[test]
    fn test_reads_valid_offsets() {
        let data = [0x78, 0x56, 0x34, 0x12, 0xEF, 0xBE, 0xAD, 0xDE];
        assert_eq!(read_u32_le_at(&data, 0), Some(0x1234_5678));
        assert_eq!(read_u32_le_at(&data, 4), Some(0xDEAD_BEEF));
    }

    #[test]
    fn test_rejects_oob() {
        let data = [1_u8, 2, 3];
        assert_eq!(read_u32_le_at(&data, 0), None);
    }
}
