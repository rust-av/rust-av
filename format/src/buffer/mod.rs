mod accreader;

pub use self::accreader::AccReader;

use std::io::{BufRead, Seek};

pub trait Buffered: BufRead + Seek + Send {
    fn data(&self) -> &[u8];
    fn grow(&mut self, len: usize);
}
