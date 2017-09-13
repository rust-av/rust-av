// workarounds
#![allow(unused_doc_comment)]

// language extensions
#![feature(box_syntax, plugin)]
#![plugin(interpolate_idents)]

// crates
#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

// local crates
extern crate av_data;
extern crate av_format;

pub mod data {
    pub use av_data::*;
}

pub mod format {
    pub use av_format::*;
}

// core functionalities
pub mod bitstream;
mod entropy;
mod io;

// raw multimedia data manipulation
mod filter;
mod resample;
mod scale;
