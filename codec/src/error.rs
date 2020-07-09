use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid Data")]
    InvalidData,
    #[error("Additional data needed")]
    MoreDataNeeded,
    #[error("Configuration Incomplete")]
    ConfigurationIncomplete,
    #[error("Configuration Invalid")]
    ConfigurationInvalid,
    #[error("Unsupported feature {0}")]
    Unsupported(String),
    // TODO add support for dependency-specific errors here
    // Inner(failure::Context)
}

pub type Result<T> = ::std::result::Result<T, Error>;
