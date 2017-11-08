use pixel::Formaton;
use audiosample::Soniton;

use std::rc::Rc;
use std::convert::From;

#[derive(Debug)]
pub enum Value<'a> {
    I64(i64),
    U64(u64),
    Str(&'a str),
    Bool(bool),
    Pair(i64, i64),
    Formaton(Rc<Formaton>),
    Soniton(Rc<Soniton>),
}

impl<'a> From<i64> for Value<'a> {
    fn from(v: i64) -> Self {
        Value::I64(v)
    }
}

impl<'a> From<u64> for Value<'a> {
    fn from(v: u64) -> Self {
        Value::U64(v)
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(v: &'a str) -> Self {
        Value::Str(v)
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl<'a> From<(i64, i64)> for Value<'a> {
    fn from(v: (i64, i64)) ->Self {
        Value::Pair(v.0, v.1)
    }
}

impl<'a> From<Rc<Formaton>> for Value<'a> {
    fn from(v: Rc<Formaton>) -> Self {
        Value::Formaton(v)
    }
}

impl<'a> From<Rc<Soniton>> for Value<'a> {
    fn from(v: Rc<Soniton>) -> Self {
        Value::Soniton(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;

    fn p<'a, T>(v: T) where T: Into<Value<'a>>+Debug {
        println!("{:?}", v);
    }

    #[test]
    fn value_str() {
        p("test");
    }
}
