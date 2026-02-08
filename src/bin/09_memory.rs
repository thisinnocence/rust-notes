use std::mem::{align_of, size_of};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Header {
    magic: u16,
    version: u8,
    flags: u8,
    len: u32,
}

fn parse_header(buf: &[u8]) -> Option<Header> {
    if buf.len() < 8 {
        return None;
    }

    Some(Header {
        magic: u16::from_le_bytes([buf[0], buf[1]]),
        version: buf[2],
        flags: buf[3],
        len: u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
    })
}

fn main() {
    println!("Header size={}, align={}", size_of::<Header>(), align_of::<Header>());

    let raw = [0x34, 0x12, 0x01, 0x02, 0x10, 0x00, 0x00, 0x00];
    let h = parse_header(&raw).expect("valid header bytes");
    println!("parsed header: {:?}", h);
}
