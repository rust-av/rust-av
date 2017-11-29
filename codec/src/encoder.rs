use std::collections::HashMap;
use std::convert::Into;

use data::packet::Packet;
use data::frame::ArcFrame;
use data::value::Value;

use error::*;

pub trait Encoder : Send {
    fn get_extradata(&self) -> Option<Vec<u8>>;
    fn send_frame(&mut self, pkt: &ArcFrame) -> Result<()>;
    fn receive_packet(&mut self) -> Result<Packet>;
    fn flush(&mut self) -> Result<()>;

    fn configure(&mut self) -> Result<()>;
    fn set_option<'a>(&mut self, key: &str, val: Value<'a>) -> Result<()>;
    // fn get_option(&mut self, key: &str) -> Option<Value>;
}

pub struct Context {
    enc: Box<Encoder>,
    // TODO: Queue up packets/frames
    // TODO: Store here more information
    // TODO: Have a resource pool
    // format: Format
}

impl Context {
    // TODO: More constructors
    pub fn by_name(codecs: &Codecs, name: &str) -> Option<Context> {
        if let Some(builder) = codecs.by_name(name) {
            let enc = builder.create();
            Some(Context { enc: enc })
        } else {
            None
        }
    }

    pub fn configure(&mut self) -> Result<()> {
        self.enc.configure()
    }

    pub fn set_option<'a, V>(&mut self, key: &str, val: V) -> Result<()>
        where V: Into<Value<'a>>
    {
        // TODO: support more options
        self.enc.set_option(key, val.into())
    }

    pub fn get_extradata(&mut self) -> Option<Vec<u8>> {
        self.enc.get_extradata()
    }
    pub fn send_frame(&mut self, frame: &ArcFrame) -> Result<()> {
        self.enc.send_frame(frame)
    }
    // TODO: Return an Event?
    pub fn receive_packet(&mut self) -> Result<Packet> {
        self.enc.receive_packet()
    }

    pub fn flush(&mut self) -> Result<()> {
        self.enc.flush()
    }
}

#[derive(Debug)]
pub struct Descr {
    pub codec: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub mime: &'static str,
    // TODO more fields regarding capabilities
}

pub trait Descriptor {
    fn create(&self) -> Box<Encoder>;
    fn describe<'a>(&'a self) -> &'a Descr;
}

pub struct Codecs {
    list: HashMap<&'static str, Vec<&'static Descriptor>>
}

pub use common::CodecList;

impl CodecList for Codecs {
    type D = Descriptor;
    fn new() -> Codecs {
        Codecs { list: HashMap::new() }
    }
    // TODO more lookup functions
    fn by_name(&self, name: &str) -> Option<&'static Descriptor> {
        if let Some(descs) = self.list.get(name) {
            Some(descs[0])
        } else {
            None
        }
    }

    fn append(&mut self, desc: &'static Descriptor) {
        let codec_name = desc.describe().codec;

        self.list.entry(codec_name).or_insert(Vec::new()).push(desc);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod dummy {
        use super::super::*;
        use std::rc::Rc;
        use data::pixel::Formaton;
        use super::super::super::error::ErrorKind;

        struct Enc {
            state: usize,
            w: Option<usize>,
            h: Option<usize>,
            format: Option<Rc<Formaton>>
        }

        pub struct Des {
            descr: Descr,
        }

        impl Descriptor for Des {
            fn create(&self) -> Box<Encoder> {
                box Enc { state: 0, w: None, h: None, format: None }
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
                    Err(ErrorKind::ConfigurationIncomplete.into())
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
                    _ => return Err(ErrorKind::Unsupported.into())
                }

                Ok(())
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
            }
        };
    }
    use self::dummy::DUMMY_DESCR;

    #[test]
    fn lookup() {
        let codecs = Codecs::from_list(&[DUMMY_DESCR]);

        let _enc = codecs.by_name("dummy");
    }
}
