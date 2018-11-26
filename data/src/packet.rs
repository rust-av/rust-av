#![allow(dead_code)]

use std::io::{Read, Result, Write};
use timeinfo::TimeInfo;

// use data::SideData;

#[derive(Debug, Clone)]
pub struct Packet {
    pub data: Vec<u8>,
    pub pos: Option<usize>,
    pub stream_index: isize,
    pub t: TimeInfo,

    // side_data : SideData;
    pub is_key: bool,
    pub is_corrupted: bool,
}

impl Packet {
    pub fn with_capacity(capacity: usize) -> Self {
        Packet {
            data: Vec::with_capacity(capacity),
            t: TimeInfo::default(),
            pos: None,
            stream_index: -1,
            is_key: false,
            is_corrupted: false,
        }
    }

    pub fn zeroed(size: usize) -> Self {
        Packet {
            data: vec![0; size],
            t: TimeInfo::default(),
            pos: None,
            stream_index: -1,
            is_key: false,
            is_corrupted: false,
        }
    }

    pub fn new() -> Self {
        Self::with_capacity(0)
    }
}

pub trait ReadPacket: Read {
    fn get_packet(&mut self, len: usize) -> Result<Packet> {
        let mut pkt = Packet::zeroed(len);
        self.read(pkt.data.as_mut_slice())?;
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

use std::sync::Arc;

pub type ArcPacket = Arc<Packet>;

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_packet() {
        let data: Vec<u8> = (0..128).collect();
        let mut buf = Cursor::new(data.clone());

        match buf.get_packet(64) {
            Ok(pkt) => assert_eq!(pkt.data, &data[..64]),
            _ => assert!(false),
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
