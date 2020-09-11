use std::fmt;
use std::string::*;

/// Audio format definition.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Soniton {
    /// Bits per sample.
    pub bits: u8,
    /// Tells if audio format is big-endian.
    pub be: bool,
    /// Audio samples are packed (e.g. 20-bit audio samples) and not padded.
    pub packed: bool,
    /// Audio data is stored in planar format
    /// (channels in sequence i.e. C1 C1 C1... C2 C2 C2) instead of interleaving
    /// samples (i.e. C1 C2 C1 C2) for different channels.
    pub planar: bool,
    /// Audio data is in floating point format.
    pub float: bool,
    /// Audio data is signed (usually only 8-bit audio is unsigned).
    pub signed: bool,
}

// TODO: make it a trait for usize?
/// Alignes a value to a specific number of bytes.
fn align(v: usize, a: usize) -> usize {
    (v + a - 1) & !(a - 1)
}

/// Returns the number of bytes necessary to represent the number of bits
/// passed as input.
fn round_to_byte(v: usize) -> usize {
    (v + 7) >> 3
}

impl Soniton {
    /// Constructs a new audio format definition.
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

    /// Returns the amount of bytes needed to store
    /// the audio of requested length (in samples).
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

/// Known audio channel types.
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
    /// Tells whether the channel is some center channel.
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

    /// Tells whether the channel is some left channel.
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

    /// Tells whether the channel is some right channel.
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

/// An ordered sequence of channels.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ChannelMap {
    ids: Vec<ChannelType>,
}

impl ChannelMap {
    /// Creates a new sequence of channels.
    pub fn new() -> Self {
        ChannelMap { ids: Vec::new() }
    }

    /// Adds a single channel to the map.
    pub fn add_channel(&mut self, ch: ChannelType) {
        self.ids.push(ch);
    }

    /// Adds several channels to the map in order of occurrence.
    pub fn add_channels(&mut self, chs: &[ChannelType]) {
        for ch in chs {
            self.ids.push(*ch);
        }
    }

    /// Returns the total number of channels of the map.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Tells if the channel map is empty.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Gets the channel type for a requested index.
    pub fn get_channel(&self, idx: usize) -> ChannelType {
        self.ids[idx]
    }

    /// Tries to find the position of a determined type of channel in the map.
    pub fn find_channel_id(&self, t: ChannelType) -> Option<u8> {
        for i in 0..self.ids.len() {
            if self.ids[i] as i32 == t as i32 {
                return Some(i as u8);
            }
        }
        None
    }

    /// Creates a default channel map.
    ///
    /// Depending on the `count` value, the channel map is defined differently.
    ///
    /// When `count` is 1 --> the channel map is composed by a single centered
    /// channel.
    ///
    /// When `count` is 2 --> the channel map is composed by a right and a left
    /// channel respectively.
    ///
    /// For other `count` values, no other implementations are given for now.
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

/// A set of default constant channels for general use.
pub mod formats {
    use super::*;

    /// Predefined format for interleaved 8-bit unsigned audio.
    pub const U8: Soniton = Soniton {
        bits: 8,
        be: false,
        packed: false,
        planar: false,
        float: false,
        signed: false,
    };

    /// Predefined format for interleaved 16-bit signed audio.
    pub const S16: Soniton = Soniton {
        bits: 16,
        be: false,
        packed: false,
        planar: false,
        float: false,
        signed: true,
    };

    /// Predefined format for interleaved 32-bit signed audio.
    pub const S32: Soniton = Soniton {
        bits: 32,
        be: false,
        packed: false,
        planar: false,
        float: false,
        signed: true,
    };

    /// Predefined format for interleaved floating points 32-bit signed audio.
    pub const F32: Soniton = Soniton {
        bits: 32,
        be: false,
        packed: false,
        planar: false,
        float: true,
        signed: true,
    };

    /// Predefined format for interleaved floating points 64-bit signed audio.
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
