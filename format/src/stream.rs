#![allow(dead_code)]

use rational::Rational32;
use data::frame::MediaKind;

#[derive(Clone,Debug,PartialEq)]
pub enum CodecID {
    VP9,
    Opus
}

#[derive(Clone,Debug,PartialEq)]
pub struct CodecParams {
    pub kind: MediaKind,
    pub codec_id: CodecID,
    pub extradata: Vec<u8>,
    pub tag: u32,
    pub bit_rate: usize,
    pub bits_per_coded_sample: usize,
    pub profile: usize,
    pub level: usize,
}

#[derive(Clone,Debug,PartialEq)]
pub struct Stream {
    pub id: usize,
    pub index: usize,
    pub params : CodecParams,
    pub start: Option<u64>,
    pub duration: Option<u64>,
    pub timebase : Rational32,
//  seek_index : SeekIndex
}

pub struct StreamGroup<'a> {
    pub id: usize,
    pub start: u64,
    pub end: u64,
    pub streams: &'a [Stream],
}
