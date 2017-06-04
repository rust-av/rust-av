use data::timeinfo::*;
use data::pixel::*;
use data::audiosample::*;

use alloc::heap::allocate;
use bytes::Bytes;
use std::ptr::write_bytes;

error_chain! {
    errors {
        InvalidIndex
    }
}

pub struct VideoFrame {
    pub width: usize,
    pub height: usize,
    pub format: Box<Formaton>,
}

impl VideoFrame {
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

pub struct AudioFrame {
    pub samples: usize,
    pub rate: usize,
    pub map: ChannelMap,
    pub format: Box<Soniton>,
}

impl AudioFrame {
    fn size(&self, align: usize) -> usize {
        unimplemented!()
    }
}

pub enum FrameKind {
    Video(VideoFrame),
    Audio(AudioFrame),
}

use self::FrameKind::*;


pub trait FrameBuffer {
    fn as_slice<'a>(&'a self, idx: usize) -> Result<&'a [u8]>;
    fn as_mut_slice<'a>(&'a mut self, idx: usize) -> Result<&'a mut [u8]>;
    fn linesize(&self, idx: usize) -> usize;
    fn count(&self) -> usize;
}

pub struct Frame {
    kind: FrameKind,
    buf: Box<FrameBuffer>,
    t: TimeInfo,
}

const ALIGNMENT: usize = 32;

struct Plane {
    buf: Bytes,
    linesize: usize,
}

struct DefaultFrameBuffer {
    buf: Bytes,
    planes: Vec<Plane>,
}

impl DefaultFrameBuffer {
    pub fn new(kind: &FrameKind) -> DefaultFrameBuffer {
        match kind {
            &Video(ref video) => {
                let size = video.size(ALIGNMENT);
                let data = unsafe { allocate(size, ALIGNMENT) };
                // unsafe { write_bytes(data, 0, size) };
                let mut buf = Bytes::from(unsafe { Vec::from_raw_parts(data, size, size) });
                let mut buffer = DefaultFrameBuffer {
                    buf: buf,
                    planes: Vec::new(),
                };
                for &component in video.format.into_iter() {
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
