use crate::audiosample::ChannelMap;
use crate::buffer_ref::BufferRef;
use crate::frame::{get_plane_size, AudioInfo, VideoInfo};

/// Decoded video frame.
///
/// Frames are stored in native type (8/16/32-bit elements) inside
/// a single buffer.
/// In case of image with several components those components are stored
/// sequentially and can be accessed in the buffer starting at
/// corresponding component offset.
#[derive(Clone)]
pub struct VideoBuffer<T> {
    info: VideoInfo,
    data: BufferRef<Vec<T>>,
    offs: Vec<usize>,
    strides: Vec<usize>,
}

impl<T: Clone> VideoBuffer<T> {
    /// Returns the component offset (0 for all unavailable offsets).
    pub fn get_offset(&self, idx: usize) -> usize {
        if idx >= self.offs.len() {
            0
        } else {
            self.offs[idx]
        }
    }
    /// Returns picture info.
    pub fn get_info(&self) -> VideoInfo {
        self.info.clone()
    }
    /// Returns an immutable reference to the data.
    pub fn get_data(&self) -> &Vec<T> {
        self.data.as_ref()
    }
    /// Returns a mutable reference to the data.
    pub fn get_data_mut(&mut self) -> Option<&mut Vec<T>> {
        self.data.as_mut()
    }
    /// Returns the number of components in picture format.
    pub fn get_num_components(&self) -> usize {
        self.offs.len()
    }
    /// Creates a copy of current `VideoBuffer`.
    pub fn copy_buffer(&mut self) -> Self {
        let mut data: Vec<T> = Vec::with_capacity(self.data.len());
        data.clone_from(self.data.as_ref());
        let mut offs: Vec<usize> = Vec::with_capacity(self.offs.len());
        offs.clone_from(&self.offs);
        let mut strides: Vec<usize> = Vec::with_capacity(self.strides.len());
        strides.clone_from(&self.strides);
        VideoBuffer {
            info: self.info.clone(),
            data: BufferRef::new(data),
            offs,
            strides,
        }
    }
    /// Returns stride (distance between subsequent lines)
    /// for the requested component.
    pub fn get_stride(&self, idx: usize) -> usize {
        if idx >= self.strides.len() {
            return 0;
        }
        self.strides[idx]
    }
    /// Returns requested component dimensions.
    pub fn get_dimensions(&self, idx: usize) -> (usize, usize) {
        get_plane_size(&self.info, idx)
    }
    /// Converts current instance into buffer reference.
    pub fn into_ref(self) -> BufferRef<Self> {
        BufferRef::new(self)
    }

    fn print_contents(&self, datatype: &str) {
        println!("{} video buffer size {}", datatype, self.data.len());
        println!(" format {}", self.info);
        print!(" offsets:");
        for off in self.offs.iter() {
            print!(" {}", *off);
        }
        println!();
        print!(" strides:");
        for stride in self.strides.iter() {
            print!(" {}", *stride);
        }
        println!();
    }
}

/// A specialised type for reference-counted `VideoBuffer`.
pub type VideoBufferRef<T> = BufferRef<VideoBuffer<T>>;

/// Decoded audio frame.
///
/// Frames are stored in native type (8/16/32-bit elements) inside
/// a single buffer.
/// In case of planar audio samples for each channel are stored sequentially and
/// can be accessed in the buffer starting at corresponding channel offset.
#[derive(Clone)]
pub struct AudioBuffer<T> {
    info: AudioInfo,
    data: BufferRef<Vec<T>>,
    offs: Vec<usize>,
    stride: usize,
    step: usize,
    chmap: ChannelMap,
    len: usize,
}

impl<T: Clone> AudioBuffer<T> {
    /// Returns the start position of requested channel data.
    pub fn get_offset(&self, idx: usize) -> usize {
        if idx >= self.offs.len() {
            0
        } else {
            self.offs[idx]
        }
    }
    /// Returns the distance between the start of one channel and the next one.
    pub fn get_stride(&self) -> usize {
        self.stride
    }
    /// Returns the distance between the samples in one channel.
    pub fn get_step(&self) -> usize {
        self.step
    }
    /// Returns audio format information.
    pub fn get_info(&self) -> AudioInfo {
        self.info.clone()
    }
    /// Returns channel map.
    pub fn get_chmap(&self) -> &ChannelMap {
        &self.chmap
    }
    /// Returns an immutable reference to the data.
    pub fn get_data(&self) -> &Vec<T> {
        self.data.as_ref()
    }
    /// Returns reference to the data.
    pub fn get_data_ref(&self) -> BufferRef<Vec<T>> {
        self.data.clone()
    }
    /// Returns a mutable reference to the data.
    pub fn get_data_mut(&mut self) -> Option<&mut Vec<T>> {
        self.data.as_mut()
    }
    /// Clones current `AudioBuffer` into a new one.
    pub fn copy_buffer(&mut self) -> Self {
        let mut data: Vec<T> = Vec::with_capacity(self.data.len());
        data.clone_from(self.data.as_ref());
        let mut offs: Vec<usize> = Vec::with_capacity(self.offs.len());
        offs.clone_from(&self.offs);
        AudioBuffer {
            info: self.info.clone(),
            data: BufferRef::new(data),
            offs,
            chmap: self.get_chmap().clone(),
            len: self.len,
            stride: self.stride,
            step: self.step,
        }
    }
    /// Return the length of frame in samples.
    pub fn get_length(&self) -> usize {
        self.len
    }
    /// Truncates buffer length if possible.
    ///
    /// In case when new length is larger than old length nothing is done.
    pub fn truncate(&mut self, new_len: usize) {
        self.len = self.len.min(new_len);
    }

    fn print_contents(&self, datatype: &str) {
        println!(
            "Audio buffer with {} data, stride {}, step {}",
            datatype, self.stride, self.step
        );
        println!(" format {}", self.info);
        println!(" channel map {}", self.chmap);
        print!(" offsets:");
        for off in self.offs.iter() {
            print!(" {}", *off);
        }
        println!();
    }
}

impl AudioBuffer<u8> {
    /// Constructs a new `AudioBuffer` instance.
    pub fn new_from_buf(info: AudioInfo, data: BufferRef<Vec<u8>>, chmap: ChannelMap) -> Self {
        let len = data.len();
        AudioBuffer {
            info,
            data,
            chmap,
            offs: Vec::new(),
            len,
            stride: 0,
            step: 0,
        }
    }
}

/// A list of possible decoded frame types.
#[derive(Clone)]
pub enum BufferType {
    /// 8-bit video buffer.
    Video(VideoBufferRef<u8>),
    /// 16-bit video buffer
    /// (i.e. every component or packed pixel fits into 16 bits).
    Video16(VideoBufferRef<u16>),
    /// 32-bit video buffer
    /// (i.e. every component or packed pixel fits into 32 bits).
    Video32(VideoBufferRef<u32>),
    /// Packed video buffer.
    VideoPacked(VideoBufferRef<u8>),
    /// Audio buffer with 8-bit unsigned integer audio.
    AudioU8(AudioBuffer<u8>),
    /// Audio buffer with 16-bit signed integer audio.
    AudioI16(AudioBuffer<i16>),
    /// Audio buffer with 32-bit signed integer audio.
    AudioI32(AudioBuffer<i32>),
    /// Audio buffer with 32-bit floating point audio.
    AudioF32(AudioBuffer<f32>),
    /// Packed audio buffer.
    AudioPacked(AudioBuffer<u8>),
    /// Buffer with generic data (e.g. subtitles).
    Data(BufferRef<Vec<u8>>),
    /// No data present.
    None,
}

impl BufferType {
    /// Returns the offset to the requested component or channel.
    pub fn get_offset(&self, idx: usize) -> usize {
        match *self {
            Self::Video(ref vb) => vb.get_offset(idx),
            Self::Video16(ref vb) => vb.get_offset(idx),
            Self::Video32(ref vb) => vb.get_offset(idx),
            Self::VideoPacked(ref vb) => vb.get_offset(idx),
            Self::AudioU8(ref ab) => ab.get_offset(idx),
            Self::AudioI16(ref ab) => ab.get_offset(idx),
            Self::AudioI32(ref ab) => ab.get_offset(idx),
            Self::AudioF32(ref ab) => ab.get_offset(idx),
            Self::AudioPacked(ref ab) => ab.get_offset(idx),
            _ => 0,
        }
    }
    /// Returns information for video frames.
    pub fn get_video_info(&self) -> Option<VideoInfo> {
        match *self {
            Self::Video(ref vb) => Some(vb.get_info()),
            Self::Video16(ref vb) => Some(vb.get_info()),
            Self::Video32(ref vb) => Some(vb.get_info()),
            Self::VideoPacked(ref vb) => Some(vb.get_info()),
            _ => None,
        }
    }
    /// Returns reference to 8-bit (or packed) video buffer.
    pub fn get_vbuf(&self) -> Option<VideoBufferRef<u8>> {
        match *self {
            Self::Video(ref vb) => Some(vb.clone()),
            Self::VideoPacked(ref vb) => Some(vb.clone()),
            _ => None,
        }
    }
    /// Returns reference to 16-bit video buffer.
    pub fn get_vbuf16(&self) -> Option<VideoBufferRef<u16>> {
        match *self {
            Self::Video16(ref vb) => Some(vb.clone()),
            _ => None,
        }
    }
    /// Returns reference to 32-bit video buffer.
    pub fn get_vbuf32(&self) -> Option<VideoBufferRef<u32>> {
        match *self {
            Self::Video32(ref vb) => Some(vb.clone()),
            _ => None,
        }
    }
    /// Returns information for audio frames.
    pub fn get_audio_info(&self) -> Option<AudioInfo> {
        match *self {
            Self::AudioU8(ref ab) => Some(ab.get_info()),
            Self::AudioI16(ref ab) => Some(ab.get_info()),
            Self::AudioI32(ref ab) => Some(ab.get_info()),
            Self::AudioF32(ref ab) => Some(ab.get_info()),
            Self::AudioPacked(ref ab) => Some(ab.get_info()),
            _ => None,
        }
    }
    /// Returns audio channel map.
    pub fn get_chmap(&self) -> Option<&ChannelMap> {
        match *self {
            Self::AudioU8(ref ab) => Some(ab.get_chmap()),
            Self::AudioI16(ref ab) => Some(ab.get_chmap()),
            Self::AudioI32(ref ab) => Some(ab.get_chmap()),
            Self::AudioF32(ref ab) => Some(ab.get_chmap()),
            Self::AudioPacked(ref ab) => Some(ab.get_chmap()),
            _ => None,
        }
    }
    /// Returns audio frame duration in samples.
    pub fn get_audio_length(&self) -> usize {
        match *self {
            Self::AudioU8(ref ab) => ab.get_length(),
            Self::AudioI16(ref ab) => ab.get_length(),
            Self::AudioI32(ref ab) => ab.get_length(),
            Self::AudioF32(ref ab) => ab.get_length(),
            Self::AudioPacked(ref ab) => ab.get_length(),
            _ => 0,
        }
    }
    /// Returns the distance between starts of two channels.
    pub fn get_audio_stride(&self) -> usize {
        match *self {
            Self::AudioU8(ref ab) => ab.get_stride(),
            Self::AudioI16(ref ab) => ab.get_stride(),
            Self::AudioI32(ref ab) => ab.get_stride(),
            Self::AudioF32(ref ab) => ab.get_stride(),
            Self::AudioPacked(ref ab) => ab.get_stride(),
            _ => 0,
        }
    }
    /// Returns the distance between two samples in one channel.
    pub fn get_audio_step(&self) -> usize {
        match *self {
            Self::AudioU8(ref ab) => ab.get_step(),
            Self::AudioI16(ref ab) => ab.get_step(),
            Self::AudioI32(ref ab) => ab.get_step(),
            Self::AudioF32(ref ab) => ab.get_step(),
            Self::AudioPacked(ref ab) => ab.get_step(),
            _ => 0,
        }
    }
    /// Returns reference to 8-bit (or packed) audio buffer.
    pub fn get_abuf_u8(&self) -> Option<AudioBuffer<u8>> {
        match *self {
            Self::AudioU8(ref ab) => Some(ab.clone()),
            Self::AudioPacked(ref ab) => Some(ab.clone()),
            _ => None,
        }
    }
    /// Returns reference to 16-bit audio buffer.
    pub fn get_abuf_i16(&self) -> Option<AudioBuffer<i16>> {
        match *self {
            Self::AudioI16(ref ab) => Some(ab.clone()),
            _ => None,
        }
    }
    /// Returns reference to 32-bit integer audio buffer.
    pub fn get_abuf_i32(&self) -> Option<AudioBuffer<i32>> {
        match *self {
            Self::AudioI32(ref ab) => Some(ab.clone()),
            _ => None,
        }
    }
    /// Returns reference to 32-bit floating point audio buffer.
    pub fn get_abuf_f32(&self) -> Option<AudioBuffer<f32>> {
        match *self {
            Self::AudioF32(ref ab) => Some(ab.clone()),
            _ => None,
        }
    }
    /// Prints internal buffer layout.
    pub fn print_buffer_metadata(&self) {
        match *self {
            Self::Video(ref buf) => buf.print_contents("8-bit"),
            Self::Video16(ref buf) => buf.print_contents("16-bit"),
            Self::Video32(ref buf) => buf.print_contents("32-bit"),
            Self::VideoPacked(ref buf) => buf.print_contents("packed"),
            Self::AudioU8(ref buf) => buf.print_contents("8-bit unsigned integer"),
            Self::AudioI16(ref buf) => buf.print_contents("16-bit integer"),
            Self::AudioI32(ref buf) => buf.print_contents("32-bit integer"),
            Self::AudioF32(ref buf) => buf.print_contents("32-bit float"),
            Self::AudioPacked(ref buf) => buf.print_contents("packed"),
            Self::Data(ref buf) => {
                println!("Data buffer, len = {}", buf.len());
            }
            Self::None => {
                println!("No buffer");
            }
        };
    }
}
