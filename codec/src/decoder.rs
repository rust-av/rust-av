use std::collections::HashMap;

use crate::data::frame::ArcFrame;
use crate::data::packet::Packet;

pub use crate::common::CodecList;
use crate::error::*;

pub trait Decoder: Send {
    // TODO support codec configuration using set_option
    // fn open(&mut self) -> Result<()>;
    fn set_extradata(&mut self, extra: &[u8]);
    fn send_packet(&mut self, pkt: &Packet) -> Result<()>;
    fn receive_frame(&mut self) -> Result<ArcFrame>;
    fn configure(&mut self) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub struct Descr {
    pub codec: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub mime: &'static str,
    // TODO more fields regarding capabilities
}

pub struct Context {
    dec: Box<dyn Decoder>,
    // TODO: Queue up packets/frames
}

impl Context {
    // TODO: More constructors
    pub fn by_name(codecs: &Codecs, name: &str) -> Option<Context> {
        if let Some(builder) = codecs.by_name(name) {
            let dec = builder.create();
            Some(Context { dec: dec })
        } else {
            None
        }
    }
    pub fn set_extradata(&mut self, extra: &[u8]) {
        self.dec.set_extradata(extra);
    }
    pub fn send_packet(&mut self, pkt: &Packet) -> Result<()> {
        self.dec.send_packet(pkt)
    }
    pub fn receive_frame(&mut self) -> Result<ArcFrame> {
        self.dec.receive_frame()
    }
    pub fn configure(&mut self) -> Result<()> {
        self.dec.configure()
    }

    pub fn flush(&mut self) -> Result<()> {
        self.dec.flush()
    }
}

pub trait Descriptor {
    fn create(&self) -> Box<dyn Decoder>;
    fn describe<'a>(&'a self) -> &'a Descr;
}

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
        if let Some(descs) = self.list.get(name) {
            Some(descs[0])
        } else {
            None
        }
    }

    fn append(&mut self, desc: &'static Self::D) {
        let codec_name = desc.describe().codec;

        self.list.entry(codec_name).or_insert(Vec::new()).push(desc);
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
                desc: "Dummy encoder",
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
