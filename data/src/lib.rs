//! Structs and traits to interact with multimedia data.

#![deny(missing_docs, clippy::undocumented_unsafe_blocks)]

/// A module to represent and interact with rational numbers.
pub mod rational {
    pub use num_rational::*;
}

pub mod audiosample;
pub mod frame;
pub mod packet;
pub mod params;
pub mod pixel;
pub mod timeinfo;
pub mod value;
