use thiserror::Error;

/// General coding errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid input data.
    #[error("Invalid Data")]
    InvalidData,
    /// A coding operation needs more data to be completed.
    #[error("Additional data needed")]
    MoreDataNeeded,
    /// Incomplete input configuration.
    #[error("Configuration Incomplete")]
    ConfigurationIncomplete,
    /// Invalid input configuration.
    #[error("Configuration Invalid")]
    ConfigurationInvalid,
    /// Unsupported requested feature.
    #[error("Unsupported feature {0}")]
    Unsupported(String),
    // TODO add support for dependency-specific errors here
    // Inner(failure::Context)
}

/// A specialised `Result` type for coding operations.
pub type Result<T> = ::std::result::Result<T, Error>;
