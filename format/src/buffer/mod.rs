mod accreader;

pub use self::accreader::AccReader;

use std::io::{BufRead, Seek, Cursor};

pub trait Buffered: BufRead + Seek {
    fn data(&self) -> &[u8];
}
