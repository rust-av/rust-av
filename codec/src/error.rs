#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid Data")]
    InvalidData,
    #[fail(display = "Additional data needed")]
    MoreDataNeeded,
    #[fail(display = "Configuration Incomplete")]
    ConfigurationIncomplete,
    #[fail(display = "Configuration Invalid")]
    ConfigurationInvalid,
    #[fail(display = "Unsupported feature {}", _0)]
    Unsupported(String),
    // TODO add support for dependency-specific errors here
    // Inner(failure::Context)
}

pub type Result<T> = ::std::result::Result<T, Error>;
