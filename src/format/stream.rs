#![allow(dead_code)]

use num::rational::Rational32;

bitflags! {
    flags ChannelLayout: u64 {
        const CH_FRONT_LEFT = 0b1,
    }
}

struct SampleFormat {
    channel_layout: ChannelLayout,
    channels: usize,
    rate: usize,
    block_align: usize,
    initial_padding: usize,
    trailing_padding: usize,
}

struct PixelFormat {
    width: u32,
    height: u32,
    aspect_ratio: Rational32,
    field_order: usize,
}

enum MediaFormat {
    Audio(SampleFormat),
    Video(PixelFormat),
    Unknown
}

enum MediaType {
    Audio,
    Video,
    Subtitle,
    Data,
    Unknown
}

enum CodecID {
    //
}

pub struct CodecParams {
    codec_type: MediaType,
    codec_id: CodecID,
    extradata: Vec<u8>,
    tag: u32,
    bit_rate: usize,
    bits_per_coded_sample: usize,
    profile: usize,
    level: usize,

    format : MediaFormat
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
