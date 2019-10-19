use std::fmt;
use std::string::*;

/// Parameters describing the representation of an audio sample 
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Soniton {
    /// Number of bits in each sample.
    pub bits: u8,
    /// True if the sample data is in big endian layout, false if little endian.
    pub be: bool,
    /// For data which isn't a multiple of 8 this is true if there is no padding between samples,
    /// false if the data is padded.
    pub packed: bool,
    /// If true data is stored in a planar layout with channels occurring in sequence i.e. C1 C1
    /// C1... C2 C2 C2 . If false data is interleaved i.e. C1 C2 C1 C2.
    pub planar: bool,
    /// True if the sample data is a floating point data type, false otherwise.
    pub float: bool,
    /// True if the sample data is signed, false otherwise.
    pub signed: bool,
}

// TODO: make it a trait for usize?
/// Given a length in bytes `v` return the size when aligned to `a` bytes
fn align(v: usize, a: usize) -> usize {
    (v + a - 1) & !(a - 1)
}

/// Round the size in bits to the number of bytes required
fn round_to_byte(v: usize) -> usize {
    (v + 7) >> 3
}

impl Soniton {
    /// Creates a new representation of a sample with the provided parameters
    pub fn new(bits: u8, be: bool, packed: bool, planar: bool, float: bool, signed: bool) -> Self {
        Soniton {
            bits,
            be,
            packed,
            planar,
            float,
            signed,
        }
    }

    /// For a stream of the given length and alignment return the size of the buffer containing the
    /// data
    pub fn get_audio_size(self, length: usize, alignment: usize) -> usize {
        let s = if self.packed {
            round_to_byte(length * (self.bits as usize))
        } else {
            length * round_to_byte(self.bits as usize)
        };

        align(s, alignment)
    }
}

impl fmt::Display for Soniton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = if self.float {
            "float"
        } else if self.signed {
            "int"
        } else {
            "uint"
        };
        let end = if self.be { "BE" } else { "LE" };
        write!(
            f,
            "({} bps, {} planar: {} packed: {} {})",
            self.bits, end, self.packed, self.planar, fmt
        )
    }
}

/// Enum representing the different types of audio channels
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Returns true if the channel is centered
    pub fn is_center(self) -> bool {
        match self {
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

    /// Returns true if the channel is a left audio channel, false otherwise
    pub fn is_left(self) -> bool {
        match self {
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

    /// Returns true if the channel is a right audio channel, false otherwise
    pub fn is_right(self) -> bool {
        match self {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Stores the audio channels in the file in order of occurence
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ChannelMap {
    ids: Vec<ChannelType>,
}

impl ChannelMap {
    /// Creates a new channel map
    pub fn new() -> Self {
        ChannelMap { ids: Vec::new() }
    }

    /// Adds a single channel to the map
    pub fn add_channel(&mut self, ch: ChannelType) {
        self.ids.push(ch);
    }

    /// Adds number of channels to the map in order of occurrence
    pub fn add_channels(&mut self, chs: &[ChannelType]) {
        for ch in chs {
            self.ids.push(*ch);
        }
    }

    /// Returns the number of channels in the stream
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Gets the channel type for the channel at the given index
    pub fn get_channel(&self, idx: usize) -> ChannelType {
        self.ids[idx]
    }

    /// Finds the first channel index containing a channel of the given type
    pub fn find_channel_id(&self, t: ChannelType) -> Option<u8> {
        for i in 0..self.ids.len() {
            if self.ids[i] as i32 == t as i32 {
                return Some(i as u8);
            }
        }
        None
    }

    /// Creates a default channel map. For 1 channel this is a single centred channel for 2
    /// channels a Right channel and a Left channel. No implementation currently exists for other
    /// counts.
    pub fn default_map(count: usize) -> Self {
        use self::ChannelType::*;
        let ids = match count {
            1 => vec![C],
            2 => vec![R, L],
            _ => unimplemented!(),
        };

        ChannelMap { ids }
    }
}

/// A set of default constant channels for general use
pub mod formats {
    use super::*;

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
    pub const S32: Soniton = Soniton {
        bits: 32,
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
    pub const F64: Soniton = Soniton {
        bits: 64,
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
