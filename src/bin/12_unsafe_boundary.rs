fn read_u32_le_at(buf: &[u8], offset: usize) -> Option<u32> {
    if offset.checked_add(4)? > buf.len() {
        return None;
    }

    // 将 unsafe 缩到最小：只在真正需要原始指针读时使用。
    let p = buf.as_ptr().wrapping_add(offset) as *const u32;

    // SAFETY:
    // 1) 上面的长度检查保证 [offset, offset+4) 在 buf 内。
    // 2) 使用 read_unaligned，允许 p 非对齐。
    // 3) p 来源于有效切片指针，读取 4 字节不会越界。
    let v = unsafe { std::ptr::read_unaligned(p) };
    Some(u32::from_le(v))
}

fn main() {
    let data = [0x78, 0x56, 0x34, 0x12, 0xEF, 0xBE, 0xAD, 0xDE];

    let a = read_u32_le_at(&data, 0).expect("offset 0 should be valid");
    let b = read_u32_le_at(&data, 4).expect("offset 4 should be valid");
    let c = read_u32_le_at(&data, 6);

    println!("a=0x{a:08x}, b=0x{b:08x}, c={c:?}");
}
