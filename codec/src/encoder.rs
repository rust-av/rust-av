use std::collections::HashMap;
use std::convert::Into;

use crate::data::frame::ArcFrame;
use crate::data::packet::Packet;
use crate::data::params::CodecParams;
use crate::data::value::Value;

use crate::error::*;

/// Used to interact with an encoder.
pub trait Encoder: Send {
    /// Returns the extra data added by an encoder to a codec.
    fn get_extradata(&self) -> Option<Vec<u8>>;
    /// Sends to the encoder a frame to be encoded.
    fn send_frame(&mut self, pkt: &ArcFrame) -> Result<()>;
    /// Returns an encoded packet.
    fn receive_packet(&mut self) -> Result<Packet>;
    /// Tells encoder to clear its internal state.
    fn flush(&mut self) -> Result<()>;

    /// Configures the encoder.
    fn configure(&mut self) -> Result<()>;
    /// Sets an encoder option.
    fn set_option<'a>(&mut self, key: &str, val: Value<'a>) -> Result<()>;
    // fn get_option(&mut self, key: &str) -> Option<Value>;
    //
    /// Sets the parameters associated to a determined codec.
    fn set_params(&mut self, params: &CodecParams) -> Result<()>;
    /// Gets the parameters associated to a determined codec.
    fn get_params(&self) -> Result<CodecParams>;
}

/// Auxiliary structure to encapsulate an encoder object and
/// its additional data.
pub struct Context {
    enc: Box<dyn Encoder>,
    // TODO: Queue up packets/frames
    // TODO: Store here more information
    // TODO: Have a resource pool
    // format: Format
}

impl Context {
    // TODO: More constructors
    /// Retrieves a codec descriptor from a codec list through its name,
    /// creates the relative encoder, and encapsulates it into a new `Context`.
    pub fn by_name(codecs: &Codecs, name: &str) -> Option<Context> {
        if let Some(builder) = codecs.by_name(name) {
            let enc = builder.create();
            Some(Context { enc })
        } else {
            None
        }
    }

    /// Configures the encoder.
    pub fn configure(&mut self) -> Result<()> {
        self.enc.configure()
    }

    /// Sets the parameters associated to a determined codec.
    pub fn set_params(&mut self, params: &CodecParams) -> Result<()> {
        self.enc.set_params(params)
    }

    /// Gets the parameters associated to a determined codec.
    pub fn get_params(&self) -> Result<CodecParams> {
        self.enc.get_params()
    }

    /// Sets an encoder option.
    pub fn set_option<'a, V>(&mut self, key: &str, val: V) -> Result<()>
    where
        V: Into<Value<'a>>,
    {
        // TODO: support more options
        self.enc.set_option(key, val.into())
    }

    /// Returns the extra data added by an encoder to a codec.
    pub fn get_extradata(&mut self) -> Option<Vec<u8>> {
        self.enc.get_extradata()
    }
    /// Sends to the encoder a frame to be encoded.
    pub fn send_frame(&mut self, frame: &ArcFrame) -> Result<()> {
        self.enc.send_frame(frame)
    }
    /// Returns an encoded packet.
    // TODO: Return an Event?
    pub fn receive_packet(&mut self) -> Result<Packet> {
        self.enc.receive_packet()
    }

    /// Tells encoder to clear its internal state.
    pub fn flush(&mut self) -> Result<()> {
        self.enc.flush()
    }
}

/// Codec descriptor.
///
/// Contains information on a codec and its own encoder.
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

/// Used to get the descriptor of a codec and create its own encoder.
pub trait Descriptor {
    /// Creates a new encoder for the requested codec.
    fn create(&self) -> Box<dyn Encoder>;
    /// Returns the codec descriptor.
    fn describe(&self) -> &Descr;
}

/// A list of codec descriptors.
pub struct Codecs {
    list: HashMap<&'static str, Vec<&'static dyn Descriptor>>,
}

pub use crate::common::CodecList;

impl CodecList for Codecs {
    type D = dyn Descriptor;
    fn new() -> Codecs {
        Codecs {
            list: HashMap::new(),
        }
    }
    // TODO more lookup functions
    fn by_name(&self, name: &str) -> Option<&'static dyn Descriptor> {
        self.list.get(name).map(|descs| descs[0])
    }

    fn append(&mut self, desc: &'static dyn Descriptor) {
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
        use super::super::super::error::Error;
        use super::super::*;
        use crate::data::pixel::Formaton;
        use std::sync::Arc;

        struct Enc {
            state: usize,
            w: Option<usize>,
            h: Option<usize>,
            format: Option<Arc<Formaton>>,
        }

        pub struct Des {
            descr: Descr,
        }

        impl Descriptor for Des {
            fn create(&self) -> Box<dyn Encoder> {
                Box::new(Enc {
                    state: 0,
                    w: None,
                    h: None,
                    format: None,
                })
            }
            fn describe<'a>(&'a self) -> &'a Descr {
                &self.descr
            }
        }

        impl Encoder for Enc {
            fn configure(&mut self) -> Result<()> {
                if self.h.is_some() && self.w.is_some() && self.format.is_some() {
                    Ok(())
                } else {
                    Err(Error::ConfigurationIncomplete)
                }
            }
            fn get_extradata(&self) -> Option<Vec<u8>> {
                Some(vec![self.state as u8; 1])
            }
            fn send_frame(&mut self, _frame: &ArcFrame) -> Result<()> {
                self.state += 1;
                Ok(())
            }
            fn receive_packet(&mut self) -> Result<Packet> {
                let mut p = Packet::with_capacity(1);

                p.data.push(self.state as u8);

                Ok(p)
            }
            fn set_option<'a>(&mut self, key: &str, val: Value<'a>) -> Result<()> {
                match (key, val) {
                    ("w", Value::U64(v)) => self.w = Some(v as usize),
                    ("h", Value::U64(v)) => self.h = Some(v as usize),
                    ("format", Value::Formaton(f)) => self.format = Some(f),
                    _ => return Err(Error::Unsupported(format!("{} key", key))),
                }

                Ok(())
            }

            fn set_params(&mut self, params: &CodecParams) -> Result<()> {
                use crate::data::params::*;

                if let Some(MediaKind::Video(ref info)) = params.kind {
                    self.w = Some(info.width);
                    self.h = Some(info.height);
                    self.format = info.format.clone();
                }
                Ok(())
            }

            fn get_params(&self) -> Result<CodecParams> {
                use crate::data::params::*;

                if self.w.is_none() || self.w.is_none() || self.format.is_none() {
                    return Err(Error::ConfigurationIncomplete);
                }

                Ok(CodecParams {
                    kind: Some(MediaKind::Video(VideoInfo {
                        height: self.w.unwrap(),
                        width: self.h.unwrap(),
                        format: self.format.clone(),
                    })),
                    codec_id: Some("dummy".to_owned()),
                    extradata: self.get_extradata(),
                    bit_rate: 0,
                    convergence_window: 0,
                    delay: 0,
                })
            }

            fn flush(&mut self) -> Result<()> {
                Ok(())
            }
        }

        pub const DUMMY_DESCR: &Des = &Des {
            descr: Descr {
                codec: "dummy",
                name: "dummy",
                desc: "Dummy encoder",
                mime: "x-application/dummy",
            },
        };
    }
    use self::dummy::DUMMY_DESCR;

    #[test]
    fn lookup() {
        let codecs = Codecs::from_list(&[DUMMY_DESCR]);

        let _enc = codecs.by_name("dummy");
    }
}
