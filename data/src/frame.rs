#![allow(dead_code, unused_variables)]
use alloc::heap::{Heap, Alloc, Layout};
use bytes::BytesMut;

use std::rc::Rc;

use audiosample::*;
use pixel::*;
use timeinfo::*;

error_chain! {
    errors {
        InvalidIndex
    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct VideoInfo {
    pub width: usize,
    pub height: usize,
    pub format: Rc<Formaton>,
}

impl VideoInfo {
    fn size(&self, align: usize) -> usize {
        let mut size = 0;
        for &component in self.format.into_iter() {
            if let Some(c) = component {
                size += c.get_data_size(self.width, self.height, align);
            }
        }
        size
    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct AudioInfo {
    pub samples: usize,
    pub rate: usize,
    pub map: ChannelMap,
    pub format: Rc<Soniton>,
}

impl AudioInfo {
    fn size(&self, align: usize) -> usize {
        unimplemented!()
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum MediaKind {
    Video(VideoInfo),
    Audio(AudioInfo),
}

use self::MediaKind::*;


pub trait FrameBuffer {
    fn as_slice<'a>(&'a self, idx: usize) -> Result<&'a [u8]>;
    fn as_mut_slice<'a>(&'a mut self, idx: usize) -> Result<&'a mut [u8]>;
    fn linesize(&self, idx: usize) -> Result<usize>;
    fn count(&self) -> usize;
}

pub struct Frame {
    pub kind: MediaKind,
    pub buf: Box<FrameBuffer>,
    pub t: Option<TimeInfo>,
}

const ALIGNMENT: usize = 32;

struct Plane {
    buf: BytesMut,
    linesize: usize,
}

struct DefaultFrameBuffer {
    buf: BytesMut,
    planes: Vec<Plane>,
}

impl FrameBuffer for DefaultFrameBuffer {
    fn as_slice<'a>(&'a self, idx: usize) -> Result<&'a [u8]> {
        match self.planes.get(idx) {
            None => Err(Error::from_kind(ErrorKind::InvalidIndex)),
            Some(plane) => Ok(&plane.buf)
        }
    }
    fn as_mut_slice<'a>(&'a mut self, idx: usize) -> Result<&'a mut [u8]> {
        match self.planes.get_mut(idx) {
            None => Err(Error::from_kind(ErrorKind::InvalidIndex)),
            Some(plane) => Ok(&mut plane.buf)
        }
    }
    fn linesize(&self, idx: usize) -> Result<usize> {
        match self.planes.get(idx) {
            None => Err(Error::from_kind(ErrorKind::InvalidIndex)),
            Some(plane) => Ok(plane.linesize)
        }
    }
    fn count(&self) -> usize {
        self.planes.len()
    }
}

impl DefaultFrameBuffer {
    pub fn new(kind: &MediaKind) -> DefaultFrameBuffer {
        match kind {
            &Video(ref video) => {
                let size = video.size(ALIGNMENT);
                let data = unsafe { Heap.alloc(Layout::from_size_align(size, ALIGNMENT).unwrap()).unwrap() };
                //let data = unsafe { Heap.alloc_zeroed(Layout::from_size_align(size, ALIGNMENT)) };
                let buf = BytesMut::from(unsafe { Vec::from_raw_parts(data, size, size) });
                let mut buffer = DefaultFrameBuffer {
                    buf: buf,
                    planes: Vec::new(),
                };
                for &component in video.format.iter() {
                    if let Some(c) = component {
                        let planesize = c.get_data_size(video.width, video.height, ALIGNMENT);
                        let linesize = c.get_linesize(video.width, ALIGNMENT);
                        buffer.planes.push(Plane {
                            buf: buffer.buf.split_to(planesize),
                            linesize: linesize,
                        });
                    }
                }
                buffer
            }
            _ => unimplemented!(),
        }
    }
}

pub fn new_default_frame(kind: &MediaKind, t: Option<TimeInfo>) -> Frame {
    let buf = DefaultFrameBuffer::new(kind);

    Frame { kind: kind.clone(), buf: box buf, t: t }
}
