use std::io::Error;
use data::packet::Packet;

pub trait Demuxer {
    fn open(&mut self);
    fn read_headers(&mut self) -> Result<(), Error>;
    fn read_packet(&mut self) -> Result<Packet, Error>;
    fn close(&mut self);
}

pub struct DemuxerDescription {
    name: String,
    description: String,
    extensions: Vec<String>,
    mime: Vec<String>,
}

/// Least amount of data needed to check the bytestream structure
/// to match some known format.
pub const PROBE_DATA: usize = 4 * 1024;

/// Probe threshold values
pub enum Score {
    /// Minimum acceptable value, a file matched just by the extension
    EXTENSION = 50,
    /// The underlying layer provides the information, trust it up to a point
    MIME = 75,
    /// The data actually match a format structure
    MAX = 100,
}

pub trait DemuxerBuilder {
    fn describe(&self) -> &'static DemuxerDescription;
    fn probe(&self, data: &[u8; PROBE_DATA]) -> u8;
    fn alloc(&self) -> Box<Demuxer>;
}

pub fn probe<'a>(demuxers: &[&'static DemuxerBuilder],
                 data: &[u8; PROBE_DATA])
                 -> Option<&'a DemuxerBuilder> {
    let mut max = u8::min_value();
    let mut candidate: Option<&DemuxerBuilder> = None;
    for builder in demuxers {
        let score = builder.probe(data);

        if score > max {
            max = score;
            candidate = Some(*builder);
        }
    }

    if max > Score::EXTENSION as u8 {
        candidate
    } else {
        None
    }
}
