// workarounds
#![allow(unused_doc_comment)]

// language extensions
#![feature(box_syntax, plugin)]

extern crate av_data as data;
extern crate num_rational as rational;
#[macro_use]
extern crate error_chain;

pub mod common;
pub mod decoder;
pub mod encoder;
pub mod error;
