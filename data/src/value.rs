//! Option values definitions.

use crate::audiosample::Soniton;
use crate::pixel::Formaton;

use std::convert::From;
use std::sync::Arc;

/// Accepted option values.
#[derive(Debug)]
pub enum Value<'a> {
    /// Signed integer value.
    I64(i64),
    /// Unsigned integer value.
    U64(u64),
    /// Unicode string slice value.
    Str(&'a str),
    /// Boolean value.
    Bool(bool),
    /// Pair of signed integer values.
    Pair(i64, i64),
    /// Image colorspace representation value.
    Formaton(Arc<Formaton>),
    /// Audio format definition value.
    Soniton(Arc<Soniton>),
}

impl From<i64> for Value<'_> {
    fn from(v: i64) -> Self {
        Value::I64(v)
    }
}

impl From<u64> for Value<'_> {
    fn from(v: u64) -> Self {
        Value::U64(v)
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(v: &'a str) -> Self {
        Value::Str(v)
    }
}

impl From<bool> for Value<'_> {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<(i64, i64)> for Value<'_> {
    fn from(v: (i64, i64)) -> Self {
        Value::Pair(v.0, v.1)
    }
}

impl From<Arc<Formaton>> for Value<'_> {
    fn from(v: Arc<Formaton>) -> Self {
        Value::Formaton(v)
    }
}

impl From<Arc<Soniton>> for Value<'_> {
    fn from(v: Arc<Soniton>) -> Self {
        Value::Soniton(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;

    fn p<'a, T>(v: T)
    where
        T: Into<Value<'a>> + Debug,
    {
        println!("{:?}", v);
    }

    #[test]
    fn value_str() {
        p("test");
    }
}
