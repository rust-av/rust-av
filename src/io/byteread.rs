use std::io::{Error, Read, Result};
use std::io::ErrorKind::*;

use bitstream::byteread::*;

/**
 * Safe bytereader abstraction
 */


#[allow(dead_code)]
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
    ($s: ident, $name: ident, $size: expr) => ({
        let mut buf = [0; $size];
        try!(get_buffer($s, &mut buf));
        Ok($name(&buf))
    })
}

pub trait ByteRead: Read {
    fn get_u8(&mut self) -> Result<u8> {
        get!(self, get_u8, 1)
    }
    fn get_i8(&mut self) -> Result<i8> {
        get!(self, get_i8, 1)
    }
    fn get_u16l(&mut self) -> Result<u16> {
        get!(self, get_u16l, 2)
    }
    fn get_i16l(&mut self) -> Result<i16> {
        get!(self, get_i16l, 2)
    }
    fn get_u32l(&mut self) -> Result<u32> {
        get!(self, get_u32l, 4)
    }
    fn get_i32l(&mut self) -> Result<i32> {
        get!(self, get_i32l, 4)
    }
    fn get_u64l(&mut self) -> Result<u64> {
        get!(self, get_u64l, 8)
    }
    fn get_i64l(&mut self) -> Result<i64> {
        get!(self, get_i64l, 8)
    }
    fn get_f32l(&mut self) -> Result<f32> {
        get!(self, get_f32l, 4)
    }
    fn get_f64l(&mut self) -> Result<f64> {
        get!(self, get_f64l, 8)
    }
    fn get_u16b(&mut self) -> Result<u16> {
        get!(self, get_u16b, 2)
    }
    fn get_i16b(&mut self) -> Result<i16> {
        get!(self, get_i16b, 2)
    }
    fn get_u32b(&mut self) -> Result<u32> {
        get!(self, get_u32b, 4)
    }
    fn get_i32b(&mut self) -> Result<i32> {
        get!(self, get_i32b, 4)
    }
    fn get_u64b(&mut self) -> Result<u64> {
        get!(self, get_u64b, 8)
    }
    fn get_i64b(&mut self) -> Result<i64> {
        get!(self, get_i64b, 8)
    }
    fn get_f32b(&mut self) -> Result<f32> {
        get!(self, get_f32b, 4)
    }
    fn get_f64b(&mut self) -> Result<f64> {
        get!(self, get_f64b, 8)
    }
}

impl<R: Read + ?Sized> ByteRead for R {}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use io::byteread::*;
    use std::io::ErrorKind::*;

    macro_rules! test {
        {$fun: ident, $val: expr, $len: expr} => {
            #[test]
            fn $fun() {
                let mut buf = Cursor::new(vec![1; 17]);

                for _ in 0..$len {
                    let v = buf.$fun().unwrap();
                    assert!(v == $val);
                }
                match buf.$fun() {
                    Ok(_) => assert!(false),
                    Err(e) => assert!(e.kind() == UnexpectedEof)
                }
            }
        }
    }

    test! { get_u8, 1, 17 }
    test! { get_i8, 1, 17 }
    test! { get_u16l, 257, 8 }
    test! { get_i16l, 257, 8 }
    test! { get_u32l, 16843009, 4 }
    test! { get_i32l, 16843009, 4 }
    test! { get_u64l, 72340172838076673u64, 2 }
    test! { get_i64l, 72340172838076673i64, 2 }
}
