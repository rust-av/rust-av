use data::packet::Packet;

pub trait Demuxer {
    fn open(&mut self);
    fn read_headers(&mut self);
    fn read_packet(&mut self) -> Packet;
    fn close(&mut self);
}

pub struct DemuxerDescription {
    name : str,
    description : str,
    extensions : Vec<str>,
    mime : Vec<str>,
}

pub const PROBE_DATA = 4 * 1024;

pub trait DemuxerBuilder {
    fn describe(&self) -> &'static DemuxerDescription;
    fn probe(&self, data: &[u8; PROBE_DATA]) -> u8;
    fn alloc(&self) -> Box<Demuxer>;
}

fn probe(demuxers: &[&DemuxerBuilder], data: &[u8; PROBE_DATA]) -> Option<&DemuxerBuilder> {
    let max = u8.min_value();
    let mut candidate : Option<&DemuxerBuilder> = None ;
    for builder in demuxers {
        let score = builder.probe(data);

        if score > max {
            max = score;
            candidate = Some(builder);
        }
    }

    candidate
}
