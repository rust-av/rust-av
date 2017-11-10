use rational::Rational64;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TimeInfo {
    pub pts: Option<i64>,
    pub dts: Option<i64>,
    pub duration: Option<u64>,
    pub timebase: Option<Rational64>,
}
