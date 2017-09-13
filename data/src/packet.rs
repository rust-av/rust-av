#![allow(dead_code)]

use std::io::{Read, Write, Result};

// use data::SideData;

bitflags! {
    flags PacketFlags: u32 {
        const KEY     = 0b0001,
        const CORRUPT = 0b0010,
        const NONE    = 0,
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    data : Vec<u8>,
    pts : Option<i64>,
    dts : Option<i64>,
    pos : Option<i64>,
    stream_index : isize,

    // side_data : SideData;

    flags : PacketFlags,
}

impl Packet {
    pub fn with_capacity(capacity: usize) -> Self {
        Packet {
            data : Vec::with_capacity(capacity),
            pts : None,
            dts : None,
            pos : None,
            stream_index : -1,
            flags : NONE
        }
    }

    pub fn new() -> Self {
        Self::with_capacity(0)
    }
}

pub trait ReadPacket: Read {
    fn get_packet(&mut self, len: usize) -> Result<Packet> {
        let mut pkt = Packet::with_capacity(len);
        unsafe {
            pkt.data.set_len(len);
            try!(self.read(pkt.data.as_mut_slice()));
        }
        Ok(pkt)
    }
}

pub trait WritePacket: Write {
    fn put_packet(&mut self, pkt: Packet) -> Result<()> {
        self.write_all(pkt.data.as_slice())
    }
}

impl<R: Read + ?Sized> ReadPacket for R {}
impl<W: Write + ?Sized> WritePacket for W {}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn read_packet() {
        let mut buf = Cursor::new(vec![0; 1024]);

        match buf.get_packet(1024) {
            Ok(pkt) => assert!(pkt.data[0] == 0),
            _ => assert!(false)
        }
    }

    #[test]
    fn write_packet() {
        let size = 1024;
        let mut buf = Cursor::new(Vec::with_capacity(size));

        let mut pkt = Packet::with_capacity(size);

        for i in 0..size {
            pkt.data.push(i as u8);
        }

        buf.put_packet(pkt).unwrap();

        let vec = buf.into_inner();

        for i in 0..size {
            println!("{}", vec[i]);
            assert!(vec[i] == i as u8);
        }
    }
}
