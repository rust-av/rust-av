// workarounds
#![allow(unused_doc_comment)]

// crates
extern crate num_rational as rational;

#[macro_use]
extern crate error_chain;

// local crates
extern crate av_data;

mod data {
    pub use av_data::*;
}

pub mod buffer;
pub mod stream;
pub mod demuxer;
pub mod error;
