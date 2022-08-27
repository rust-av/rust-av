//! A collection of utilities for handling multimedia formats.
//! The goal is to provide functionality similar to ffmpeg/libav,
//! but written in pure Rust.

#![deny(missing_docs, clippy::undocumented_unsafe_blocks)]

/// Contains utilities for encoding and decoding video and audio formats.
pub mod codec {
    pub use av_codec::*;
}

/// Structs and traits to interact with multimedia data.
pub mod data {
    pub use av_data::*;
}

/// Contains utilities for muxing and demuxing into various container formats.
pub mod format {
    pub use av_format::*;
}

/// Bytes and bitstream reading/writing functionality.
pub mod bitstream {
    pub use av_bitstream::*;
}

pub use av_data::rational;

// core functionalities
mod entropy;
mod io;

// raw multimedia data manipulation
mod filter;
mod resample;
mod scale;
