use crate::rational::Rational64;
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct TimeInfo {
    pub pts: Option<i64>,
    pub dts: Option<i64>,
    pub duration: Option<u64>,
    pub timebase: Option<Rational64>,
    pub user_private: Option<Arc<dyn Any + Send + Sync>>,
}
