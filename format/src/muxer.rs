use crate::common::*;
use crate::data::packet::Packet;
use crate::data::value::*;
use std::any::Any;
use std::io::Write;
use std::sync::Arc;

use crate::error::*;

pub trait Muxer: Send {
    fn configure(&mut self) -> Result<()>;
    fn write_header(&mut self, out: &mut dyn Write) -> Result<()>;
    fn write_packet(&mut self, out: &mut dyn Write, pkt: Arc<Packet>) -> Result<()>;
    fn write_trailer(&mut self, out: &mut dyn Write) -> Result<()>;

    fn set_global_info(&mut self, info: GlobalInfo) -> Result<()>;
    fn set_option<'a>(&mut self, key: &str, val: Value<'a>) -> Result<()>;
}

pub struct Context {
    muxer: Box<dyn Muxer + Send>,
    writer: Box<dyn Write + Send>,
    buf: Vec<u8>,
    pub user_private: Option<Box<dyn Any + Send + Sync>>,
}

impl Context {
    pub fn new<W: Write + 'static + Send>(muxer: Box<dyn Muxer + Send>, writer: Box<W>) -> Self {
        Context {
            muxer,
            writer,
            buf: Vec::new(),
            user_private: None,
        }
    }

    pub fn configure(&mut self) -> Result<()> {
        self.muxer.configure()
    }

    pub fn write_header(&mut self) -> Result<usize> {
        self.muxer.write_header(&mut self.buf)?;
        //FIXME: we should have proper management of the buffer's index
        match self.writer.write_all(&self.buf) {
            Ok(()) => {
                let len = self.buf.len();
                self.buf.clear();
                Ok(len)
            }
            Err(e) => Err(Error::Io(e)),
        }
    }

    pub fn write_packet(&mut self, pkt: Arc<Packet>) -> Result<usize> {
        self.muxer.write_packet(&mut self.buf, pkt)?;
        //FIXME: we should have proper management of the buffer's index
        match self.writer.write_all(&self.buf) {
            Ok(()) => {
                let len = self.buf.len();
                self.buf.clear();
                Ok(len)
            }
            Err(e) => Err(Error::Io(e)),
        }
    }

    pub fn write_trailer(&mut self) -> Result<usize> {
        self.muxer.write_trailer(&mut self.buf)?;
        //FIXME: we should have proper management of the buffer's index
        match self.writer.write_all(&self.buf) {
            Ok(()) => {
                let len = self.buf.len();
                self.buf.clear();
                Ok(len)
            }
            Err(e) => Err(Error::Io(e)),
        }
    }

    pub fn set_global_info(&mut self, info: GlobalInfo) -> Result<()> {
        self.muxer.set_global_info(info)
    }

    pub fn set_option<'a, V>(&mut self, key: &str, val: V) -> Result<()>
    where
        V: Into<Value<'a>>,
    {
        self.muxer.set_option(key, val.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Descr {
    pub name: &'static str,
    pub demuxer: &'static str,
    pub description: &'static str,
    pub extensions: &'static [&'static str],
    pub mime: &'static [&'static str],
}

pub trait Descriptor {
    fn create(&self) -> Box<dyn Muxer>;
    fn describe(&self) -> &Descr;
}

pub trait Lookup {
    fn by_name(&self, name: &str) -> Option<&'static dyn Descriptor>;
}

impl Lookup for [&'static dyn Descriptor] {
    fn by_name(&self, name: &str) -> Option<&'static dyn Descriptor> {
        self.iter().find(|&&d| d.describe().name == name).copied()
    }
}
