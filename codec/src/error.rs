#[derive(err_derive::Error, Debug)]
pub enum Error {
    #[error(display = "Invalid Data")]
    InvalidData,
    #[error(display = "Additional data needed")]
    MoreDataNeeded,
    #[error(display = "Configuration Incomplete")]
    ConfigurationIncomplete,
    #[error(display = "Configuration Invalid")]
    ConfigurationInvalid,
    #[error(display = "Unsupported feature {}", _0)]
    Unsupported(String),
    // TODO add support for dependency-specific errors here
    // Inner(failure::Context)
}

pub type Result<T> = ::std::result::Result<T, Error>;
