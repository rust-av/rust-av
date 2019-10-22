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
