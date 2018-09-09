// language extensions
#![feature(rust_2018_preview)]

#[macro_use]
extern crate failure;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub mod bitread;
pub mod byteread;
pub mod bytewrite;
pub mod codebook;


