use crate::common::*;
use crate::data::packet::Packet;
use crate::data::value::*;
use std::any::Any;
use std::io::Write;
use std::sync::Arc;

use crate::error::*;

/// Used to implement muxing operations.
pub trait Muxer: Send {
    /// Configures a muxer.
    fn configure(&mut self) -> Result<()>;
    /// Writes a stream header into a data structure implementing
    /// the `Write` trait.
    fn write_header(&mut self, out: &mut dyn Write) -> Result<()>;
    /// Writes a stream packet into a data structure implementing
    /// the `Write` trait.
    fn write_packet(&mut self, out: &mut dyn Write, pkt: Arc<Packet>) -> Result<()>;
    /// Writes a stream trailer into a data structure implementing
    /// the `Write` trait.
    fn write_trailer(&mut self, out: &mut dyn Write) -> Result<()>;

    /// Sets global media file information for a muxer.
    fn set_global_info(&mut self, info: GlobalInfo) -> Result<()>;
    /// Sets a muxer option.
    ///
    /// This method should be called as many times as the number of options
    /// present in a muxer.
    fn set_option<'a>(&mut self, key: &str, val: Value<'a>) -> Result<()>;
}

/// Auxiliary structure to encapsulate a muxer object and
/// its additional data.
pub struct Context {
    muxer: Box<dyn Muxer + Send>,
    writer: Box<dyn Write + Send>,
    buf: Vec<u8>,
    /// User private data.
    ///
    /// This data cannot be cloned.
    pub user_private: Option<Box<dyn Any + Send + Sync>>,
}

impl Context {
    /// Creates a new `Context` instance.
    pub fn new<W: Write + 'static + Send>(muxer: Box<dyn Muxer + Send>, writer: Box<W>) -> Self {
        Context {
            muxer,
            writer,
            buf: Vec::new(),
            user_private: None,
        }
    }

    /// Configures a muxer.
    pub fn configure(&mut self) -> Result<()> {
        self.muxer.configure()
    }

    /// Writes a stream header to an internal buffer and returns how many
    /// bytes were written or an error.
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

    /// Writes a stream packet to an internal buffer and returns how many
    /// bytes were written or an error.
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

    /// Writes a stream trailer to an internal buffer and returns how many
    /// bytes were written or an error.
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

    /// Sets global media file information for a muxer.
    pub fn set_global_info(&mut self, info: GlobalInfo) -> Result<()> {
        self.muxer.set_global_info(info)
    }

    /// Sets a muxer option.
    ///
    /// This method should be called as many times as the number of options
    /// present in a muxer.
    pub fn set_option<'a, V>(&mut self, key: &str, val: V) -> Result<()>
    where
        V: Into<Value<'a>>,
    {
        self.muxer.set_option(key, val.into())
    }
}

/// Format descriptor.
///
/// Contains information on a format and its own muxer.
#[derive(Clone, Debug, PartialEq)]
pub struct Descr {
    /// Format name.
    pub name: &'static str,
    /// Muxer name.
    pub demuxer: &'static str,
    /// Format description.
    pub description: &'static str,
    /// Format media file extensions.
    pub extensions: &'static [&'static str],
    /// Format MIME.
    pub mime: &'static [&'static str],
}

/// Used to get a format descriptor and create a new muxer.
pub trait Descriptor {
    /// Creates a new muxer for the requested format.
    fn create(&self) -> Box<dyn Muxer>;
    /// Returns the descriptor of a format.
    fn describe(&self) -> &Descr;
}

/// Used to look for a specific format.
pub trait Lookup {
    /// Retrieves a specific format by name.
    fn by_name(&self, name: &str) -> Option<&'static dyn Descriptor>;
}

impl Lookup for [&'static dyn Descriptor] {
    fn by_name(&self, name: &str) -> Option<&'static dyn Descriptor> {
        self.iter().find(|&&d| d.describe().name == name).copied()
    }
}
