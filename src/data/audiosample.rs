use std::string::*;
use std::fmt;

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Soniton {
    bits: u8,
    be: bool,
    packed: bool,
    planar: bool,
    float: bool,
    signed: bool,
}

bitflags! {
    pub flags Flags: u8 {
        const BE       = 0x01,
        const PACKED   = 0x02,
        const PLAR   = 0x04,
        const FLOAT    = 0x08,
        const SIGNED   = 0x10,
    }
}

impl Soniton {
    pub fn new(bits: u8, flags: Flags) -> Self {
        let is_be = flags.contains(BE);
        let is_pk = flags.contains(PACKED);
        let is_pl = flags.contains(PLAR);
        let is_fl = flags.contains(FLOAT);
        let is_sg = flags.contains(SIGNED);

        Soniton {
            bits: bits,
            be: is_be,
            packed: is_pk,
            planar: is_pl,
            float: is_fl,
            signed: is_sg,
        }
    }

    pub fn get_bits(&self) -> u8 {
        self.bits
    }
    pub fn is_be(&self) -> bool {
        self.be
    }
    pub fn is_packed(&self) -> bool {
        self.packed
    }
    pub fn is_planar(&self) -> bool {
        self.planar
    }
    pub fn is_float(&self) -> bool {
        self.float
    }
    pub fn is_signed(&self) -> bool {
        self.signed
    }

    pub fn get_audio_size(&self, length: u64) -> usize {
        if self.packed {
            ((length * (self.bits as u64) + 7) >> 3) as usize
        } else {
            (length * (((self.bits + 7) >> 3) as u64)) as usize
        }
    }
}

impl fmt::Display for Soniton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = if self.float {
            "float"
        } else if self.signed {
            "int"
        } else {
            "uint"
        };
        let end = if self.be {
            "BE"
        } else {
            "LE"
        };
        write!(f,
               "({} bps, {} planar: {} packed: {} {})",
               self.bits,
               end,
               self.packed,
               self.planar,
               fmt)
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum ChannelType {
    C,
    L,
    R,
    Cs,
    Ls,
    Rs,
    Lss,
    Rss,
    LFE,
    Lc,
    Rc,
    Lh,
    Rh,
    Ch,
    LFE2,
    Lw,
    Rw,
    Ov,
    Lhs,
    Rhs,
    Chs,
    Ll,
    Rl,
    Cl,
    Lt,
    Rt,
    Lo,
    Ro,
}

impl ChannelType {
    pub fn is_center(&self) -> bool {
        match *self {
            ChannelType::C => true,
            ChannelType::Ch => true,
            ChannelType::Cl => true,
            ChannelType::Ov => true,
            ChannelType::LFE => true,
            ChannelType::LFE2 => true,
            ChannelType::Cs => true,
            ChannelType::Chs => true,
            _ => false,
        }
    }
    pub fn is_left(&self) -> bool {
        match *self {
            ChannelType::L => true,
            ChannelType::Ls => true,
            ChannelType::Lss => true,
            ChannelType::Lc => true,
            ChannelType::Lh => true,
            ChannelType::Lw => true,
            ChannelType::Lhs => true,
            ChannelType::Ll => true,
            ChannelType::Lt => true,
            ChannelType::Lo => true,
            _ => false,
        }
    }
    pub fn is_right(&self) -> bool {
        match *self {
            ChannelType::R => true,
            ChannelType::Rs => true,
            ChannelType::Rss => true,
            ChannelType::Rc => true,
            ChannelType::Rh => true,
            ChannelType::Rw => true,
            ChannelType::Rhs => true,
            ChannelType::Rl => true,
            ChannelType::Rt => true,
            ChannelType::Ro => true,
            _ => false,
        }
    }
}

impl fmt::Display for ChannelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            ChannelType::C => "C".to_string(),
            ChannelType::L => "L".to_string(),
            ChannelType::R => "R".to_string(),
            ChannelType::Cs => "Cs".to_string(),
            ChannelType::Ls => "Ls".to_string(),
            ChannelType::Rs => "Rs".to_string(),
            ChannelType::Lss => "Lss".to_string(),
            ChannelType::Rss => "Rss".to_string(),
            ChannelType::LFE => "LFE".to_string(),
            ChannelType::Lc => "Lc".to_string(),
            ChannelType::Rc => "Rc".to_string(),
            ChannelType::Lh => "Lh".to_string(),
            ChannelType::Rh => "Rh".to_string(),
            ChannelType::Ch => "Ch".to_string(),
            ChannelType::LFE2 => "LFE2".to_string(),
            ChannelType::Lw => "Lw".to_string(),
            ChannelType::Rw => "Rw".to_string(),
            ChannelType::Ov => "Ov".to_string(),
            ChannelType::Lhs => "Lhs".to_string(),
            ChannelType::Rhs => "Rhs".to_string(),
            ChannelType::Chs => "Chs".to_string(),
            ChannelType::Ll => "Ll".to_string(),
            ChannelType::Rl => "Rl".to_string(),
            ChannelType::Cl => "Cl".to_string(),
            ChannelType::Lt => "Lt".to_string(),
            ChannelType::Rt => "Rt".to_string(),
            ChannelType::Lo => "Lo".to_string(),
            ChannelType::Ro => "Ro".to_string(),
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone)]
pub struct ChannelMap {
    ids: Vec<ChannelType>,
}

impl ChannelMap {
    pub fn new() -> Self {
        ChannelMap { ids: Vec::new() }
    }
    pub fn add_channel(&mut self, ch: ChannelType) {
        self.ids.push(ch);
    }
    pub fn add_channels(&mut self, chs: &[ChannelType]) {
        for i in 0..chs.len() {
            self.ids.push(chs[i]);
        }
    }
    pub fn num_channels(&self) -> usize {
        self.ids.len()
    }
    pub fn get_channel(&self, idx: usize) -> ChannelType {
        self.ids[idx]
    }
    pub fn find_channel_id(&self, t: ChannelType) -> Option<u8> {
        for i in 0..self.ids.len() {
            if self.ids[i] as i32 == t as i32 {
                return Some(i as u8);
            }
        }
        None
    }
}

pub mod formats {
    use data::audiosample::*;

    pub const U8: Soniton = Soniton {
        bits: 8,
        be: false,
        packed: false,
        planar: false,
        float: false,
        signed: false,
    };
    pub const S16: Soniton = Soniton {
        bits: 16,
        be: false,
        packed: false,
        planar: false,
        float: false,
        signed: true,
    };
    pub const F32: Soniton = Soniton {
        bits: 32,
        be: false,
        packed: false,
        planar: false,
        float: true,
        signed: true,
    };
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fmt() {
        println!("{}", formats::S16);
        println!("{}", formats::U8);
        println!("{}", formats::F32);
    }
}
