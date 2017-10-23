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

// TODO: Document
// TODO: Change it to provide Droppable/Seekable information or use a separate enum?
#[derive(Clone, Debug, PartialEq)]
pub enum PictureType {
    UNKNOWN,
    I,
    P,
    B,
    S,
    SI,
    SP,
    SB,
    BI,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VideoInfo {
    pub pic_type: PictureType,
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

#[derive(Clone, Debug, PartialEq)]
pub struct AudioInfo {
    pub samples: usize,
    pub rate: usize,
    pub map: ChannelMap,
    pub format: Rc<Soniton>,
}

impl AudioInfo {
    fn size(&self, align: usize) -> usize {
        self.format.get_audio_size(self.samples, align) * self.map.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
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

use std::fmt;

impl fmt::Debug for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FrameBuffer")
    }
}

#[derive(Debug)]
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
            Some(plane) => Ok(&plane.buf),
        }
    }
    fn as_mut_slice<'a>(&'a mut self, idx: usize) -> Result<&'a mut [u8]> {
        match self.planes.get_mut(idx) {
            None => Err(Error::from_kind(ErrorKind::InvalidIndex)),
            Some(plane) => Ok(&mut plane.buf),
        }
    }
    fn linesize(&self, idx: usize) -> Result<usize> {
        match self.planes.get(idx) {
            None => Err(Error::from_kind(ErrorKind::InvalidIndex)),
            Some(plane) => Ok(plane.linesize),
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
                let data = unsafe {
                    Heap.alloc(Layout::from_size_align(size, ALIGNMENT).unwrap())
                        .unwrap()
                };
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
            },
            &Audio(ref audio) => {
                let size = audio.size(ALIGNMENT);
                let data = unsafe {
                    Heap.alloc(Layout::from_size_align(size, ALIGNMENT).unwrap())
                        .unwrap()
                };
                let buf = BytesMut::from(unsafe { Vec::from_raw_parts(data, size, size) });
                let mut buffer = DefaultFrameBuffer {
                    buf: buf,
                    planes: Vec::new(),
                };
                for _ in 0..audio.map.len() {
                    let size = audio.format.get_audio_size(audio.samples, ALIGNMENT);
                    buffer.planes.push(Plane {
                        buf: buffer.buf.split_to(size),
                        linesize: size,
                    });
                }
                buffer
            },
        }
    }
}

pub fn new_default_frame(kind: &MediaKind, t: Option<TimeInfo>) -> Frame {
    let buf = DefaultFrameBuffer::new(kind);

    Frame {
        kind: kind.clone(),
        buf: box buf,
        t: t,
    }
}

use std::ptr::copy_nonoverlapping;

fn copy_plane(
    dst: &mut [u8],
    dst_linesize: usize,
    src: &[u8],
    src_linesize: usize,
    w: usize,
    h: usize,
) {
    let dst_chunks = dst.chunks_mut(dst_linesize);
    let src_chunks = src.chunks(src_linesize);

    for (d, s) in dst_chunks.zip(src_chunks).take(h) {
        unsafe {
            copy_nonoverlapping(s.as_ptr(), d.as_mut_ptr(), w);
        }
    }
}

fn copy_image<'a, IM, IU, I>(
    dst: IM,
    dst_linesizes: IU,
    src: I,
    src_linesizes: IU,
    w: usize,
    h: usize,
    fmt: &Formaton,
) where
    IM: Iterator<Item = &'a mut [u8]>,
    I: Iterator<Item = &'a [u8]>,
    IU: Iterator<Item = usize>,
{
    let dst_iter = dst.zip(dst_linesizes);
    let src_iter = src.zip(src_linesizes);
    let iter = dst_iter.zip(src_iter).zip(fmt.iter());

    for (((d, d_linesize), (s, s_linesize)), c) in iter {
        copy_plane(
            d,
            d_linesize,
            s,
            s_linesize,
            c.unwrap().get_width(w),
            c.unwrap().get_height(h),
        );
    }
}

// TODO: Add proper tests
fn copy_to_frame<'a, I, IU>(dst: &mut Frame, mut src: I, mut src_linesize: IU, w: usize, h: usize)
where
    I: Iterator<Item = &'a [u8]>,
    IU: Iterator<Item = usize>,
{
    if let MediaKind::Video(ref fmt) = dst.kind {
        let mut f_iter = fmt.format.iter();
        let w = fmt.width;
        let h = fmt.height;
        for i in 0..dst.buf.count() {
            let d_linesize = dst.buf.linesize(i).unwrap();
            let s_linesize = src_linesize.next().unwrap();
            let d = dst.buf.as_mut_slice(i).unwrap();
            let s = src.next().unwrap();
            let c = f_iter.next().unwrap();
            copy_plane(
                d,
                d_linesize,
                s,
                s_linesize,
                c.unwrap().get_width(w),
                c.unwrap().get_height(h),
            );
        }
    } else {
        unimplemented!();
    }
}

use std::mem;
use std::slice;

// TODO make it a separate trait
impl Frame {
    pub fn copy_from_slice<'a, I, IU>(&mut self, mut src: I, mut src_linesize: IU)
    where
        I: Iterator<Item = &'a [u8]>,
        IU: Iterator<Item = usize>,
    {
        if let MediaKind::Video(ref fmt) = self.kind {
            let mut f_iter = fmt.format.iter();
            let w = fmt.width;
            let h = fmt.height;
            for i in 0..self.buf.count() {
                let d_linesize = self.buf.linesize(i).unwrap();
                let s_linesize = src_linesize.next().unwrap();
                let d = self.buf.as_mut_slice(i).unwrap();
                let s = src.next().unwrap();
                let c = f_iter.next().unwrap();
                copy_plane(
                    d,
                    d_linesize,
                    s,
                    s_linesize,
                    c.unwrap().get_width(w),
                    c.unwrap().get_height(h),
                );
            }
        } else {
            unimplemented!();
        }
    }

    pub fn copy_from_raw_parts<'a, I, IU>(&mut self, mut src: I, mut src_linesize: IU)
    where
        I: Iterator<Item = *const u8>,
        IU: Iterator<Item = usize>,
    {
        if let MediaKind::Video(ref fmt) = self.kind {
            let mut f_iter = fmt.format.iter();
            let w = fmt.width;
            let h = fmt.height;
            for i in 0..self.buf.count() {
                let d_linesize = self.buf.linesize(i).unwrap();
                let s_linesize = src_linesize.next().unwrap();
                let d = self.buf.as_mut_slice(i).unwrap();
                let c = f_iter.next().unwrap();
                let r = src.next().unwrap();
                let hb = c.unwrap().get_height(h);
                let s = unsafe { slice::from_raw_parts(r, hb * s_linesize) };
                copy_plane(d, d_linesize, s, s_linesize, c.unwrap().get_width(w), hb);
                mem::forget(s);
            }
        } else {
            unimplemented!();
        }
    }
}
