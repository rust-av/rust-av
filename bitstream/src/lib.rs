#![deny(clippy::undocumented_unsafe_blocks)]

extern crate num_traits;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub mod bitread;
pub mod byteread;
pub mod bytewrite;
pub mod codebook;
