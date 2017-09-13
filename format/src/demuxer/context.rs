#![allow(dead_code)]

use buffer::Buffered;
use stream::*;
use demuxer::demux::*;


use std::io::{BufRead,Error};

pub struct GlobalInfo {
    duration: Option<u64>,
    streams: Vec<Stream>,
//    programs: Vec<StreamGroup>,
//    chapters: Vec<StreamGroup>,
}

pub struct DemuxerContext<'a> {
    demuxer: Box<Demuxer+'a>,
    reader: Box<Buffered>,
    info: GlobalInfo,
}

impl<'a> DemuxerContext<'a> {
    pub fn new<R: Buffered+'static>(demuxer: Box<Demuxer + 'a>, reader: Box<R>) -> Self {
        DemuxerContext {
            demuxer:  demuxer,
            reader:   reader,
            info: GlobalInfo {
                duration: None,
                streams:  Vec::with_capacity(2),
            }
        }
    }

    pub fn read_headers(&mut self) -> Result<(), Error> {
        let ref mut demux = self.demuxer;

        let res = demux.read_headers(&self.reader, &mut self.info);
        match res {
            Err(e)   => Err(e),
            Ok(seek) => {
                //TODO: handle seeking here
                let res = self.reader.seek(seek);
                try!(self.reader.fill_buf());
                println!("stream now at index: {:?}", res);
                Ok(())
            }
        }
    }

    pub fn read_packet(&mut self) -> Result<Event, Error> {
        let ref mut demux = self.demuxer;

        let res = demux.read_packet(&self.reader);
        match res {
            Err(e)   => Err(e),
            Ok((seek, event)) => {
                //TODO: handle seeking here
                let res = self.reader.seek(seek);
                try!(self.reader.fill_buf());
                if let &Event::NewStream(ref st) = &event {
                    self.info.streams.push(st.clone());
                }
                println!("stream now at index: {:?}", res);
                Ok(event)
            }
        }
    }
}

