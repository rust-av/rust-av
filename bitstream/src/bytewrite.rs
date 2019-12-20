macro_rules! write_bytes_le {
    ($buf:ident, $n:ident) => {
        let bytes = $n.to_le_bytes();
        $buf.to_vec().extend_from_slice(&bytes);
    };
}

macro_rules! write_bytes_be {
    ($buf:ident, $n:ident) => {
        let bytes = $n.to_be_bytes();
        $buf.to_vec().extend_from_slice(&bytes);
    };
}

#[inline]
pub fn put_u8(buf: &mut [u8], n: u8) {
    buf[0] = n;
}

#[inline]
pub fn put_i8(buf: &mut [u8], n: i8) {
    buf[0] = n as u8;
}

#[inline]
pub fn put_u16l(buf: &mut [u8], n: u16) {
    write_bytes_le!(buf, n);
}

#[inline]
pub fn put_u16b(buf: &mut [u8], n: u16) {
    write_bytes_be!(buf, n);
}

#[inline]
pub fn put_u32l(buf: &mut [u8], n: u32) {
    write_bytes_le!(buf, n);
}

#[inline]
pub fn put_u32b(buf: &mut [u8], n: u32) {
    write_bytes_be!(buf, n);
}

#[inline]
pub fn put_u64l(buf: &mut [u8], n: u64) {
    write_bytes_le!(buf, n);
}

#[inline]
pub fn put_u64b(buf: &mut [u8], n: u64) {
    write_bytes_be!(buf, n);
}

#[inline]
pub fn put_i16l(buf: &mut [u8], n: i16) {
    put_u16l(buf, n as u16);
}

#[inline]
pub fn put_i16b(buf: &mut [u8], n: i16) {
    put_u16b(buf, n as u16);
}

#[inline]
pub fn put_i32l(buf: &mut [u8], n: i32) {
    put_u32l(buf, n as u32);
}

#[inline]
pub fn put_i32b(buf: &mut [u8], n: i32) {
    put_u32b(buf, n as u32);
}

#[inline]
pub fn put_i64l(buf: &mut [u8], n: i64) {
    put_u64l(buf, n as u64);
}

#[inline]
pub fn put_i64b(buf: &mut [u8], n: i64) {
    put_u64b(buf, n as u64);
}

#[inline]
pub fn put_f32l(buf: &mut [u8], n: f32) {
    write_bytes_le!(buf, n);
}

#[inline]
pub fn put_f32b(buf: &mut [u8], n: f32) {
    write_bytes_be!(buf, n);
}

#[inline]
pub fn put_f64l(buf: &mut [u8], n: f64) {
    write_bytes_le!(buf, n);
}

#[inline]
pub fn put_f64b(buf: &mut [u8], n: f64) {
    write_bytes_be!(buf, n);
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::byteread::*;

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
