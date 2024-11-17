use std::fmt;

/// General coding errors.
#[derive(Debug)]
pub enum Error {
    /// Invalid input data.
    InvalidData,
    /// A coding operation needs more data to be completed.
    MoreDataNeeded,
    /// Incomplete input configuration.
    ConfigurationIncomplete,
    /// Invalid input configuration.
    ConfigurationInvalid,
    /// Unsupported requested feature.
    Unsupported(String),
    // TODO add support for dependency-specific errors here
    // Inner(failure::Context)
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidData => write!(f, "Invalid Data"),
            Error::MoreDataNeeded => write!(f, "Additional data needed"),
            Error::ConfigurationIncomplete => write!(f, "Configuration Incomplete"),
            Error::ConfigurationInvalid => write!(f, "Configuration Invalid"),
            Error::Unsupported(feat) => write!(f, "Unsupported feature {feat}"),
        }
    }
}

/// A specialized `Result` type for coding operations.
pub type Result<T> = ::std::result::Result<T, Error>;
