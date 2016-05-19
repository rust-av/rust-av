#![allow(dead_code)]

// use data::packet::*;
use format::stream::*;
use format::demuxer::demux::*;

use std::io::BufRead; //TODO: Use an extended BufRead

pub struct DemuxerContext {
    demuxer: Box<Demuxer>,
    reader: Box<BufRead>,
    duration: Option<u64>,
    streams: Vec<Stream>,
//    programs: Vec<StreamGroup>,
//    chapters: Vec<StreamGroup>,
}

impl DemuxerContext {
    fn new<D: Demuxer + 'static, R: BufRead + 'static>(demuxer: D, reader: R) -> Self {
        DemuxerContext {
            demuxer: box demuxer,
            reader: box reader,
            duration: None,
            streams: Vec::with_capacity(2)
        }
    }
}
