// workarounds
#![allow(unused_doc_comments)]

// language extensions
#![feature(rust_2018_preview)]

// crates
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

// local crates
extern crate av_data;

mod data {
    pub use av_data::*;
}

pub use av_data::rational;

pub mod common;
pub mod buffer;
pub mod stream;
pub mod demuxer;
pub mod muxer;
pub mod error;
