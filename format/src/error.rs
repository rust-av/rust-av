use std::io;

use thiserror::Error;

/// General muxing/demuxing errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid input data.
    #[error("Invalid Data")]
    InvalidData,
    /// A muxing/demuxing operation needs more data to be completed.
    #[error("{0} more bytes needed")]
    MoreDataNeeded(usize),
    #[error("I/O error")]
    /// A more generic I/O error.
    Io(#[from] io::Error),
}

/// A specialised `Result` type for muxing/demuxing operations.
pub type Result<T> = ::std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::Other, "foobar");

        let err: Error = io_err.into();

        match err {
            Error::Io(_) => {}
            _ => panic!("Error doesn't match"),
        }
    }
}
