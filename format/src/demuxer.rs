#![allow(clippy::borrowed_box)]

use crate::error::*;

use crate::buffer::Buffered;
use std::any::Any;
use std::io::SeekFrom;
use std::sync::Arc;

use crate::common::*;

use crate::data::packet::Packet;
use crate::stream::Stream;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Event {
    NewPacket(Packet),
    NewStream(Stream),
    MoreDataNeeded(usize),
    Continue,
    Eof,
}

pub trait Demuxer: Send {
    fn read_headers(&mut self, buf: &Box<dyn Buffered>, info: &mut GlobalInfo) -> Result<SeekFrom>;
    fn read_event(&mut self, buf: &Box<dyn Buffered>) -> Result<(SeekFrom, Event)>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Descr {
    pub name: &'static str,
    pub demuxer: &'static str,
    pub description: &'static str,
    pub extensions: &'static [&'static str],
    pub mime: &'static [&'static str],
}

pub trait Descriptor {
    fn create(&self) -> Box<dyn Demuxer>;
    fn describe(&self) -> &Descr;

    fn probe(&self, data: &[u8]) -> u8;
}

pub struct Context {
    demuxer: Box<dyn Demuxer>,
    reader: Box<dyn Buffered>,
    pub info: GlobalInfo,
    pub user_private: Option<Arc<dyn Any + Send + Sync>>,
}

impl Context {
    pub fn new<R: Buffered + 'static>(demuxer: Box<dyn Demuxer>, reader: Box<R>) -> Self {
        Context {
            demuxer,
            reader,
            info: GlobalInfo {
                duration: None,
                timebase: None,
                streams: Vec::with_capacity(2),
            },
            user_private: None,
        }
    }

    fn read_headers_internal(&mut self) -> Result<()> {
        let demux = &mut self.demuxer;

        let res = demux.read_headers(&self.reader, &mut self.info);
        match res {
            Err(e) => Err(e),
            Ok(seek) => {
                //TODO: handle seeking here
                let res = self.reader.seek(seek);
                trace!("stream now at index: {:?}", res);
                Ok(())
            }
        }
    }

    pub fn read_headers(&mut self) -> Result<()> {
        loop {
            // TODO: wrap fill_buf() with a check for Eof
            self.reader.fill_buf()?;
            match self.read_headers_internal() {
                Err(e) => match e {
                    Error::MoreDataNeeded(needed) => {
                        self.reader.grow(needed);
                    }
                    _ => return Err(e),
                },
                Ok(_) => return Ok(()),
            }
        }
    }

    fn read_event_internal(&mut self) -> Result<Event> {
        let demux = &mut self.demuxer;

        let res = demux.read_event(&self.reader);
        match res {
            Err(e) => Err(e),
            Ok((seek, mut event)) => {
                //TODO: handle seeking here
                let _ = self.reader.seek(seek)?;
                if let Event::NewStream(ref st) = event {
                    self.info.streams.push(st.clone());
                }
                if let Event::MoreDataNeeded(size) = event {
                    return Err(Error::MoreDataNeeded(size));
                }
                if let Event::NewPacket(ref mut pkt) = event {
                    if pkt.t.timebase.is_none() {
                        if let Some(ref st) = self
                            .info
                            .streams
                            .iter()
                            .find(|s| s.index as isize == pkt.stream_index)
                        {
                            pkt.t.timebase = Some(st.timebase);
                        }
                    }
                }
                Ok(event)
            }
        }
    }

    pub fn read_event(&mut self) -> Result<Event> {
        // TODO: guard against infiniloops and maybe factor the loop.
        loop {
            match self.read_event_internal() {
                Err(e) => match e {
                    Error::MoreDataNeeded(needed) => {
                        let len = self.reader.data().len();

                        // we might have sent MoreDatNeeded(0) to request a new call
                        if len >= needed {
                            continue;
                        }
                        self.reader.grow(needed);
                        self.reader.fill_buf()?;
                        if self.reader.data().len() <= len {
                            return Ok(Event::Eof);
                        }
                    }
                    _ => return Err(e),
                },
                Ok(ev) => return Ok(ev),
            }
        }
    }
}

pub const PROBE_DATA: usize = 4 * 1024;
pub const PROBE_SCORE_EXTENSION: u8 = 50;

// TODO:
// IntoIterator<Item = &'static Descriptor> is confusing

pub trait Probe {
    fn probe(&self, data: &[u8]) -> Option<&'static dyn Descriptor>;
}

impl<'a> Probe for [&'static dyn Descriptor] {
    fn probe(&self, data: &[u8]) -> Option<&'static dyn Descriptor> {
        let mut max = u8::min_value();
        let mut candidate: Option<&'static dyn Descriptor> = None;
        for desc in self {
            let score = desc.probe(data);

            if score > max {
                max = score;
                candidate = Some(*desc);
            }
        }

        if max > PROBE_SCORE_EXTENSION {
            candidate
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::packet::Packet;
    use std::io::SeekFrom;

    struct DummyDes {
        d: Descr,
    }

    struct DummyDemuxer {}

    impl Demuxer for DummyDemuxer {
        fn read_headers(
            &mut self,
            buf: &Box<dyn Buffered>,
            _info: &mut GlobalInfo,
        ) -> Result<SeekFrom> {
            let len = buf.data().len();
            if 9 > len {
                let needed = 9 - len;
                Err(Error::MoreDataNeeded(needed))
            } else {
                Ok(SeekFrom::Current(9))
            }
        }
        fn read_event(&mut self, buf: &Box<dyn Buffered>) -> Result<(SeekFrom, Event)> {
            let size = 2;
            let len = buf.data().len();
            if size > len {
                Err(Error::MoreDataNeeded(size - len))
            } else {
                debug!("{:?}", buf.data());
                match &buf.data()[..2] {
                    b"p1" => Ok((SeekFrom::Current(3), Event::NewPacket(Packet::new()))),
                    b"e1" => Ok((SeekFrom::Current(3), Event::MoreDataNeeded(0))),
                    _ => Err(Error::InvalidData.into()),
                }
            }
        }
    }

    impl Descriptor for DummyDes {
        fn create(&self) -> Box<dyn Demuxer> {
            Box::new(DummyDemuxer {})
        }
        fn describe<'a>(&'a self) -> &'a Descr {
            &self.d
        }
        fn probe(&self, data: &[u8]) -> u8 {
            match data {
                b"dummy" => 100,
                _ => 0,
            }
        }
    }

    const DUMMY_DES: &dyn Descriptor = &DummyDes {
        d: Descr {
            name: "dummy",
            demuxer: "dummy",
            description: "Dummy dem",
            extensions: &["dm", "dum"],
            mime: &["application/dummy"],
        },
    };

    #[test]
    fn probe() {
        let demuxers: &[&'static dyn Descriptor] = &[DUMMY_DES];

        demuxers.probe(b"dummy").unwrap();
    }

    use crate::buffer::*;
    use std::io::Cursor;

    #[test]
    fn read_headers() {
        let buf = b"dummy header";
        let r = AccReader::with_capacity(4, Cursor::new(buf));
        let d = DUMMY_DES.create();
        let mut c = Context::new(d, Box::new(r));

        c.read_headers().unwrap();
    }

    #[test]
    fn read_event() {
        let buf = b"dummy header p1 e1 p1 ";

        let r = AccReader::with_capacity(4, Cursor::new(buf));
        let d = DUMMY_DES.create();
        let mut c = Context::new(d, Box::new(r));

        c.read_headers().unwrap();

        println!("{:?}", c.read_event());
        println!("{:?}", c.read_event());
        println!("{:?}", c.read_event());
        println!("{:?}", c.read_event());
    }
}
