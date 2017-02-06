#![allow(dead_code)]

use data::packet::*;
use buffer::Buffered;
use format::stream::*;
use format::demuxer::demux::*;


use std::io::{BufRead,Error,SeekFrom};
use std::marker::Sized;

pub struct DemuxerContext<'a> {
    demuxer: Box<Demuxer+'a>,
    reader: Box<Buffered>,
    duration: Option<u64>,
    streams: Vec<Stream>,
//    programs: Vec<StreamGroup>,
//    chapters: Vec<StreamGroup>,
}

impl<'a> DemuxerContext<'a> {
    pub fn new<R: Buffered+'static>(demuxer: Box<Demuxer + 'a>, reader: Box<R>) -> Self {
        DemuxerContext {
            demuxer:  demuxer,
            reader:   reader,
            duration: None,
            streams:  Vec::with_capacity(2),
        }
    }

    pub fn read_headers(&mut self) -> Result<(), Error> {
        let ref mut demux = self.demuxer;

        let res = demux.read_headers(&self.reader);
        match res {
            Err(e)   => Err(e),
            Ok(seek) => {
                //TODO: handle seeking here
                let res = self.reader.seek(seek);
                println!("stream now at index: {:?}", res);
                Ok(())
            }
        }
    }

    pub fn read_packet(&mut self) -> Result<Packet, Error> {
        let ref mut demux = self.demuxer;

        let res = demux.read_packet(&self.reader);
        match res {
            Err(e)   => Err(e),
            Ok((seek, packet)) => {
                //TODO: handle seeking here
                let res = self.reader.seek(seek);
                println!("stream now at index: {:?}", res);
                Ok(packet)
            }
        }
    }
}

