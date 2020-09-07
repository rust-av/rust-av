macro_rules! write_bytes_le {
    ($buf:ident, $n:ident) => {
        let bytes = $n.to_le_bytes();
        &mut $buf[..bytes.len()].copy_from_slice(&bytes);
    };
}

macro_rules! write_bytes_be {
    ($buf:ident, $n:ident) => {
        let bytes = $n.to_be_bytes();
        &mut $buf[..bytes.len()].copy_from_slice(&bytes);
    };
}

/// Writes an unsigned byte at the start of a buffer.
#[inline]
pub fn put_u8(buf: &mut [u8], n: u8) {
    buf[0] = n;
}

/// Converts a `i8` into an unsigned byte and
/// writes it at the start of a buffer.
#[inline]
pub fn put_i8(buf: &mut [u8], n: i8) {
    buf[0] = n as u8;
}

/// Writes 2 unsigned bytes in a little-endian order at the start of a buffer.
#[inline]
pub fn put_u16l(buf: &mut [u8], n: u16) {
    write_bytes_le!(buf, n);
}

/// Writes 2 unsigned bytes in a big-endian order at the start of a buffer.
#[inline]
pub fn put_u16b(buf: &mut [u8], n: u16) {
    write_bytes_be!(buf, n);
}

/// Writes 4 unsigned bytes in a little-endian order at the start of a buffer.
#[inline]
pub fn put_u32l(buf: &mut [u8], n: u32) {
    write_bytes_le!(buf, n);
}

/// Writes 4 unsigned bytes in a big-endian order at the start of a buffer.
#[inline]
pub fn put_u32b(buf: &mut [u8], n: u32) {
    write_bytes_be!(buf, n);
}

/// Writes 8 unsigned bytes in a little-endian order at the start of a buffer.
#[inline]
pub fn put_u64l(buf: &mut [u8], n: u64) {
    write_bytes_le!(buf, n);
}

/// Writes 8 unsigned bytes in a big-endian order at the start of a buffer.
#[inline]
pub fn put_u64b(buf: &mut [u8], n: u64) {
    write_bytes_be!(buf, n);
}

/// Converts an `i16` into 2 unsigned bytes and
/// writes them in a little-endian order at the start of a buffer.
#[inline]
pub fn put_i16l(buf: &mut [u8], n: i16) {
    put_u16l(buf, n as u16);
}

/// Converts an `i16` into 2 unsigned bytes and
/// writes them in a big-endian order at the start of a buffer.
#[inline]
pub fn put_i16b(buf: &mut [u8], n: i16) {
    put_u16b(buf, n as u16);
}

/// Converts an `i32` into 4 unsigned bytes and
/// writes them in a little-endian order at the start of a buffer.
#[inline]
pub fn put_i32l(buf: &mut [u8], n: i32) {
    put_u32l(buf, n as u32);
}

/// Converts an `i32` into 4 unsigned bytes and
/// writes them in a big-endian order at the start of a buffer.
#[inline]
pub fn put_i32b(buf: &mut [u8], n: i32) {
    put_u32b(buf, n as u32);
}

/// Converts an `i64` into 8 unsigned bytes and
/// writes them in a little-endian order at the start of a buffer.
#[inline]
pub fn put_i64l(buf: &mut [u8], n: i64) {
    put_u64l(buf, n as u64);
}

/// Converts an `i64` into 8 unsigned bytes and
/// writes them in a big-endian order at the start of a buffer.
#[inline]
pub fn put_i64b(buf: &mut [u8], n: i64) {
    put_u64b(buf, n as u64);
}

/// Converts a `f32` into 4 unsigned bytes and
/// writes them in a little-endian order at the start of a buffer.
#[inline]
pub fn put_f32l(buf: &mut [u8], n: f32) {
    write_bytes_le!(buf, n);
}

/// Converts a `f32` into 4 unsigned bytes and
/// writes them in a big-endian order at the start of a buffer.
#[inline]
pub fn put_f32b(buf: &mut [u8], n: f32) {
    write_bytes_be!(buf, n);
}

/// Converts a `f64` into 8 unsigned bytes and
/// writes them in a little-endian order at the start of a buffer.
#[inline]
pub fn put_f64l(buf: &mut [u8], n: f64) {
    write_bytes_le!(buf, n);
}

/// Converts a `f64` into 8 unsigned bytes and
/// writes them in a big-endian order at the start of a buffer.
#[inline]
pub fn put_f64b(buf: &mut [u8], n: f64) {
    write_bytes_be!(buf, n);
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::byteread::*;
    use std::mem::size_of;

    #[test]
    fn put_and_get_u8() {
        let mut buf = [0; 3];
        put_u8(&mut buf, 1);
        assert!(1 == get_u8(&buf));

        put_u8(&mut buf[1..], 2);
        assert!(2 == get_u8(&buf[1..]));

        put_u8(&mut buf[2..], 255);
        assert!(255 == get_u8(&buf[2..]));
    }

    #[test]
    fn put_and_get_i8() {
        let mut buf = [0; 3];
        put_i8(&mut buf, 1);
        assert!(1 == get_i8(&buf));

        put_i8(&mut buf[1..], 2);
        assert!(2 == get_i8(&buf[1..]));

        put_i8(&mut buf[2..], -128);
        assert!(-128 == get_i8(&buf[2..]));
    }

    macro_rules! decl_put_and_get_endian_tests {
        ($($TYPE:ty),*) => {
            $(
                paste::item! {
                    #[test]
                    fn [<put_and_get_ $TYPE _be>]() {
                        let item_size = size_of::<$TYPE>();
                        let mut buf = vec![0; 3 * item_size];
                        [<put_ $TYPE b>](&mut buf, 1 as $TYPE);
                        assert!(1 as $TYPE == [<get_ $TYPE b>](&buf));

                        [<put_ $TYPE b>](&mut buf[(1 * item_size)..], 2 as $TYPE);
                        assert!(2 as $TYPE == [<get_ $TYPE b>](&buf[(1 * item_size)..]));

                        [<put_ $TYPE b>](&mut buf[(2 * item_size)..], 255 as $TYPE);
                        assert!(255 as $TYPE == [<get_ $TYPE b>](&buf[(2 * item_size)..]));
                    }
                }

                paste::item! {
                    #[test]
                    fn [<put_and_get_ $TYPE _l>]() {
                        let item_size = size_of::<$TYPE>();
                        let mut buf = vec![0; 3 * item_size];
                        [<put_ $TYPE l>](&mut buf, 1 as $TYPE);
                        assert!(1 as $TYPE == [<get_ $TYPE l>](&buf));

                        [<put_ $TYPE l>](&mut buf[(1 * item_size)..], 2 as $TYPE);
                        assert!(2 as $TYPE == [<get_ $TYPE l>](&buf[(1 * item_size)..]));

                        [<put_ $TYPE l>](&mut buf[(2 * item_size)..], 255 as $TYPE);
                        assert!(255 as $TYPE == [<get_ $TYPE l>](&buf[(2 * item_size)..]));
                    }
                }
            )*
        };
    }

    macro_rules! decl_put_and_get_endian_float_tests {
        ($($TYPE:ty),*) => {
            $(
                paste::item! {
                    #[test]
                    fn [<put_and_get_ $TYPE _be>]() {
                        let item_size = size_of::<$TYPE>();
                        let mut buf = vec![0; 3 * item_size];
                        [<put_ $TYPE b>](&mut buf, 1 as $TYPE);
                        assert!(1 as $TYPE - [<get_ $TYPE b>](&buf) < ::std::$TYPE::EPSILON);

                        [<put_ $TYPE b>](&mut buf[(1 * item_size)..], 2 as $TYPE);
                        assert!(2 as $TYPE - [<get_ $TYPE b>](&buf[(1 * item_size)..]) < ::std::$TYPE::EPSILON);

                        [<put_ $TYPE b>](&mut buf[(2 * item_size)..], 255 as $TYPE);
                        assert!(255 as $TYPE - [<get_ $TYPE b>](&buf[(2 * item_size)..]) < ::std::$TYPE::EPSILON);
                    }
                }

                paste::item! {
                    #[test]
                    fn [<put_and_get_ $TYPE _l>]() {
                        let item_size = size_of::<$TYPE>();
                        let mut buf = vec![0; 3 * item_size];
                        [<put_ $TYPE l>](&mut buf, 1 as $TYPE);
                        assert!(1 as $TYPE - [<get_ $TYPE l>](&buf) < ::std::$TYPE::EPSILON);

                        [<put_ $TYPE l>](&mut buf[(1 * item_size)..], 2 as $TYPE);
                        assert!(2 as $TYPE - [<get_ $TYPE l>](&buf[(1 * item_size)..]) < ::std::$TYPE::EPSILON);

                        [<put_ $TYPE l>](&mut buf[(2 * item_size)..], 255 as $TYPE);
                        assert!(255 as $TYPE - [<get_ $TYPE l>](&buf[(2 * item_size)..]) < ::std::$TYPE::EPSILON);
                    }
                }
            )*
        };
    }

    decl_put_and_get_endian_tests!(u16, i16, u32, i32, u64, i64);
    decl_put_and_get_endian_float_tests!(f32, f64);
}
