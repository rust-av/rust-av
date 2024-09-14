use crate::error::*;

use crate::buffer::Buffered;
use std::any::Any;
use std::io::SeekFrom;
use std::sync::Arc;

use crate::common::*;

use crate::data::packet::Packet;
use crate::stream::Stream;

/// Events processed by a demuxer analyzing a source.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Event {
    /// A new packet is found by a demuxer.
    NewPacket(Packet),
    /// A new stream is found by a demuxer.
    NewStream(Stream),
    /// More data are needed by a demuxer to complete its operations.
    MoreDataNeeded(usize),
    /// Event not processable by a demuxer.
    ///
    /// Demux the next event.
    Continue,
    /// End of File.
    ///
    /// Stop demuxing data.
    Eof,
}

/// Used to implement demuxing operations.
pub trait Demuxer: Send + Sync {
    /// Reads stream headers and global information from a data structure
    /// implementing the `Buffered` trait.
    ///
    /// Global information are saved into a `GlobalInfo` structure.
    fn read_headers(&mut self, buf: &mut dyn Buffered, info: &mut GlobalInfo) -> Result<SeekFrom>;
    /// Reads an event from a data structure implementing the `Buffered` trait.
    fn read_event(&mut self, buf: &mut dyn Buffered) -> Result<(SeekFrom, Event)>;
}

/// Auxiliary structure to encapsulate a demuxer object and
/// its additional data.
pub struct Context<D: Demuxer, R: Buffered> {
    demuxer: D,
    reader: R,
    /// Global media file information.
    pub info: GlobalInfo,
    /// User private data.
    ///
    /// This data cannot be cloned.
    pub user_private: Option<Arc<dyn Any + Send + Sync>>,
}

impl<D: Demuxer, R: Buffered> Context<D, R> {
    /// Creates a new `Context` instance.
    pub fn new(demuxer: D, reader: R) -> Self {
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

    /// Returns the underlying demuxer.
    pub fn demuxer(&self) -> &D {
        &self.demuxer
    }

    fn read_headers_internal(&mut self) -> Result<()> {
        let demux = &mut self.demuxer;

        let res = demux.read_headers(&mut self.reader, &mut self.info);
        match res {
            Err(e) => Err(e),
            Ok(seek) => {
                //TODO: handle seeking here
                let res = self.reader.seek(seek);
                log::trace!("stream now at index: {:?}", res);
                Ok(())
            }
        }
    }

    /// Reads stream headers and global information from a data source.
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

        let res = demux.read_event(&mut self.reader);
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
                        if let Some(st) = self
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

    /// Reads an event from a data source.
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

/// Format descriptor.
///
/// Contains information on a format and its own demuxer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Descr {
    /// Format name.
    pub name: &'static str,
    /// Demuxer name.
    pub demuxer: &'static str,
    /// Format description.
    pub description: &'static str,
    /// Format media file extensions.
    pub extensions: &'static [&'static str],
    /// Format MIME.
    pub mime: &'static [&'static str],
}

/// Used to get a format descriptor and create a new demuxer.
pub trait Descriptor {
    /// The specific type of the demuxer.
    type OutputDemuxer: Demuxer;

    /// Creates a new demuxer for the requested format.
    fn create(&self) -> Self::OutputDemuxer;
    /// Returns the descriptor of a format.
    fn describe(&self) -> &Descr;

    /// Returns a score which represents how much the input data are associated
    /// to a format.
    fn probe(&self, data: &[u8]) -> u8;
}

/// Maximum data size to probe a format.
pub const PROBE_DATA: usize = 4 * 1024;

/// Data whose probe score is equal or greater than the value of this constant
/// surely is associated to the format currently being analyzed.
pub const PROBE_SCORE_EXTENSION: u8 = 50;

/// Used to define different ways to probe a format.
pub trait Probe<T: Descriptor + ?Sized> {
    /// Probes whether the input data is associated to a determined format.
    fn probe(&self, data: &[u8]) -> Option<&'static T>;
}

impl<T: Descriptor + ?Sized> Probe<T> for [&'static T] {
    fn probe(&self, data: &[u8]) -> Option<&'static T> {
        let mut max = u8::MIN;
        let mut candidate: Option<&'static T> = None;
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
            buf: &mut dyn Buffered,
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
        fn read_event(&mut self, buf: &mut dyn Buffered) -> Result<(SeekFrom, Event)> {
            let size = 2;
            let len = buf.data().len();
            if size > len {
                Err(Error::MoreDataNeeded(size - len))
            } else {
                log::debug!("{:?}", buf.data());
                match &buf.data()[..2] {
                    b"p1" => Ok((SeekFrom::Current(3), Event::NewPacket(Packet::new()))),
                    b"e1" => Ok((SeekFrom::Current(3), Event::MoreDataNeeded(0))),
                    _ => Err(Error::InvalidData),
                }
            }
        }
    }

    impl Descriptor for DummyDes {
        type OutputDemuxer = DummyDemuxer;

        fn create(&self) -> Self::OutputDemuxer {
            DummyDemuxer {}
        }
        fn describe<'a>(&'_ self) -> &'_ Descr {
            &self.d
        }
        fn probe(&self, data: &[u8]) -> u8 {
            match data {
                b"dummy" => 100,
                _ => 0,
            }
        }
    }

    const DUMMY_DES: &dyn Descriptor<OutputDemuxer = DummyDemuxer> = &DummyDes {
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
        let demuxers: &[&'static dyn Descriptor<OutputDemuxer = DummyDemuxer>] = &[DUMMY_DES];

        demuxers.probe(b"dummy").unwrap();
    }

    use crate::buffer::*;
    use std::io::Cursor;

    #[test]
    fn read_headers() {
        let buf = b"dummy header";
        let r = AccReader::with_capacity(4, Cursor::new(buf));
        let d = DUMMY_DES.create();
        let mut c = Context::new(d, r);

        c.read_headers().unwrap();
    }

    #[test]
    fn read_event() {
        let buf = b"dummy header p1 e1 p1 ";

        let r = AccReader::with_capacity(4, Cursor::new(buf));
        let d = DUMMY_DES.create();
        let mut c = Context::new(d, r);

        c.read_headers().unwrap();

        println!("{:?}", c.read_event());
        println!("{:?}", c.read_event());
        println!("{:?}", c.read_event());
        println!("{:?}", c.read_event());
    }
}
