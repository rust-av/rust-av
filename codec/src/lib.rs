//! Contains utilities for encoding and decoding video and audio formats.

#![deny(missing_docs, clippy::undocumented_unsafe_blocks)]

/// Data structs shared between encoders and decoders.
pub mod common;
/// Utilities for decoding video and audio formats.
pub mod decoder;
/// Utilities for encoding video and audio formats.
pub mod encoder;
/// Error types.
pub mod error;
