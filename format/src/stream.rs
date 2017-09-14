#![allow(dead_code)]

use std::rc::Rc;

use rational::Rational32;
use data::audiosample::{Soniton, ChannelMap};
use data::pixel::Formaton;

#[derive(Clone,Debug,PartialEq)]
pub enum CodecID {
    VP9,
    Opus
}

#[derive(Clone,Debug,PartialEq)]
pub struct VideoInfo {
    pub width: usize,
    pub height: usize,
    pub format: Option<Rc<Formaton>>,
}

#[derive(Clone,Debug,PartialEq)]
pub struct AudioInfo {
    pub samples: usize,
    pub rate: usize,
    pub map: Option<ChannelMap>,
    pub format: Option<Rc<Soniton>>,
}

#[derive(Clone,Debug,PartialEq)]
pub enum MediaKind {
    Video(VideoInfo),
    Audio(AudioInfo),
}

#[derive(Clone,Debug,PartialEq)]
pub struct CodecParams {
    pub kind: Option<MediaKind>,
    pub codec_id: Option<CodecID>,
    pub extradata: Option<Vec<u8>>,
//    pub tag: Option<u32>,
    pub bit_rate: usize,
//    pub bits_per_coded_sample: usize,
}

#[derive(Clone,Debug,PartialEq)]
pub struct Stream {
    pub id: usize,
    pub index: usize,
    pub params : CodecParams,
    pub start: Option<u64>,
    pub duration: Option<u64>,
    pub timebase : Rational32,
//  seek_index : SeekIndex
}

pub struct StreamGroup<'a> {
    pub id: usize,
    pub start: u64,
    pub end: u64,
    pub streams: &'a [Stream],
}
