//! Bytestream reader functionality.

// TODO: arch-specific version
// TODO: aligned/non-aligned version

/// Reads the first byte of a buffer.
#[inline]
pub fn get_u8(buf: &[u8]) -> u8 {
    assert!(!buf.is_empty());
    buf[0]
}

/// Reads the first byte of a buffer and returns it as an `i8`.
#[inline]
pub fn get_i8(buf: &[u8]) -> i8 {
    assert!(!buf.is_empty());
    buf[0] as i8
}

/// Reads the first 2 bytes of a buffer and stores them
/// in a little-endian order.
#[inline]
pub fn get_u16l(buf: &[u8]) -> u16 {
    assert!(buf.len() > 1);
    let data = [buf[0], buf[1]];
    u16::from_le_bytes(data)
}

/// Reads the first 2 bytes of a buffer and stores them in a big-endian order.
#[inline]
pub fn get_u16b(buf: &[u8]) -> u16 {
    assert!(buf.len() > 1);
    let data = [buf[0], buf[1]];
    u16::from_be_bytes(data)
}

/// Reads the first 4 bytes of a buffer and stores them
/// in a little-endian order.
#[inline]
pub fn get_u32l(buf: &[u8]) -> u32 {
    assert!(buf.len() > 3);
    let data = [buf[0], buf[1], buf[2], buf[3]];
    u32::from_le_bytes(data)
}

/// Reads the first 4 bytes of a buffer and stores them in a big-endian order.
#[inline]
pub fn get_u32b(buf: &[u8]) -> u32 {
    assert!(buf.len() > 3);
    let data = [buf[0], buf[1], buf[2], buf[3]];
    u32::from_be_bytes(data)
}

/// Reads the first 8 bytes of a buffer and stores them
/// in a little-endian order.
#[inline]
pub fn get_u64l(buf: &[u8]) -> u64 {
    assert!(buf.len() > 7);
    let data = [
        buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
    ];
    u64::from_le_bytes(data)
}

/// Reads the first 8 bytes of a buffer and stores them in a big-endian order.
#[inline]
pub fn get_u64b(buf: &[u8]) -> u64 {
    assert!(buf.len() > 7);
    let data = [
        buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
    ];
    u64::from_be_bytes(data)
}

/// Reads the first 2 bytes of a buffer, stores them in a little-endian order,
/// and returns them as an `i16`.
#[inline]
pub fn get_i16l(buf: &[u8]) -> i16 {
    get_u16l(buf) as i16
}

/// Reads the first 2 bytes of a buffer, stores them in a big-endian order,
/// and returns them as an `i16`.
#[inline]
pub fn get_i16b(buf: &[u8]) -> i16 {
    get_u16b(buf) as i16
}

/// Reads the first 4 bytes of a buffer, stores them in a little-endian order,
/// and returns them as an `i32`.
#[inline]
pub fn get_i32l(buf: &[u8]) -> i32 {
    get_u32l(buf) as i32
}

/// Reads the first 4 bytes of a buffer, stores them in a big-endian order,
/// and returns them as an `i32`.
#[inline]
pub fn get_i32b(buf: &[u8]) -> i32 {
    get_u32b(buf) as i32
}

/// Reads the first 8 bytes of a buffer, stores them in a little-endian order,
/// and returns them as an `i64`.
#[inline]
pub fn get_i64l(buf: &[u8]) -> i64 {
    get_u64l(buf) as i64
}

/// Reads the first 8 bytes of a buffer, stores them in a big-endian order,
/// and returns them as an `i64`.
#[inline]
pub fn get_i64b(buf: &[u8]) -> i64 {
    get_u64b(buf) as i64
}

/// Reads the first 4 bytes of a buffer, stores them in a little-endian order,
/// and returns them as an `f32`.
#[inline]
pub fn get_f32l(buf: &[u8]) -> f32 {
    f32::from_bits(get_u32l(buf))
}

/// Reads the first 4 bytes of a buffer, stores them in a big-endian order,
/// and returns them as an `f32`.
#[inline]
pub fn get_f32b(buf: &[u8]) -> f32 {
    f32::from_bits(get_u32b(buf))
}

/// Reads the first 8 bytes of a buffer, stores them in a little-endian order,
/// and returns them as an `f64`.
#[inline]
pub fn get_f64l(buf: &[u8]) -> f64 {
    f64::from_bits(get_u64l(buf))
}

/// Reads the first 8 bytes of a buffer, stores them in a big-endian order,
/// and returns them as an `f64`.
#[inline]
pub fn get_f64b(buf: &[u8]) -> f64 {
    f64::from_bits(get_u64b(buf))
}
