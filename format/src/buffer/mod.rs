mod accreader;

pub use self::accreader::AccReader;

use std::io::{BufRead, Seek};

/// Used to interact with a buffer.
pub trait Buffered: BufRead + Seek + Send + Sync {
    /// Returns the data contained in a buffer as a sequence of bytes.
    fn data(&self) -> &[u8];
    /// Increases the size of a buffer.
    fn grow(&mut self, len: usize);
}
