// workarounds
#![allow(unused_doc_comments)]

// language extensions
#![feature(box_syntax, plugin)]

extern crate av_data as data;
extern crate num_rational as rational;
#[macro_use]
extern crate failure;

pub mod common;
pub mod decoder;
pub mod encoder;
pub mod error;
