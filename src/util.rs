use core::mem::transmute;

#[inline(always)]
pub const fn folded_multiply(s: u64, by: u64) -> u64 {
    let result = (s as u128).wrapping_mul(by as u128);
    ((result & 0xffff_ffff_ffff_ffff) as u64) ^ ((result >> 64) as u64)
}

#[inline(always)]
pub const fn read_small(head: *const u8, tail: *const u8, length: usize) -> [u64; 2] {
    if length >= 4 {
        [read_u32(head) as u64, read_last_u32(tail) as u64] // len 4-8
    } else if length >= 2 {
        unsafe { [read_u16(head) as u64, *head.add(length - 1) as u64] } // len 2-3
    } else if length > 0 {
        unsafe { [*head as u64, *head as u64] }
    } else {
        [0, 0]
    }
}

#[inline(always)]
pub const fn read_u128(data: *const u8) -> u128 {
    unsafe { transmute(*(data as *const [u8; 16])) }
}

#[inline(always)]
pub const fn read_last_u128(data: *const u8) -> u128 {
    unsafe { transmute(*(data.sub(16) as *const [u8; 16])) }
}

#[inline(always)]
pub const fn read_u64(data: *const u8) -> u64 {
    unsafe { transmute(*(data as *const [u8; 8])) }
}

#[inline(always)]
pub const fn read_last_u64(data: *const u8) -> u64 {
    unsafe { transmute(*(data.sub(8) as *const [u8; 8])) }
}

#[inline(always)]
pub const fn read_u32(data: *const u8) -> u32 {
    unsafe { transmute(*(data as *const [u8; 4])) }
}

#[inline(always)]
pub const fn read_last_u32(data: *const u8) -> u32 {
    unsafe { transmute(*(data.sub(4) as *const [u8; 4])) }
}

#[inline(always)]
pub const fn read_u16(data: *const u8) -> u16 {
    unsafe { transmute(*(data as *const [u8; 2])) }
}
