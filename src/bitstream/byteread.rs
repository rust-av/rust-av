#![allow(dead_code)]

// TODO: Implement for Cursor?
/**
 * Safe bytereader abstraction
 */
pub trait ByteRead {
    fn get_byte(&mut self) -> u8;

    fn get_u16l(&mut self) -> u16;
    fn get_u16b(&mut self) -> u16;

    fn get_u32l(&mut self) -> u32;
    fn get_u32b(&mut self) -> u32;

    fn get_u64l(&mut self) -> u64;
    fn get_u64b(&mut self) -> u64;

    fn get_i16l(&mut self) -> i16;
    fn get_i16b(&mut self) -> i16;

    fn get_i32l(&mut self) -> i32;
    fn get_i32b(&mut self) -> i32;

    fn get_i64l(&mut self) -> i64;
    fn get_i64b(&mut self) -> i64;

    fn get_f32l(&mut self) -> f32;
    fn get_f32b(&mut self) -> f32;

    fn get_f64l(&mut self) -> f64;
    fn get_f64b(&mut self) -> f64;
}

/**
 * Unsafe building blocks
 */

// TODO: arch-specific version
// TODO: aligned/non-aligned version

macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => ({
        unsafe {
            (*($src.as_ptr() as *const $ty)).$which()
        }
    });
}

#[inline]
pub fn get_u16l(buf:&[u8]) -> u16 {
    read_num_bytes!(u16, 2, buf, to_le)
}

#[inline]
pub fn get_u16b(buf:&[u8]) -> u16 {
    read_num_bytes!(u16, 2, buf, to_be)
}

#[inline]
pub fn get_u32l(buf:&[u8]) -> u32 {
    read_num_bytes!(u32, 4, buf, to_le)
}

#[inline]
pub fn get_u32b(buf:&[u8]) -> u32 {
    read_num_bytes!(u32, 4, buf, to_be)
}

#[inline]
pub fn get_u64l(buf:&[u8]) -> u64 {
    read_num_bytes!(u64, 8, buf, to_le)
}

#[inline]
pub fn get_u64b(buf:&[u8]) -> u64 {
    read_num_bytes!(u64, 8, buf, to_be)
}

#[inline]
pub fn get_i16l(buf:&[u8]) -> i16 {
    get_u16l(buf) as i16
}

#[inline]
pub fn get_i16b(buf:&[u8]) -> i16 {
    get_u16b(buf) as i16
}

#[inline]
pub fn get_i32l(buf:&[u8]) -> i32 {
    get_u32l(buf) as i32
}

#[inline]
pub fn get_i32b(buf:&[u8]) -> i32 {
    get_u32b(buf) as i32
}

#[inline]
pub fn get_i64l(buf:&[u8]) -> i64 {
    get_u64l(buf) as i64
}

#[inline]
pub fn get_i64b(buf:&[u8]) -> i64 {
    get_u64b(buf) as i64
}

#[inline]
pub fn get_f32l(buf:&[u8]) -> f32 {
    get_u32l(buf) as f32
}

#[inline]
pub fn get_f32b(buf:&[u8]) -> f32 {
    get_u32b(buf) as f32
}

#[inline]
pub fn get_f64l(buf:&[u8]) -> f64 {
    get_u64l(buf) as f64
}

#[inline]
pub fn get_f64b(buf:&[u8]) -> f64 {
    get_u64b(buf) as f64
}

// TODO: write meaningful tests.
#[cfg(test)]
mod test {


}
