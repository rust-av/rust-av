#![allow(dead_code)]

use rational::Rational32;
use data::frame::MediaKind;

#[derive(Clone)]
enum CodecID {
    VP9,
    Opus
}

#[derive(Clone)]
pub struct CodecParams {
    kind: MediaKind,
    codec_id: CodecID,
    extradata: Vec<u8>,
    tag: u32,
    bit_rate: usize,
    bits_per_coded_sample: usize,
    profile: usize,
    level: usize,
}

#[derive(Clone)]
pub struct Stream {
    id: usize,
    index: usize,
    params : CodecParams,
    start: Option<u64>,
    duration: Option<u64>,
    timebase : Rational32,
//  seek_index : SeekIndex
}

pub struct StreamGroup<'a> {
    id: usize,
    start: u64,
    end: u64,
    streams: &'a [Stream],
}
