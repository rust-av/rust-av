use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid Data")]
    InvalidData,
    #[error("{0} more bytes needed")]
    MoreDataNeeded(usize),
    #[error("I/O error")]
    Io(#[from] io::Error),
}

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
