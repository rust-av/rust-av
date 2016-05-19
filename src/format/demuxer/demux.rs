use data::packet::Packet;

pub trait Demuxer {
    fn open(&mut self);
    fn read_headers(&mut self);
    fn read_packet(&mut self) -> Packet;
    fn close(&mut self);
}
