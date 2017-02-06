mod accreader;

pub use self::accreader::AccReader;

use std::io::BufRead;

pub trait Buffered: BufRead {
  fn data(&self) -> &[u8];
}

