use std::mem::{align_of, size_of};

#[derive(Debug)]
struct RustHeader {
    magic: u16,
    version: u8,
    flags: u8,
    len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct CHeader {
    magic: u16,
    version: u8,
    flags: u8,
    len: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct PackedHeader {
    magic: u16,
    version: u8,
    flags: u8,
    len: u32,
}

fn parse_c_header(buf: &[u8]) -> Option<CHeader> {
    if buf.len() < 8 {
        return None;
    }

    Some(CHeader {
        magic: u16::from_le_bytes([buf[0], buf[1]]),
        version: buf[2],
        flags: buf[3],
        len: u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
    })
}

fn parse_packed_len_unaligned(buf: &[u8]) -> Option<u32> {
    if buf.len() < size_of::<PackedHeader>() {
        return None;
    }

    let base = buf.as_ptr();
    let len_offset = 4usize;
    let p = base.wrapping_add(len_offset) as *const u32;

    // SAFETY:
    // 1) len_offset..len_offset+4 在 buf 范围内。
    // 2) 使用 read_unaligned 允许未对齐读取。
    let le = unsafe { std::ptr::read_unaligned(p) };
    Some(u32::from_le(le))
}

fn main() {
    println!(
        "RustHeader size={}, align={}",
        size_of::<RustHeader>(),
        align_of::<RustHeader>()
    );
    println!(
        "CHeader    size={}, align={}",
        size_of::<CHeader>(),
        align_of::<CHeader>()
    );
    println!(
        "Packed     size={}, align={}",
        size_of::<PackedHeader>(),
        align_of::<PackedHeader>()
    );

    let raw = [0x34, 0x12, 0x01, 0x02, 0x10, 0x00, 0x00, 0x00];
    let h = parse_c_header(&raw).expect("valid header bytes");
    let len = parse_packed_len_unaligned(&raw).expect("valid packed bytes");
    println!("parsed c header: {:?}, packed_len={len}", h);

    // 防止 RustHeader 被优化到未使用告警，顺带展示字段读取。
    let r = RustHeader {
        magic: h.magic,
        version: h.version,
        flags: h.flags,
        len: h.len,
    };
    println!(
        "rust header fields: {} {} {} {}",
        r.magic, r.version, r.flags, r.len
    );
}
