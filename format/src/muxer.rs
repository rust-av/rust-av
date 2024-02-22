use crate::common::*;
use crate::data::packet::Packet;
use crate::data::value::*;
use std::any::Any;
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::sync::Arc;

use crate::error::*;

/// Runtime wrapper around a [`Write`] trait object
/// which optionally supports [`Seek`] functionality.
pub struct Writer<W = Cursor<Vec<u8>>> {
    writer: W,
    bytes_written: usize,
}

impl<W: Write> Writer<W> {
    /// Creates a [`Writer`] from an object that implements the [`Write`] trait.
    pub fn new(inner: W) -> Self {
        Self {
            writer: inner,
            bytes_written: 0,
        }
    }
}

impl<W: Write> Writer<W> {
    /// Returns stream position.
    pub fn position(&mut self) -> usize {
        self.bytes_written
    }

    /// Returns a reference to the underlying writer and bytes written.
    pub fn as_ref(&self) -> (&W, usize) {
        (&self.writer, self.bytes_written)
    }
}

impl<W: Write> Write for Writer<W> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let result = self.writer.write(bytes);

        if let Ok(written) = result {
            self.bytes_written += written;
        }

        result
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> Seek for Writer<W>
where
    W: Seek,
{
    fn seek(&mut self, seek: SeekFrom) -> std::io::Result<u64> {
        let res = self.writer.seek(seek)?;
        self.bytes_written = res as usize;
        Ok(res)
    }
}

/// Used to implement muxing operations.
pub trait Muxer: Send {
    /// Configures a muxer.
    fn configure(&mut self) -> Result<()>;
    /// Writes a stream header into a data structure implementing
    /// the `Write` trait.
    fn write_header<W: Write>(&mut self, out: &mut Writer<W>) -> Result<()>;
    /// Writes a stream packet into a data structure implementing
    /// the `Write` trait.
    fn write_packet<W: Write>(&mut self, out: &mut Writer<W>, pkt: Arc<Packet>) -> Result<()>;
    /// Writes a stream trailer into a data structure implementing
    /// the `Write` trait.
    fn write_trailer<W: Write>(&mut self, out: &mut Writer<W>) -> Result<()>;

    /// Sets global media file information for a muxer.
    fn set_global_info(&mut self, info: GlobalInfo) -> Result<()>;
    /// Sets a muxer option.
    ///
    /// This method should be called as many times as the number of options
    /// present in a muxer.
    fn set_option(&mut self, key: &str, val: Value) -> Result<()>;
}

/// Auxiliary structure to encapsulate a muxer object and
/// its additional data.
pub struct Context<M: Muxer + Send, W: Write> {
    muxer: M,
    writer: Writer<W>,
    /// User private data.
    ///
    /// This data cannot be cloned.
    pub user_private: Option<Box<dyn Any + Send + Sync>>,
}

impl<M: Muxer, W: Write> Context<M, W> {
    /// Creates a new `Context` instance.
    pub fn new(muxer: M, writer: Writer<W>) -> Self {
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
    pub fn writer(&self) -> &Writer<W> {
        &self.writer
    }

    /// Consumes this muxer and returns the underlying writer.
    pub fn into_writer(self) -> Writer<W> {
        self.writer
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
    /// The specific type of the muxer.
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

        fn write_header<W: Write>(&mut self, out: &mut Writer<W>) -> Result<()> {
            let buf = b"Dummy header";
            out.write_all(buf.as_slice()).unwrap();
            Ok(())
        }

        fn write_packet<W: Write>(&mut self, out: &mut Writer<W>, pkt: Arc<Packet>) -> Result<()> {
            out.write_all(&pkt.data).unwrap();
            Ok(())
        }

        fn write_trailer<W: Write>(&mut self, out: &mut Writer<W>) -> Result<()> {
            let buf = b"Dummy trailer";
            out.write_all(buf.as_slice()).unwrap();
            Ok(())
        }

        fn set_global_info(&mut self, _info: GlobalInfo) -> Result<()> {
            Ok(())
        }

        fn set_option(&mut self, _key: &str, _val: Value) -> Result<()> {
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

    fn run_muxer<W: Write>(writer: Writer<W>) -> Context<DummyMuxer, W> {
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
    fn vec_muxer() {
        let muxer = run_muxer(Writer::new(Vec::new()));
        let (buffer, index) = muxer.writer().as_ref();
        check_underlying_buffer(buffer);
        assert_eq!(
            index,
            DUMMY_HEADER_LENGTH
                + (DUMMY_PACKETS_NUMBER * DUMMY_PACKET_LENGTH)
                + DUMMY_TRAILER_LENGTH
        );
    }

    #[test]
    fn stdout_muxer() {
        use std::io::stdout;

        let muxer = run_muxer(Writer::new(stdout()));
        let (_buffer, index) = muxer.writer().as_ref();
        assert_eq!(
            index,
            DUMMY_HEADER_LENGTH
                + (DUMMY_PACKETS_NUMBER * DUMMY_PACKET_LENGTH)
                + DUMMY_TRAILER_LENGTH
        );
    }

    #[cfg(not(target_arch = "wasm32"))] // Files depend on host, so this test
    // cannot be run for WebAssembly
    #[test]
    fn file_muxer() {
        let file = tempfile::tempfile().unwrap();
        let muxer = run_muxer(Writer::new(file));
        let mut writer = muxer.into_writer();
        writer.seek(SeekFrom::Start(3)).unwrap();
        assert!(writer.bytes_written == 3);
        assert!(writer.as_ref().0.metadata().unwrap().len() != 0);
    }
}
