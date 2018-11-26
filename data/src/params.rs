use audiosample::{ChannelMap, Soniton};
use pixel::Formaton;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct VideoInfo {
    pub width: usize,
    pub height: usize,
    pub format: Option<Arc<Formaton>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioInfo {
    pub rate: usize,
    pub map: Option<ChannelMap>,
    pub format: Option<Arc<Soniton>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediaKind {
    Video(VideoInfo),
    Audio(AudioInfo),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodecParams {
    pub kind: Option<MediaKind>,
    pub codec_id: Option<String>,
    pub extradata: Option<Vec<u8>>,
    //    pub tag: Option<u32>,
    pub bit_rate: usize,
    //    pub bits_per_coded_sample: usize,
    /// Number of samples the decode must process
    /// before outputting valid data
    pub convergence_window: usize,
    /// Number of samples the codec needs to process
    /// before returning data
    pub delay: usize,
}
