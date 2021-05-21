use std::collections::HashMap;

use crate::data::frame::ArcFrame;
use crate::data::packet::Packet;

pub use crate::common::CodecList;
use crate::error::*;

/// Used to interact with a decoder.
pub trait Decoder: Send {
    // TODO support codec configuration using set_option
    // fn open(&mut self) -> Result<()>;
    /// Saves the extra data contained in a codec.
    fn set_extradata(&mut self, extra: &[u8]);
    /// Sends to the decoder a packet to be decoded.
    fn send_packet(&mut self, pkt: &Packet) -> Result<()>;
    /// Returns a decoded frame.
    fn receive_frame(&mut self) -> Result<ArcFrame>;
    /// Configures the decoder.
    fn configure(&mut self) -> Result<()>;
    /// Tells decoder to clear its internal state.
    fn flush(&mut self) -> Result<()>;
}

/// Codec descriptor.
///
/// Contains information on a codec and its own decoder.
#[derive(Debug)]
pub struct Descr {
    /// The codec name.
    pub codec: &'static str,
    /// The extended codec name.
    pub name: &'static str,
    /// The codec description.
    pub desc: &'static str,
    /// The codec MIME.
    pub mime: &'static str,
    // TODO more fields regarding capabilities
}

/// Auxiliary structure to encapsulate a decoder object and
/// its additional data.
pub struct Context {
    dec: Box<dyn Decoder>,
    // TODO: Queue up packets/frames
}

impl Context {
    // TODO: More constructors
    /// Retrieves a codec descriptor from a codec list through its name,
    /// creates the relative decoder, and encapsulates it into a new `Context`.
    pub fn by_name(codecs: &Codecs, name: &str) -> Option<Context> {
        if let Some(builder) = codecs.by_name(name) {
            let dec = builder.create();
            Some(Context { dec })
        } else {
            None
        }
    }
    /// Saves the extra data contained in a codec.
    pub fn set_extradata(&mut self, extra: &[u8]) {
        self.dec.set_extradata(extra);
    }

    /// Sends to the decoder a packet to be decoded.
    pub fn send_packet(&mut self, pkt: &Packet) -> Result<()> {
        self.dec.send_packet(pkt)
    }
    /// Returns a decoded frame.
    pub fn receive_frame(&mut self) -> Result<ArcFrame> {
        self.dec.receive_frame()
    }
    /// Configures the decoder.
    pub fn configure(&mut self) -> Result<()> {
        self.dec.configure()
    }

    /// Tells decoder to clear its internal state.
    pub fn flush(&mut self) -> Result<()> {
        self.dec.flush()
    }
}

/// Used to get the descriptor of a codec and create its own decoder.
pub trait Descriptor {
    /// Creates a new decoder for the requested codec.
    fn create(&self) -> Box<dyn Decoder>;
    /// Returns the codec descriptor.
    fn describe(&self) -> &Descr;
}

/// A list of codec descriptors.
pub struct Codecs {
    list: HashMap<&'static str, Vec<&'static dyn Descriptor>>,
}

impl CodecList for Codecs {
    type D = dyn Descriptor;

    fn new() -> Codecs {
        Codecs {
            list: HashMap::new(),
        }
    }

    // TODO more lookup functions
    fn by_name(&self, name: &str) -> Option<&'static Self::D> {
        self.list.get(name).map(|descs| descs[0])
    }

    fn append(&mut self, desc: &'static Self::D) {
        let codec_name = desc.describe().codec;

        self.list
            .entry(codec_name)
            .or_insert_with(Vec::new)
            .push(desc);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod dummy {
        use super::super::*;
        use crate::data::pixel::Formaton;
        use std::sync::Arc;

        struct Dec {
            state: usize,
            format: Option<Arc<Formaton>>,
        }

        pub struct Des {
            descr: Descr,
        }

        impl Descriptor for Des {
            fn create(&self) -> Box<dyn Decoder> {
                Box::new(Dec {
                    state: 0,
                    format: None,
                })
            }
            fn describe<'a>(&'a self) -> &'a Descr {
                &self.descr
            }
        }

        impl Decoder for Dec {
            fn configure(&mut self) -> Result<()> {
                Ok(())
            }
            fn set_extradata(&mut self, extra: &[u8]) {
                if extra.len() > 4 {
                    self.state = 42;
                } else {
                    self.state = 12;
                }
            }
            fn send_packet(&mut self, _packet: &Packet) -> Result<()> {
                self.state += 1;
                Ok(())
            }
            fn receive_frame(&mut self) -> Result<ArcFrame> {
                unimplemented!()
            }
            fn flush(&mut self) -> Result<()> {
                Ok(())
            }
        }

        pub const DUMMY_DESCR: &Des = &Des {
            descr: Descr {
                codec: "dummy",
                name: "dummy",
                desc: "Dummy decoder",
                mime: "x-application/dummy",
            },
        };
    }
    use self::dummy::DUMMY_DESCR;

    #[test]
    fn lookup() {
        let codecs = Codecs::from_list(&[DUMMY_DESCR]);

        let _dec = codecs.by_name("dummy").unwrap();
    }
}
