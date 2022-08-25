use crate::common::*;
use crate::data::packet::Packet;
use crate::data::value::*;
use std::any::Any;
use std::io::{Cursor, ErrorKind, Seek, SeekFrom, Write};
use std::sync::Arc;

use crate::error::*;

/// Runtime wrapper around either a [`Write`] or a [`WriteSeek`] trait object
/// which supports querying for seek support.
pub enum Writer<WO = Cursor<Vec<u8>>, WS = Cursor<Vec<u8>>> {
    NonSeekable(WO, u64),
    Seekable(WS),
}

impl<WO: Write> Writer<WO, Cursor<Vec<u8>>> {
    /// Creates a [`Writer`] from an object that implements the [`Write`] trait.
    pub fn from_nonseekable(inner: WO) -> Self {
        Self::NonSeekable(inner, 0)
    }
}

impl<WS: Write + Seek> Writer<Cursor<Vec<u8>>, WS> {
    /// Creates a [`Writer`] from an object that implements both
    /// [`Write`] and [`Seek`] traits.
    pub fn from_seekable(inner: WS) -> Self {
        Self::Seekable(inner)
    }
}

impl<WO: Write, WS: Write + Seek> Writer<WO, WS> {
    /// Returns whether the [`Writer`] can seek within the source.
    pub fn is_seekable(&self) -> bool {
        matches!(self, Self::Seekable(_))
    }

    /// Returns stream position.
    pub fn position(&mut self) -> Result<u64> {
        match self {
            Self::NonSeekable(_, index) => Ok(*index),
            Self::Seekable(ref mut inner) => inner.stream_position().map_err(|e| e.into()),
        }
    }

    /// Returns a reference to the non-seekable object whether is present.
    pub fn non_seekable_object(&self) -> Option<(&WO, u64)> {
        if let Self::NonSeekable(inner, index) = self {
            Some((inner, *index))
        } else {
            None
        }
    }

    /// Returns a reference to the seekable object whether is present.
    pub fn seekable_object(&self) -> Option<&WS> {
        if let Self::Seekable(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl<WO: Write, WS: Write + Seek> Write for Writer<WO, WS> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::NonSeekable(inner, ref mut index) => {
                let result = inner.write(bytes);

                if let Ok(written) = result {
                    *index += written as u64;
                }

                result
            }
            Self::Seekable(inner) => inner.write(bytes),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::NonSeekable(inner, ..) => inner.flush(),
            Self::Seekable(inner) => inner.flush(),
        }
    }
}

impl<WO: Write, WS: Write + Seek> Seek for Writer<WO, WS> {
    fn seek(&mut self, seek: SeekFrom) -> std::io::Result<u64> {
        match self {
            Self::NonSeekable(_, index) => {
                if let SeekFrom::Current(0) = seek {
                    Ok(*index)
                } else {
                    Err(std::io::Error::new(
                        ErrorKind::Other,
                        "Seeking not supported",
                    ))
                }
            }
            Self::Seekable(inner) => inner.seek(seek),
        }
    }
}

/// Used to implement muxing operations.
pub trait Muxer: Send {
    /// Configures a muxer.
    fn configure(&mut self) -> Result<()>;
    /// Writes a stream header into a data structure implementing
    /// the `Write` trait.
    fn write_header<WO: Write, WS: Write + Seek>(&mut self, out: &mut Writer<WO, WS>)
        -> Result<()>;
    /// Writes a stream packet into a data structure implementing
    /// the `Write` trait.
    fn write_packet<WO: Write, WS: Write + Seek>(
        &mut self,
        out: &mut Writer<WO, WS>,
        pkt: Arc<Packet>,
    ) -> Result<()>;
    /// Writes a stream trailer into a data structure implementing
    /// the `Write` trait.
    fn write_trailer<WO: Write, WS: Write + Seek>(
        &mut self,
        out: &mut Writer<WO, WS>,
    ) -> Result<()>;

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
pub struct Context<M: Muxer + Send, WO: Write, WS: Write + Seek> {
    muxer: M,
    writer: Writer<WO, WS>,
    /// User private data.
    ///
    /// This data cannot be cloned.
    pub user_private: Option<Box<dyn Any + Send + Sync>>,
}

impl<M: Muxer, WO: Write, WS: Write + Seek> Context<M, WO, WS> {
    /// Creates a new `Context` instance.
    pub fn new(muxer: M, writer: Writer<WO, WS>) -> Self {
        Context {
            muxer,
            writer,
            user_private: None,
        }
    }

    /// Configures a muxer.
    pub fn configure(&mut self) -> Result<()> {
        self.muxer.configure()
    }

    /// Writes a stream header to an internal buffer and returns how many
    /// bytes were written or an error.
    pub fn write_header(&mut self) -> Result<()> {
        self.muxer.write_header(&mut self.writer)
    }

    /// Writes a stream packet to an internal buffer and returns how many
    /// bytes were written or an error.
    pub fn write_packet(&mut self, pkt: Arc<Packet>) -> Result<()> {
        self.muxer.write_packet(&mut self.writer, pkt)
    }

    /// Writes a stream trailer to an internal buffer and returns how many
    /// bytes were written or an error.
    pub fn write_trailer(&mut self) -> Result<()> {
        self.muxer.write_trailer(&mut self.writer)?;
        self.writer.flush()?;

        Ok(())
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

    /// Returns the underlying writer.
    pub fn writer(&self) -> &Writer<WO, WS> {
        &self.writer
    }
}

/// Format descriptor.
///
/// Contains information on a format and its own muxer.
#[derive(Clone, Debug, PartialEq, Eq)]
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
    type OutputMuxer: Muxer + Send;

    /// Creates a new muxer for the requested format.
    fn create(&self) -> Self::OutputMuxer;
    /// Returns the descriptor of a format.
    fn describe(&self) -> &Descr;
}

/// Used to look for a specific format.
pub trait Lookup<T: Descriptor + ?Sized> {
    /// Retrieves a specific format by name.
    fn by_name(&self, name: &str) -> Option<&'static T>;
}

impl<T: Descriptor + ?Sized> Lookup<T> for [&'static T] {
    fn by_name(&self, name: &str) -> Option<&'static T> {
        self.iter().find(|&&d| d.describe().name == name).copied()
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    const DUMMY_HEADER_LENGTH: usize = 12;
    const DUMMY_PACKET_LENGTH: usize = 2;
    const DUMMY_PACKETS_NUMBER: usize = 2;
    const DUMMY_TRAILER_LENGTH: usize = 13;

    struct DummyDes {
        d: Descr,
    }

    struct DummyMuxer {}

    impl DummyMuxer {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Muxer for DummyMuxer {
        fn configure(&mut self) -> Result<()> {
            Ok(())
        }

        fn write_header<WO: Write, WS: Write + Seek>(
            &mut self,
            out: &mut Writer<WO, WS>,
        ) -> Result<()> {
            let buf = b"Dummy header";
            out.write_all(buf.as_slice()).unwrap();
            Ok(())
        }

        fn write_packet<WO: Write, WS: Write + Seek>(
            &mut self,
            out: &mut Writer<WO, WS>,
            pkt: Arc<Packet>,
        ) -> Result<()> {
            out.write_all(&pkt.data).unwrap();
            Ok(())
        }

        fn write_trailer<WO: Write, WS: Write + Seek>(
            &mut self,
            out: &mut Writer<WO, WS>,
        ) -> Result<()> {
            let buf = b"Dummy trailer";
            out.write_all(buf.as_slice()).unwrap();
            Ok(())
        }

        fn set_global_info(&mut self, _info: GlobalInfo) -> Result<()> {
            Ok(())
        }

        fn set_option<'a>(&mut self, _key: &str, _val: Value<'a>) -> Result<()> {
            Ok(())
        }
    }

    impl Descriptor for DummyDes {
        type OutputMuxer = DummyMuxer;

        fn create(&self) -> Self::OutputMuxer {
            DummyMuxer {}
        }
        fn describe(&self) -> &Descr {
            &self.d
        }
    }

    const DUMMY_DES: &dyn Descriptor<OutputMuxer = DummyMuxer> = &DummyDes {
        d: Descr {
            name: "dummy",
            demuxer: "dummy",
            description: "Dummy mux",
            extensions: &["mx", "mux"],
            mime: &["application/dummy"],
        },
    };

    #[test]
    fn lookup() {
        let muxers: &[&dyn Descriptor<OutputMuxer = DummyMuxer>] = &[DUMMY_DES];

        muxers.by_name("dummy").unwrap();
    }

    fn run_muxer<WO: Write, WS: Write + Seek>(
        writer: Writer<WO, WS>,
    ) -> Context<DummyMuxer, WO, WS> {
        let mux = DummyMuxer::new();

        let mut muxer = Context::new(mux, writer);

        muxer.configure().unwrap();
        muxer.write_header().unwrap();

        // Write zeroed packets of a certain size
        for _ in 0..DUMMY_PACKETS_NUMBER {
            let packet = Packet::zeroed(DUMMY_PACKET_LENGTH);
            muxer.write_packet(Arc::new(packet)).unwrap();
        }

        muxer.write_trailer().unwrap();
        muxer
    }

    fn check_underlying_buffer(buffer: &[u8]) {
        assert_eq!(
            buffer.get(..DUMMY_HEADER_LENGTH).unwrap(),
            b"Dummy header".as_slice()
        );

        assert_eq!(
            buffer
                // Get only packets, without header and trailer data
                .get(
                    DUMMY_HEADER_LENGTH
                        ..DUMMY_HEADER_LENGTH + (DUMMY_PACKETS_NUMBER * DUMMY_PACKET_LENGTH)
                )
                .unwrap(),
            &[0, 0, 0, 0]
        );

        assert_eq!(
            buffer
                // Skip header and packets
                .get(DUMMY_HEADER_LENGTH + (DUMMY_PACKETS_NUMBER * DUMMY_PACKET_LENGTH)..)
                .unwrap(),
            b"Dummy trailer".as_slice()
        );
    }

    #[test]
    fn non_seekable_muxer() {
        let muxer = run_muxer(Writer::from_nonseekable(Vec::new()));
        let (buffer, index) = muxer.writer().non_seekable_object().unwrap();
        debug_assert!(!muxer.writer().is_seekable());
        check_underlying_buffer(buffer);
        assert_eq!(
            index as usize,
            DUMMY_HEADER_LENGTH
                + (DUMMY_PACKETS_NUMBER * DUMMY_PACKET_LENGTH)
                + DUMMY_TRAILER_LENGTH
        );
    }

    #[test]
    fn seekable_muxer() {
        let muxer = run_muxer(Writer::from_seekable(Cursor::new(Vec::new())));
        let buffer = muxer.writer().seekable_object().unwrap().get_ref();
        debug_assert!(muxer.writer().is_seekable());
        check_underlying_buffer(buffer);
    }
}
