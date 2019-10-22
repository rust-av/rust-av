use std::io;

#[derive(err_derive::Error, Debug)]
pub enum Error {
    #[error(display = "Invalid Data")]
    InvalidData,
    #[error(display = "{} more bytes needed", _0)]
    MoreDataNeeded(usize),
    #[error(display = "I/O error")]
    Io(#[cause] io::Error),
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
            Error::Io(_) => {},
            _ => panic!("Error doesn't match"),
        }
    }
}
