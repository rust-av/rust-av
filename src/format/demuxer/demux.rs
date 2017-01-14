#![allow(dead_code)]

use std::io::Error;
use data::packet::Packet;
use format::demuxer::context::*;

pub trait Demuxer {
    fn open(&mut self);
    fn read_headers(&mut self, ctx: &mut DemuxerContext) -> Result<(), Error>;
    fn read_packet(&mut self, ctx: &mut DemuxerContext) -> Result<Packet, Error>;
}

pub struct DemuxerDescription {
    pub name:        &'static str,
    pub description: &'static str,
    pub extensions:  &'static [&'static str],
    pub mime:        &'static [&'static str],
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
    fn probe(&self, data: &[u8]) -> u8;
    fn alloc(&self) -> Box<Demuxer>;
}

pub fn probe<'a>(demuxers: &[&'static DemuxerBuilder],
                 data: &[u8])
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

#[macro_export]
macro_rules! module {
    {
        ($name:ident) {
            open($os:ident) => $ob:block
            read_headers($rhs:ident, $rhctx:ident) => $rhb:block
            read_packet($rps:ident, $rpctx:ident) => $rpb:block

            describe($ds:ident) => $db:block
            probe($ps:ident, $pd:ident) => $pb:block
            alloc($asel:ident) => $ab:block
        }
    } => {
        interpolate_idents! {
            struct [$name Demuxer];
            struct [$name DemuxerBuilder];

            impl Demuxer for [$name Demuxer] {
                fn open(&mut $os) $ob
                fn read_headers(&mut $rhs, $rhctx: &mut DemuxerContext) -> Result<(), Error> $rhb
                fn read_packet(&mut $rps, $rpctx: &mut DemuxerContext) -> Result<Packet, Error> $rpb
            }

            impl DemuxerBuilder for [$name DemuxerBuilder] {
                fn describe(&$ds) -> &'static DemuxerDescription $db
                fn probe(&$ps, $pd: &[[u8]]) -> u8 $pb
                fn alloc(&$asel) -> Box<Demuxer> $ab
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::Error;
    use data::packet::Packet;
    use format::demuxer::context::*;

    module! {
        (Test) {
            open(self) => { () }
            read_headers(self, ctx) => { Ok(()) }
            read_packet(self, ctx) => { unimplemented!() }

            describe(self) => {
                const D: &'static DemuxerDescription = &DemuxerDescription {
                    name: "Test",
                    description: "Test demuxer",
                    extensions: &["test", "t"],
                    mime: &["x-application/test"],
                };

                D
            }

            probe(self, data) => {
                if data[0] == 0 {
                    Score::MAX as u8
                } else {
                    0
                }
            }

            alloc(self) => {
                let demux = TestDemuxer {};

                box demux
            }
        }
    }

    const DEMUXER_BUILDERS: [&'static DemuxerBuilder; 1] = [&TestDemuxerBuilder {}];

    #[test]
    fn probe_demuxer() {
        let mut buf = [1; PROBE_DATA];

        match probe(&DEMUXER_BUILDERS, &buf) {
            Some(_) => panic!(),
            None => (),
        };

        buf[0] = 0;

        match probe(&DEMUXER_BUILDERS, &buf) {
            Some(_) => (),
            None => panic!(),
        };
    }
}
