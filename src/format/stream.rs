#![allow(dead_code)]

use num::rational::Rational32;
use data::frame::MediaKind;

enum CodecID {
    //
}

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

pub struct Stream {
    id: usize,
    index: usize,
//  params : CodecParams,
//  seek_index : SeekIndex
}

pub struct StreamGroup<'a> {
    id: usize,
    start: u64,
    end: u64,
    streams: &'a [Stream],
}
