use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid Data")]
    InvalidData,
    #[fail(display = "{} more bytes needed", _0)]
    MoreDataNeeded(usize),
    #[fail(display = "I/O error")]
    Io(#[cause] io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
