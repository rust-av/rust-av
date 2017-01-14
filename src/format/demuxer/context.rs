#![allow(dead_code)]

use data::packet::*;
use format::stream::*;
use format::demuxer::demux::*;

use std::io::BufRead; //TODO: Use an extended BufRead
use std::io::Error;
use std::marker::Sized;

pub struct DemuxerContext<'a> {
    demuxer: Box<Demuxer+'a>,
    reader: Box<BufRead>,
    duration: Option<u64>,
    streams: Vec<Stream>,
//    programs: Vec<StreamGroup>,
//    chapters: Vec<StreamGroup>,
}

impl<'a> DemuxerContext<'a> {
    pub fn new<R: BufRead+'static>(demuxer: Box<Demuxer + 'a>, reader: Box<R>) -> Self {
        DemuxerContext {
            demuxer:  demuxer,
            reader:   reader,
            duration: None,
            streams:  Vec::with_capacity(2),
        }
    }

    fn read_packet(&mut self) -> Result<Packet, Error> {
        let ref mut demux = self.demuxer;

        demux.read_packet(&self.reader)
    }
}
