#![allow(dead_code, unused_variables)]

use std::alloc::{alloc, Layout};
use std::convert::From;
use std::fmt;
use std::ptr::copy_nonoverlapping;
use std::slice;
use std::sync::Arc;

use byte_slice_cast::*;
use bytes::BytesMut;
use thiserror::Error;

use crate::audiosample::*;
use crate::pixel::*;
use crate::timeinfo::*;

use self::FrameError::*;

/// Frame errors.
#[derive(Debug, Error)]
pub enum FrameError {
    /// Invalid frame index.
    #[error("Invalid Index")]
    InvalidIndex,
    /// Invalid frame conversion.
    #[error("Invalid Conversion")]
    InvalidConversion,
}

// TODO: Change it to provide Droppable/Seekable information or use a separate enum?
/// A list of recognized frame types.
#[derive(Clone, Debug, PartialEq)]
pub enum FrameType {
    /// Intra frame type.
    I,
    /// Inter frame type.
    P,
    /// Bidirectionally predicted frame.
    B,
    /// Skip frame.
    ///
    /// When such frame is encountered, then last frame should be used again
    /// if it is needed.
    SKIP,
    /// Some other frame type.
    OTHER,
}

impl fmt::Display for FrameType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FrameType::I => write!(f, "I"),
            FrameType::P => write!(f, "P"),
            FrameType::B => write!(f, "B"),
            FrameType::SKIP => write!(f, "Skip"),
            FrameType::OTHER => write!(f, "x"),
        }
    }
}

/// Video stream information.
#[derive(Clone, Debug)]
pub struct VideoInfo {
    /// Frame width.
    pub width: usize,
    /// Frame height.
    pub height: usize,
    /// Frame is stored downside up.
    pub flipped: bool,
    /// Frame type.
    pub frame_type: FrameType,
    /// Frame pixel format.
    pub format: Arc<Formaton>,
    /// Declared bits per sample.
    pub bits: u8,
}

impl VideoInfo {
    /// Constructs a new `VideoInfo` instance.
    pub fn new(
        width: usize,
        height: usize,
        flipped: bool,
        frame_type: FrameType,
        format: Arc<Formaton>,
    ) -> Self {
        let bits = format.get_total_depth();
        VideoInfo {
            width,
            height,
            flipped,
            frame_type,
            format,
            bits,
        }
    }

    /// Returns frame width.
    pub fn get_width(&self) -> usize {
        self.width
    }
    /// Returns frame height.
    pub fn get_height(&self) -> usize {
        self.height
    }
    /// Returns frame orientation.
    pub fn is_flipped(&self) -> bool {
        self.flipped
    }
    /// Returns frame type.
    pub fn get_frame_type(&self) -> &FrameType {
        &self.frame_type
    }
    /// Returns frame pixel format.
    pub fn get_format(&self) -> Formaton {
        *self.format
    }

    /// Sets new frame width.
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }
    /// Sets new frame height.
    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    /// Returns video stream size with the specified alignment.
    pub fn size(&self, align: usize) -> usize {
        let mut size = 0;
        for &component in self.format.into_iter() {
            if let Some(c) = component {
                size += c.get_data_size(self.width, self.height, align);
            }
        }
        size
    }
}

impl fmt::Display for VideoInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl PartialEq for VideoInfo {
    fn eq(&self, info2: &VideoInfo) -> bool {
        self.width == info2.width && self.height == info2.height && self.format == info2.format
    }
}

/// Audio stream information contained in a frame.
#[derive(Clone, Debug)]
pub struct AudioInfo {
    /// Number of samples.
    pub samples: usize,
    /// Sample rate.
    pub sample_rate: usize,
    /// Sequence of stream channels.
    pub map: ChannelMap,
    /// Audio sample format.
    pub format: Arc<Soniton>,
    /// Length of one audio block in samples.
    ///
    /// None if not present.
    pub block_len: Option<usize>,
}

impl AudioInfo {
    /// Constructs a new `AudioInfo` instance.
    pub fn new(
        samples: usize,
        sample_rate: usize,
        map: ChannelMap,
        format: Arc<Soniton>,
        block_len: Option<usize>,
    ) -> Self {
        AudioInfo {
            samples,
            sample_rate,
            map,
            format,
            block_len,
        }
    }
    /// Returns audio sample rate.
    pub fn get_sample_rate(&self) -> usize {
        self.sample_rate
    }
    /// Returns the number of channels.
    pub fn get_channels_number(&self) -> usize {
        self.map.len()
    }
    /// Returns sample format.
    pub fn get_format(&self) -> Soniton {
        *self.format
    }
    /// Returns number of samples.
    pub fn get_samples(&self) -> usize {
        self.samples
    }

    /// Returns one audio block duration in samples.
    pub fn get_block_len(&self) -> Option<usize> {
        self.block_len
    }

    /// Returns audio stream size with the specified alignment.
    pub fn size(&self, align: usize) -> usize {
        self.format.get_audio_size(self.samples, align) * self.map.len()
    }
}

impl fmt::Display for AudioInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} Hz, {} ch",
            self.sample_rate,
            self.get_channels_number()
        )
    }
}

impl PartialEq for AudioInfo {
    fn eq(&self, info2: &AudioInfo) -> bool {
        self.sample_rate == info2.sample_rate
            && self.map == info2.map
            && self.format == info2.format
    }
}

/// A list of possible stream information types.
#[derive(Clone, Debug, PartialEq)]
pub enum MediaKind {
    /// Video codec information.
    Video(VideoInfo),
    /// Audio codec information.
    Audio(AudioInfo),
}

impl MediaKind {
    /// Returns video stream information.
    pub fn get_video_info(&self) -> Option<VideoInfo> {
        if let MediaKind::Video(vinfo) = self {
            Some(vinfo.clone())
        } else {
            None
        }
    }
    /// Returns audio stream information.
    pub fn get_audio_info(&self) -> Option<AudioInfo> {
        if let MediaKind::Audio(ainfo) = self {
            Some(ainfo.clone())
        } else {
            None
        }
    }
    /// Reports whether the current stream is video stream.
    pub fn is_video(&self) -> bool {
        if let MediaKind::Video(_) = self {
            true
        } else {
            false
        }
    }
    /// Reports whether the current stream is audio stream.
    pub fn is_audio(&self) -> bool {
        if let MediaKind::Audio(_) = self {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for MediaKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ret = match self {
            MediaKind::Audio(fmt) => format!("{}", fmt),
            MediaKind::Video(fmt) => format!("{}", fmt),
        };
        write!(f, "{}", ret)
    }
}

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
            None => Err(FrameError::InvalidIndex),
            Some(plane) => Ok(plane.linesize),
        }
    }
    fn count(&self) -> usize {
        self.planes.len()
    }

    fn as_slice_inner(&self, idx: usize) -> Result<&[u8], FrameError> {
        match self.planes.get(idx) {
            None => Err(FrameError::InvalidIndex),
            Some(plane) => Ok(&plane.buf),
        }
    }
    fn as_mut_slice_inner(&mut self, idx: usize) -> Result<&mut [u8], FrameError> {
        match self.planes.get_mut(idx) {
            None => Err(FrameError::InvalidIndex),
            Some(plane) => Ok(&mut plane.buf),
        }
    }
}

impl DefaultFrameBuffer {
    pub fn new(kind: &MediaKind) -> DefaultFrameBuffer {
        match *kind {
            MediaKind::Video(ref video) => {
                let size = video.size(ALIGNMENT);
                let data = unsafe { alloc(Layout::from_size_align(size, ALIGNMENT).unwrap()) };
                //let data = unsafe { Heap.alloc_zeroed(Layout::from_size_align(size, ALIGNMENT)) };
                let buf = BytesMut::from(unsafe { &Vec::from_raw_parts(data, size, size)[..] });
                let mut buffer = DefaultFrameBuffer {
                    buf,
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
            MediaKind::Audio(ref audio) => {
                let size = audio.size(ALIGNMENT);
                let data = unsafe { alloc(Layout::from_size_align(size, ALIGNMENT).unwrap()) };
                let buf = BytesMut::from(unsafe { &Vec::from_raw_parts(data, size, size)[..] });
                let mut buffer = DefaultFrameBuffer {
                    buf,
                    planes: Vec::new(),
                };
                if audio.format.planar {
                    for _ in 0..audio.map.len() {
                        let size = audio.format.get_audio_size(audio.samples, ALIGNMENT);
                        buffer.planes.push(Plane {
                            buf: buffer.buf.split_to(size),
                            linesize: size,
                        });
                    }
                } else {
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
fn copy_to_frame<'a, I, IU>(
    dst: &mut Frame,
    mut src: I,
    mut src_linesize: IU,
    width: usize,
    height: usize,
) where
    I: Iterator<Item = &'a [u8]>,
    IU: Iterator<Item = usize>,
{
    if let MediaKind::Video(ref fmt) = dst.kind {
        let mut f_iter = fmt.format.iter();
        let width = fmt.width;
        let height = fmt.height;
        for i in 0..dst.buf.count() {
            let d_linesize = dst.buf.linesize(i).unwrap();
            let s_linesize = src_linesize.next().unwrap();
            let data = dst.buf.as_mut_slice(i).unwrap();
            let ss = src.next().unwrap();
            let cc = f_iter.next().unwrap();
            copy_plane(
                data,
                d_linesize,
                ss,
                s_linesize,
                cc.unwrap().get_width(width),
                cc.unwrap().get_height(height),
            );
        }
    } else {
        unimplemented!();
    }
}

// TODO make it a separate trait
impl Frame {
    pub fn copy_from_slice<'a, I, IU>(&mut self, mut src: I, mut src_linesize: IU)
    where
        I: Iterator<Item = &'a [u8]>,
        IU: Iterator<Item = usize>,
    {
        if let MediaKind::Video(ref fmt) = self.kind {
            let mut f_iter = fmt.format.iter();
            let width = fmt.width;
            let height = fmt.height;
            for i in 0..self.buf.count() {
                let d_linesize = self.buf.linesize(i).unwrap();
                let s_linesize = src_linesize.next().unwrap();
                let data = self.buf.as_mut_slice(i).unwrap();
                let ss = src.next().unwrap();
                let cc = f_iter.next().unwrap();
                copy_plane(
                    data,
                    d_linesize,
                    ss,
                    s_linesize,
                    cc.unwrap().get_width(width),
                    cc.unwrap().get_height(height),
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
            let width = fmt.width;
            let height = fmt.height;
            for i in 0..self.buf.count() {
                let d_linesize = self.buf.linesize(i).unwrap();
                let s_linesize = src_linesize.next().unwrap();
                let data = self.buf.as_mut_slice(i).unwrap();
                let cc = f_iter.next().unwrap();
                let rr = src.next().unwrap();
                let hb = cc.unwrap().get_height(height);
                let ss = unsafe { slice::from_raw_parts(rr, hb * s_linesize) };
                copy_plane(
                    data,
                    d_linesize,
                    ss,
                    s_linesize,
                    cc.unwrap().get_width(width),
                    hb,
                );
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
        let info1 = AudioInfo::new(42, 48000, map.clone(), sn, None);

        let sn = Arc::new(formats::S16);
        let info2 = AudioInfo::new(4242, 48000, map.clone(), sn, None);

        assert_eq!(info1 == info2, true);

        let mut map = ChannelMap::new();
        map.add_channel(ChannelType::C);
        let sn = Arc::new(formats::S16);
        let info1 = AudioInfo::new(42, 48000, map.clone(), sn, None);

        let sn = Arc::new(formats::S32);
        let info2 = AudioInfo::new(42, 48000, map.clone(), sn, None);

        assert_eq!(info1 == info2, false);
    }

    use crate::pixel::formats::{RGB565, YUV420};

    #[test]
    fn test_video_format_cmp() {
        let yuv420: Formaton = *YUV420;
        let fm = Arc::new(yuv420);
        let info1 = VideoInfo::new(42, 42, false, FrameType::I, fm);

        let yuv420: Formaton = *YUV420;
        let fm = Arc::new(yuv420);
        let info2 = VideoInfo::new(42, 42, false, FrameType::P, fm);

        assert_eq!(info1 == info2, true);

        let yuv420: Formaton = *YUV420;
        let fm = Arc::new(yuv420);
        let info1 = VideoInfo::new(42, 42, false, FrameType::I, fm);

        let rgb565: Formaton = *RGB565;
        let fm = Arc::new(rgb565);
        let info2 = VideoInfo::new(42, 42, false, FrameType::I, fm);

        assert_eq!(info1 == info2, false);
    }
}
