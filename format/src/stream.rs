use rational::Rational64;
use data::params::CodecParams;

#[derive(Clone,Debug,PartialEq)]
pub struct Stream {
    pub id: usize,
    pub index: usize,
    pub params : CodecParams,
    pub start: Option<u64>,
    pub duration: Option<u64>,
    pub timebase : Rational64,
//  seek_index : SeekIndex
}

impl Stream {
    pub fn get_extradata<'a>(&'a self) -> Option<&'a [u8]> {
        self.params.extradata.as_ref().map(|e| e.as_slice())
    }
}

pub struct StreamGroup<'a> {
    pub id: usize,
    pub start: u64,
    pub end: u64,
    pub streams: &'a [Stream],
}
