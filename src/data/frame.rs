use data::timeinfo::*;
use data::pixel::*;
use data::audiosample::*;

error_chain! {
    errors {
        InvalidIndex
    }
}

pub struct VideoFrame {
    pub width : usize,
    pub height: usize,
    pub format: Box<Formaton>,
}

pub struct AudioFrame {
    pub rate: usize,
    pub map: ChannelMap,
    pub format: Box<Soniton>,
}

enum FrameKind {
    Video(VideoFrame),
    Audio(AudioFrame),
    // extend as needed
}

pub trait FrameBuffer {
    fn as_slice<'a>(&'a self, idx: usize) -> Result<&'a [u8]>;
    fn as_mut_slice<'a>(&'a mut self, idx: usize) -> Result<&'a mut[u8]>;
}

pub struct Frame {
    kind : FrameKind,
    buf : Box<FrameBuffer>,
    t : TimeInfo,
}
