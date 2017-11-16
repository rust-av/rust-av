use data::packet::Packet;
use data::rational::Rational64;
use stream::Stream;
use error::*;

use buffer::Buffered;
use std::io::SeekFrom;

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalInfo {
    pub duration: Option<u64>,
    pub timebase: Option<Rational64>,
    pub streams: Vec<Stream>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    NewPacket(Packet),
    NewStream(Stream),
    MoreDataNeeded,
}

pub trait Demuxer {
    fn read_headers(&mut self, buf: &Box<Buffered>, info: &mut GlobalInfo) -> Result<SeekFrom>;
    fn read_event(&mut self, buf: &Box<Buffered>) -> Result<(SeekFrom, Event)>;
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
    fn create(&self) -> Box<Demuxer>;
    fn describe<'a>(&'a self) -> &'a Descr;

    fn probe(&self, data: &[u8]) -> u8;
}

pub struct Context {
    demuxer: Box<Demuxer>,
    reader: Box<Buffered>,
    pub info: GlobalInfo,
}

impl Context {
    pub fn new<R: Buffered + 'static>(demuxer: Box<Demuxer>, reader: Box<R>) -> Self {
        Context {
            demuxer: demuxer,
            reader: reader,
            info: GlobalInfo {
                duration: None,
                timebase: None,
                streams: Vec::with_capacity(2),
            },
        }
    }

    pub fn read_headers(&mut self) -> Result<()> {
        let ref mut demux = self.demuxer;

        try!(self.reader.fill_buf());

        let res = demux.read_headers(&self.reader, &mut self.info);
        match res {
            Err(e) => Err(e),
            Ok(seek) => {
                //TODO: handle seeking here
                let res = self.reader.seek(seek);
                try!(self.reader.fill_buf());
                println!("stream now at index: {:?}", res);
                Ok(())
            }
        }
    }

    pub fn read_event(&mut self) -> Result<Event> {
        let ref mut demux = self.demuxer;

        let res = demux.read_event(&self.reader);
        match res {
            Err(e) => Err(e),
            Ok((seek, event)) => {
                //TODO: handle seeking here
                let res = self.reader.seek(seek);
                try!(self.reader.fill_buf());
                if let Event::NewStream(ref st) = event {
                    self.info.streams.push(st.clone());
                }
                println!("stream now at index: {:?}", res);
                Ok(event)
            }
        }
    }
}


pub const PROBE_DATA: usize = 4 * 1024;
pub const PROBE_SCORE_EXTENSION: u8 = 50;

// TODO:
// IntoIterator<Item = &'static Descriptor> is confusing

pub trait Probe {
    fn probe(&self, data: &[u8]) -> Option<&'static Descriptor>;
}

impl<'a> Probe for [&'static Descriptor] {
    fn probe(&self, data: &[u8]) -> Option<&'static Descriptor> {
        let mut max = u8::min_value();
        let mut candidate: Option<&'static Descriptor> = None;
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

    struct DummyDes {
        d: Descr,
    }

    struct DummyDemuxer {}

    impl Demuxer for DummyDemuxer {
        fn read_headers(&mut self, _buf: &Box<Buffered>, _info: &mut GlobalInfo) -> Result<SeekFrom> {
            unimplemented!()
        }
        fn read_event(&mut self, _buf: &Box<Buffered>) -> Result<(SeekFrom, Event)> {
            unimplemented!()
        }
    }

    impl Descriptor for DummyDes {
        fn create(&self) -> Box<Demuxer> {
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

    const DUMMY_DES: &Descriptor = &DummyDes {
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
        let demuxers: &[&'static Descriptor] = &[DUMMY_DES];

        demuxers.probe(b"dummy").unwrap();
    }
}
