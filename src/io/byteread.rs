use std::io::{Error, Read, Result};
use std::io::ErrorKind::*;

use bitstream::byteread::*;

/**
 * Safe bytereader abstraction
 */

fn get_buffer<R: Read + ?Sized>(reader: &mut R, buf: &mut [u8]) -> Result<()> {
    let mut nread = 0usize;
    while nread < buf.len() {
        match reader.read(&mut buf[nread..]) {
            Ok(0) => return Err(Error::new(UnexpectedEof, "Empty")),
            Ok(n) => nread += n,
            Err(e) => return Err(e)
        }
    }
    Ok(())
}

macro_rules! get {
    ($s: ident, $name: ident, $ty: ty, $size: expr) => ({
        let mut buf = [0; $size];
        try!(get_buffer($s, &mut buf));
        Ok($name(&buf))
    })
}

pub trait ByteRead: Read {
    fn get_u16l(&mut self) -> Result<u16> {
        get!(self, get_u16l, u16, 2)
    }
/*
    fn get_u8(&mut self) -> Result<u8>;
    fn get_i8(&mut self) -> Result<i8>;

    fn get_u16l(&mut self) -> Result<u16>;
    fn get_u16b(&mut self) -> Result<u16>;

    fn get_u32l(&mut self) -> Result<u32>;
    fn get_u32b(&mut self) -> Result<u32>;

    fn get_u64l(&mut self) -> Result<u64>;
    fn get_u64b(&mut self) -> Result<u64>;

    fn get_i16l(&mut self) -> Result<i16>;
    fn get_i16b(&mut self) -> Result<i16>;

    fn get_i32l(&mut self) -> Result<i32>;
    fn get_i32b(&mut self) -> Result<i32>;

    fn get_i64l(&mut self) -> Result<i64>;
    fn get_i64b(&mut self) -> Result<i64>;

    fn get_f32l(&mut self) -> Result<f32>;
    fn get_f32b(&mut self) -> Result<f32>;

    fn get_f64l(&mut self) -> Result<f64>;
    fn get_f64b(&mut self) -> Result<f64>;
*/
}

impl<R: Read + ?Sized> ByteRead for R {}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use io::byteread::*;

    #[test]
    fn get_u16l() {
        let mut buf = Cursor::new(vec![1; 15]);

        println!("{}", buf.get_u16l().unwrap());

        assert!(buf.get_u16l().unwrap() == 257);
    }
}
