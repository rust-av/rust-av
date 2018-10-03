use data::params::CodecParams;
use rational::Rational64;
use std::any::Any;
use std::sync::Arc;

/*
#[derive(Debug)]
pub struct UserPrivate(Option<Box<Any + Send>>);

impl UserPrivate {
    pub fn new<T: Clone + Any + Send>(x: T) -> Self {
        UserPrivate(Some(Box::new(x)))
    }
    pub fn is<T: 'static>(&self) -> bool {
        self.0.as_ref().map_or(false, |a| a.is::<T>())
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.0.as_ref().map_or(None, |a| a.downcast_ref::<T>())
    }
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.as_mut().map_or(None, |a| a.downcast_mut::<T>())
    }
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

impl Clone for UserPrivate {
    fn clone(&self) -> Self {
        UserPrivate(None)
    }
}

impl PartialEq for UserPrivate {
    fn eq(&self, other: &UserPrivate) -> bool {
        self.0.is_none() && other.0.is_none()
    }
}
*/

#[derive(Debug, Clone)]
pub struct Stream {
    /// Format-specific track identifier.
    ///
    /// Negative if not supported by the underlying format or if the
    /// default progression should be used.
    ///
    /// Must be unique
    pub id: isize,
    pub index: usize,
    pub params: CodecParams,
    pub start: Option<u64>,
    pub duration: Option<u64>,
    pub timebase: Rational64,
    /// User Private field, will not be cloned
    pub user_private: Option<Arc<dyn Any + Send + Sync>>,
    //  seek_index : SeekIndex
}

impl Stream {
    pub fn from_params(params: &CodecParams, timebase: Rational64) -> Self {
        Stream {
            id: -1,
            index: 0,
            params: params.clone(),
            start: None,
            duration: None,
            timebase,
            user_private: None,
        }
    }
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
