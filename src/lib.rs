// language extensions
#![feature(box_syntax, plugin)]
#![plugin(interpolate_idents)]

// crates
#![cfg_attr(feature = "assignment_operators", feature(augmented_assignments, op_assign_traits))]
#[macro_use]
extern crate bitflags;

// core functionalities
mod bitstream;
mod entropy;
mod data;
mod io;

// encoded data manipulation
mod parser;
mod codec;
mod format;

// raw multimedia data manipulation
mod filter;
mod resample;
mod scale;
