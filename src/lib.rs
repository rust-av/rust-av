// language extensions
#![feature(box_syntax, plugin, heap_api, alloc)]
#![plugin(interpolate_idents)]

// crates
#![cfg_attr(feature = "assignment_operators", feature(augmented_assignments, op_assign_traits))]
#[macro_use]
extern crate bitflags;
extern crate bytes;
extern crate num;

#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

extern crate alloc;

// core functionalities
pub mod bitstream;
mod entropy;
pub mod data;
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
