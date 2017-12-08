use data::rational::Rational64;
use stream::Stream;

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalInfo {
    pub duration: Option<u64>,
    pub timebase: Option<Rational64>,
    pub streams: Vec<Stream>,
}
