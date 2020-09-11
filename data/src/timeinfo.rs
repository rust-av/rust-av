use crate::rational::Rational64;
use std::any::Any;
use std::sync::Arc;

/// Timestamp information.
#[derive(Debug, Clone, Default)]
pub struct TimeInfo {
    /// Presentation timestamp.
    pub pts: Option<i64>,
    /// Decode timestamp.
    pub dts: Option<i64>,
    /// Duration (in timebase units).
    pub duration: Option<u64>,
    /// Timebase numerator/denominator.
    pub timebase: Option<Rational64>,
    /// Timebase user private data.
    pub user_private: Option<Arc<dyn Any + Send + Sync>>,
}
