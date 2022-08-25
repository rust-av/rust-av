//! Video and audio definitions.

use crate::audiosample::{ChannelMap, Soniton};
use crate::pixel::Formaton;
use std::sync::Arc;

/// Video stream information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VideoInfo {
    /// Picture width.
    pub width: usize,
    /// Picture height.
    pub height: usize,
    /// Picture pixel format.
    pub format: Option<Arc<Formaton>>,
}

/// Audio stream information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioInfo {
    /// Audio sample rate.
    pub rate: usize,
    /// Audio sequence of channels.
    pub map: Option<ChannelMap>,
    /// Audio sample format.
    pub format: Option<Arc<Soniton>>,
}

/// Possible stream information types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaKind {
    /// Video codec information.
    Video(VideoInfo),
    /// Audio codec information.
    Audio(AudioInfo),
}

/// Possible codec parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodecParams {
    /// Stream information type.
    pub kind: Option<MediaKind>,
    /// Codec id.
    pub codec_id: Option<String>,
    /// Codec additional data.
    pub extradata: Option<Vec<u8>>,
    /// Codec bit-rate.
    pub bit_rate: usize,
    /// Number of samples the decoder must process
    /// before outputting valid data.
    pub convergence_window: usize,
    /// Number of samples the codec needs to process
    /// before returning data.
    pub delay: usize,
}
