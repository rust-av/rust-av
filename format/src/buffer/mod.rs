mod accreader;

pub use self::accreader::AccReader;

use std::io::{BufRead, Seek, Cursor};

pub trait Buffered: BufRead + Seek {
    fn data(&self) -> &[u8];
}

impl<'a> Buffered for Cursor<&'a [u8]> {
    fn data(&self) -> &[u8] {
        &self.get_ref()[self.position() as usize..]
    }
}
