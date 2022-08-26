//! Contains utilities for muxing and demuxing into various container formats.

#![deny(missing_docs, clippy::undocumented_unsafe_blocks)]

mod data {
    pub use av_data::*;
}

pub use av_data::rational;

/// Buffered data helpers
pub mod buffer;
/// Common data structs reused between muxers and demuxers
pub mod common;
/// Utilities for demuxing containers
pub mod demuxer;
/// Error types
pub mod error;
/// Utilities for muxing containers
pub mod muxer;
/// Data structs representing a video, audio, or subtitle stream
pub mod stream;
