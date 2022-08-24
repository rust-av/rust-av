#![deny(clippy::undocumented_unsafe_blocks)]

mod data {
    pub use av_data::*;
}

pub use av_data::rational;

pub mod buffer;
pub mod common;
pub mod demuxer;
pub mod error;
pub mod muxer;
pub mod stream;
