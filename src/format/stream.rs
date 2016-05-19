#![allow(dead_code)]

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
