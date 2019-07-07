#![allow(dead_code)]

use std::ptr;
/**
 * Unsafe building blocks
 */

// TODO: arch-specific version
// TODO: aligned/non-aligned version

macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => {{
        let mut data: $ty = 0;
        unsafe {
            ptr::copy_nonoverlapping($src.as_ptr(), &mut data as *mut $ty as *mut u8, $size);
        }
        data.$which()
    }};
}

#[inline]
pub fn get_u8(buf: &[u8]) -> u8 {
    buf[0] as u8
}

#[inline]
pub fn get_i8(buf: &[u8]) -> i8 {
    buf[0] as i8
}

#[inline]
pub fn get_u16l(buf: &[u8]) -> u16 {
    read_num_bytes!(u16, 2, buf, to_le)
}

#[inline]
pub fn get_u16b(buf: &[u8]) -> u16 {
    read_num_bytes!(u16, 2, buf, to_be)
}

#[inline]
pub fn get_u32l(buf: &[u8]) -> u32 {
    read_num_bytes!(u32, 4, buf, to_le)
}

#[inline]
pub fn get_u32b(buf: &[u8]) -> u32 {
    read_num_bytes!(u32, 4, buf, to_be)
}

#[inline]
pub fn get_u64l(buf: &[u8]) -> u64 {
    read_num_bytes!(u64, 8, buf, to_le)
}

#[inline]
pub fn get_u64b(buf: &[u8]) -> u64 {
    read_num_bytes!(u64, 8, buf, to_be)
}

#[inline]
pub fn get_i16l(buf: &[u8]) -> i16 {
    get_u16l(buf) as i16
}

#[inline]
pub fn get_i16b(buf: &[u8]) -> i16 {
    get_u16b(buf) as i16
}

#[inline]
pub fn get_i32l(buf: &[u8]) -> i32 {
    get_u32l(buf) as i32
}

#[inline]
pub fn get_i32b(buf: &[u8]) -> i32 {
    get_u32b(buf) as i32
}

#[inline]
pub fn get_i64l(buf: &[u8]) -> i64 {
    get_u64l(buf) as i64
}

#[inline]
pub fn get_i64b(buf: &[u8]) -> i64 {
    get_u64b(buf) as i64
}

#[inline]
pub fn get_f32l(buf: &[u8]) -> f32 {
    f32::from_bits(get_u32l(buf))
}

#[inline]
pub fn get_f32b(buf: &[u8]) -> f32 {
    f32::from_bits(get_u32b(buf))
}

#[inline]
pub fn get_f64l(buf: &[u8]) -> f64 {
    f64::from_bits(get_u64l(buf))
}

#[inline]
pub fn get_f64b(buf: &[u8]) -> f64 {
    f64::from_bits(get_u64b(buf))
}
