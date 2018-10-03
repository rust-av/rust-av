use rational::Rational64;
use std::sync::Arc;
use std::any::Any;

#[derive(Debug, Clone, Default)]
pub struct TimeInfo {
    pub pts: Option<i64>,
    pub dts: Option<i64>,
    pub duration: Option<u64>,
    pub timebase: Option<Rational64>,
    pub user_private: Option<Arc<dyn Any + Send + Sync>>,
}
