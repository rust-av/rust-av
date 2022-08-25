//! Packets definitions.

#![allow(dead_code)]

use crate::timeinfo::TimeInfo;
use std::io::{Read, Result, Write};

/// Packet with compressed data.
#[derive(Default, Debug, Clone)]
pub struct Packet {
    /// Packet data.
    pub data: Vec<u8>,
    /// Packet position in the stream.
    ///
    /// If `None`, the packet is not associated to a stream.
    pub pos: Option<usize>,
    /// Type of stream the packet is associated to.
    pub stream_index: isize,
    /// Packet timestamp information.
    pub t: TimeInfo,

    /// Tells whether a packet contains a keyframe.
    pub is_key: bool,
    /// Tells whether a packet is corrupted.
    pub is_corrupted: bool,
}

impl Packet {
    /// Creates a new empty `Packet` of a determined capacity.
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

    /// Creates a zero-initalized `Packet` of a determined capacity.
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

    /// Creates a new empty `Packet`.
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
}

/// Used to read a packet from a source.
pub trait ReadPacket: Read {
    /// Reads a packet from a source.
    fn get_packet(&mut self, len: usize) -> Result<Packet> {
        let mut pkt = Packet::zeroed(len);
        self.read_exact(pkt.data.as_mut_slice())?;
        Ok(pkt)
    }
}

/// Used to write a packet into a source.
pub trait WritePacket: Write {
    /// Writes a packet into a source.
    fn put_packet(&mut self, pkt: Packet) -> Result<()> {
        self.write_all(pkt.data.as_slice())
    }
}

impl<R: Read + ?Sized> ReadPacket for R {}
impl<W: Write + ?Sized> WritePacket for W {}

use std::sync::Arc;

/// A specialized type for a thread-safe reference-counting pointer `Packet`.
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
            _ => unreachable!(),
        }
    }

    /*#[test]
    fn test_new(){
        let pkt = Packet::new();
        assert_eq!(0, pkt.data.len());
    }*/

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

        for (i, elem) in vec.iter().enumerate().take(size) {
            println!("{}", elem);
            assert!(*elem == i as u8);
        }
    }
}
