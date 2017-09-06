// workarounds
#![allow(unused_doc_comment)]

// language extensions
#![feature(box_syntax, plugin)]
#![plugin(interpolate_idents)]

// crates
extern crate num_rational as rational;
#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

// local crates
extern crate av_data;

pub mod data {
    pub use av_data::*;
}

// core functionalities
pub mod bitstream;
mod entropy;
mod io;
pub mod buffer;

// encoded data manipulation
mod parser;
mod codec;
pub mod format;

// raw multimedia data manipulation
mod filter;
mod resample;
mod scale;
