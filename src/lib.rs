#![deny(clippy::undocumented_unsafe_blocks)]
// workarounds
#![allow(unused_doc_comments)]

// crates

// local crates
extern crate av_bitstream;
extern crate av_data;
extern crate av_format;

pub mod data {
    pub use av_data::*;
}

pub mod format {
    pub use av_format::*;
}

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
