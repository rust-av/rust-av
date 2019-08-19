#![allow(dead_code, unused_variables)]
use bytes::BytesMut;
use std::alloc::{alloc, Layout};

use std::sync::Arc;

use crate::audiosample::*;
use crate::pixel::*;
use crate::timeinfo::*;

use byte_slice_cast::*;

#[derive(Fail, Debug)]
pub enum FrameError {
    #[fail(display = "Invalid Index")]
    InvalidIndex,
    #[fail(display = "Invalid Conversion")]
    InvalidConversion,
}

use self::FrameError::*;

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

#[derive(Clone, Debug)]
pub struct VideoInfo {
    pub pic_type: PictureType,
    pub width: usize,
    pub height: usize,
    pub format: Arc<Formaton>,
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

#[derive(Clone, Debug)]
pub struct AudioInfo {
    pub samples: usize,
    pub rate: usize,
    pub map: ChannelMap,
    pub format: Arc<Soniton>,
}

impl AudioInfo {
    fn size(&self, align: usize) -> usize {
        self.format.get_audio_size(self.samples, align) * self.map.len()
    }
}

impl PartialEq for AudioInfo {
    fn eq(&self, info2: &AudioInfo) -> bool {
        self.rate == info2.rate && self.map == info2.map && self.format == info2.format
    }
}

impl PartialEq for VideoInfo {
    fn eq(&self, info2: &VideoInfo) -> bool {
        self.width == info2.width && self.height == info2.height && self.format == info2.format
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediaKind {
    Video(VideoInfo),
    Audio(AudioInfo),
}

use std::convert::From;

impl From<VideoInfo> for MediaKind {
    fn from(v: VideoInfo) -> Self {
        MediaKind::Video(v)
    }
}

impl From<AudioInfo> for MediaKind {
    fn from(a: AudioInfo) -> Self {
        MediaKind::Audio(a)
    }
}

use self::MediaKind::*;

pub trait FrameBuffer: Send + Sync {
    fn linesize(&self, idx: usize) -> Result<usize, FrameError>;
    fn count(&self) -> usize;
    fn as_slice_inner(&self, idx: usize) -> Result<&[u8], FrameError>;
    fn as_mut_slice_inner(&mut self, idx: usize) -> Result<&mut [u8], FrameError>;
}

mod private {
    use byte_slice_cast::*;

    pub trait Supported: FromByteSlice {}
    impl Supported for u8 {}
    impl Supported for i16 {}
    impl Supported for f32 {}
}

pub trait FrameBufferConv<T: private::Supported>: FrameBuffer {
    fn as_slice(&self, idx: usize) -> Result<&[T], FrameError> {
        self.as_slice_inner(idx)?
            .as_slice_of::<T>()
            .map_err(|e| InvalidConversion)
    }
    fn as_mut_slice(&mut self, idx: usize) -> Result<&mut [T], FrameError> {
        self.as_mut_slice_inner(idx)?
            .as_mut_slice_of::<T>()
            .map_err(|e| InvalidConversion)
    }
}

impl FrameBufferConv<u8> for dyn FrameBuffer {}
impl FrameBufferConv<i16> for dyn FrameBuffer {}
impl FrameBufferConv<f32> for dyn FrameBuffer {}

use std::fmt;

impl fmt::Debug for dyn FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FrameBuffer")
    }
}

#[derive(Debug)]
pub struct Frame {
    pub kind: MediaKind,
    pub buf: Box<dyn FrameBuffer>,
    pub t: TimeInfo,
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
    fn linesize(&self, idx: usize) -> Result<usize, FrameError> {
        match self.planes.get(idx) {
            None => Err(InvalidIndex),
            Some(plane) => Ok(plane.linesize),
        }
    }
    fn count(&self) -> usize {
        self.planes.len()
    }

    fn as_slice_inner(&self, idx: usize) -> Result<&[u8], FrameError> {
        match self.planes.get(idx) {
            None => Err(InvalidIndex),
            Some(plane) => Ok(&plane.buf),
        }
    }
    fn as_mut_slice_inner(&mut self, idx: usize) -> Result<&mut [u8], FrameError> {
        match self.planes.get_mut(idx) {
            None => Err(InvalidIndex),
            Some(plane) => Ok(&mut plane.buf),
        }
    }
}

impl DefaultFrameBuffer {
    pub fn new(kind: &MediaKind) -> DefaultFrameBuffer {
        match *kind {
            Video(ref video) => {
                let size = video.size(ALIGNMENT);
                let data = unsafe { alloc(Layout::from_size_align(size, ALIGNMENT).unwrap()) };
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
                            linesize,
                        });
                    }
                }
                buffer
            }
            Audio(ref audio) => {
                let size = audio.size(ALIGNMENT);
                let data = unsafe { alloc(Layout::from_size_align(size, ALIGNMENT).unwrap()) };
                let buf = BytesMut::from(unsafe { Vec::from_raw_parts(data, size, size) });
                let mut buffer = DefaultFrameBuffer {
                    buf,
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
            }
        }
    }
}

pub type ArcFrame = Arc<Frame>;

pub fn new_default_frame<T>(kind: T, t: Option<TimeInfo>) -> Frame
where
    T: Into<MediaKind> + Clone,
{
    let k = kind.into();
    let buf = DefaultFrameBuffer::new(&k);

    Frame {
        kind: k,
        buf: Box::new(buf),
        t: t.unwrap_or_default(),
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

    pub fn copy_from_raw_parts<I, IU>(&mut self, mut src: I, mut src_linesize: IU)
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
            }
        } else {
            unimplemented!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::audiosample::formats;

    #[test]
    fn test_format_cmp() {
        let mut map = ChannelMap::new();
        map.add_channel(ChannelType::C);
        let sn = Arc::new(formats::S16);
        let info1 = AudioInfo {
            samples: 42,
            rate: 48000,
            map: map.clone(),
            format: sn,
        };
        let sn = Arc::new(formats::S16);
        let info2 = AudioInfo {
            samples: 4242,
            rate: 48000,
            map: map.clone(),
            format: sn,
        };

        assert_eq!(info1 == info2, true);

        let mut map = ChannelMap::new();
        map.add_channel(ChannelType::C);
        let sn = Arc::new(formats::S16);
        let info1 = AudioInfo {
            samples: 42,
            rate: 48000,
            map: map.clone(),
            format: sn,
        };
        let sn = Arc::new(formats::S32);
        let info2 = AudioInfo {
            samples: 42,
            rate: 48000,
            map: map.clone(),
            format: sn,
        };

        assert_eq!(info1 == info2, false);
    }

    use crate::pixel::formats::{RGB565, YUV420};

    #[test]
    fn test_video_format_cmp() {
        let yuv420: Formaton = *YUV420;
        let fm = Arc::new(yuv420);
        let info1 = VideoInfo {
            pic_type: PictureType::I,
            width: 42,
            height: 42,
            format: fm,
        };
        let yuv420: Formaton = *YUV420;
        let fm = Arc::new(yuv420);
        let info2 = VideoInfo {
            pic_type: PictureType::P,
            width: 42,
            height: 42,
            format: fm,
        };

        assert_eq!(info1 == info2, true);

        let yuv420: Formaton = *YUV420;
        let fm = Arc::new(yuv420);
        let info1 = VideoInfo {
            pic_type: PictureType::I,
            width: 42,
            height: 42,
            format: fm,
        };
        let rgb565: Formaton = *RGB565;
        let fm = Arc::new(rgb565);
        let info2 = VideoInfo {
            pic_type: PictureType::I,
            width: 42,
            height: 42,
            format: fm,
        };

        assert_eq!(info1 == info2, false);
    }
}
