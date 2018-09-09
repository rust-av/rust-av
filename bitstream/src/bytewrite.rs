#![allow(dead_code)]

use std::mem::transmute;
use std::ptr::copy_nonoverlapping;

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $dst:expr, $n:expr, $which:ident) => ({
        assert!($size <= $dst.len());
        unsafe {
            // N.B. https://github.com/rust-lang/rust/issues/22776
            let bytes = transmute::<_, [u8; $size]>($n.$which());
            copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
        }
    })
}

#[inline]
pub fn put_u8(buf:&mut[u8], n:u8) {
    buf[0] = n;
}

#[inline]
pub fn put_i8(buf:&mut[u8], n:i8) {
    buf[0] = n as u8;
}

#[inline]
pub fn put_u16l(buf:&mut[u8], n:u16) {
    write_num_bytes!(u16, 2, buf, n, to_le);
}

#[inline]
pub fn put_u16b(buf:&mut[u8], n:u16) {
    write_num_bytes!(u16, 2, buf, n, to_be);
}

#[inline]
pub fn put_u32l(buf:&mut[u8], n:u32) {
    write_num_bytes!(u32, 4, buf, n, to_le);
}

#[inline]
pub fn put_u32b(buf:&mut[u8], n:u32) {
    write_num_bytes!(u32, 4, buf, n, to_be);
}

#[inline]
pub fn put_u64l(buf:&mut[u8], n:u64) {
    write_num_bytes!(u64, 8, buf, n, to_le);
}

#[inline]
pub fn put_u64b(buf:&mut[u8], n:u64) {
    write_num_bytes!(u64, 8, buf, n, to_be);
}

#[inline]
pub fn put_i16l(buf:&mut[u8], n:i16) {
    put_u16l(buf, n as u16);
}

#[inline]
pub fn put_i16b(buf:&mut[u8], n:i16) {
    put_u16b(buf, n as u16);
}

#[inline]
pub fn put_i32l(buf:&mut[u8], n:i32) {
    put_u32l(buf, n as u32);
}

#[inline]
pub fn put_i32b(buf:&mut[u8], n:i32) {
    put_u32b(buf, n as u32);
}

#[inline]
pub fn put_i64l(buf:&mut[u8], n:i64) {
    put_u64l(buf, n as u64);
}

#[inline]
pub fn put_i64b(buf:&mut[u8], n:i64) {
    put_u64b(buf, n as u64);
}

#[inline]
pub fn put_f32l(buf:&mut[u8], n:f32) {
    put_u32l(buf, unsafe { transmute(n) });
}

#[inline]
pub fn put_f32b(buf:&mut[u8], n:f32) {
    put_u32b(buf, unsafe { transmute(n) });
}

#[inline]
pub fn put_f64l(buf:&mut[u8], n:f64) {
    put_u64l(buf, unsafe { transmute(n) });
}

#[inline]
pub fn put_f64b(buf:&mut[u8], n:f64) {
    put_u64b(buf, unsafe { transmute(n) });
}

#[cfg(test)]
mod test {

    use super::*;
    use byteread::*;

    #[test]
    fn put_and_get_u8() {
        let mut buf = [0; 3];
        put_u8(&mut buf, 1);
        assert!(1 == get_u8(&mut buf));

        put_u8(&mut buf[1..], 2);
        assert!(2 == get_u8(&mut buf[1..]));

        put_u8(&mut buf[2..], 255);
        assert!(255 == get_u8(&mut buf[2..]));
    }

    #[test]
    fn put_and_get_i8() {
        let mut buf = [0; 3];
        put_i8(&mut buf, 1);
        assert!(1 == get_i8(&mut buf));

        put_i8(&mut buf[1..], 2);
        assert!(2 == get_i8(&mut buf[1..]));

        put_i8(&mut buf[2..], -128);
        assert!(-128 == get_i8(&mut buf[2..]));
    }

}
