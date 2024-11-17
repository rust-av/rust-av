use std::fmt;
use std::io;

/// General muxing/demuxing errors.
#[derive(Debug)]
pub enum Error {
    /// Invalid input data.
    InvalidData,
    /// A muxing/demuxing operation needs more data to be completed.
    MoreDataNeeded(usize),
    /// A more generic I/O error.
    Io(io::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(inner) => Some(inner),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidData => write!(f, "Invalid Data"),
            Error::MoreDataNeeded(n) => write!(f, "{n} more bytes needed"),
            Error::Io(_) => write!(f, "I/O error"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

/// A specialized `Result` type for muxing/demuxing operations.
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
