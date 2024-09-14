//! Expose all necessary data structures to represent pixels.
//!
//! Re-exports num_traits::FromPrimitive and num_traits::cast::ToPrimitive
//! in order to make easy to cast a parsed value into correct enum structures.

use num_derive::{FromPrimitive, ToPrimitive};
pub use num_traits::cast::ToPrimitive;
pub use num_traits::FromPrimitive;
use std::fmt;
use std::ops::Index;
use std::slice;

/// YUV color range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum YUVRange {
    /// Pixels in the range [16, 235].
    Limited,
    /// Pixels in the range [0, 255].
    Full,
}

impl fmt::Display for YUVRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            YUVRange::Limited => write!(f, "Limited range"),
            YUVRange::Full => write!(f, "Full range"),
        }
    }
}

/// Describes the matrix coefficients used in deriving
/// luma and chroma signals from the green, blue and red or X, Y and Z primaries.
///
/// Values adopted from Table 4 of ISO/IEC 23001-8:2013/DCOR1.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum MatrixCoefficients {
    /// The identity matrix.
    /// Typically used for:
    ///
    /// - GBR (often referred to as RGB)
    /// - YZX (often referred to as XYZ)
    /// - IEC 61966-2-1 sRGB
    /// - SMPTE ST 428-1 (2019)
    Identity = 0,
    /// - Rec. ITU-R BT.709-6
    /// - Rec. ITU-R BT.1361-0 conventional colour gamut system and extended colour
    ///   gamut system (historical)
    /// - IEC 61966-2-4 xvYCC709
    /// - SMPTE RP 177 (1993) Annex B
    BT709 = 1,
    /// Image characteristics are unknown or are determined by the application.
    Unspecified = 2,
    /// For future use by ITU-T | ISO/IEC.
    Reserved = 3,
    /// United States Federal Communications Commission (2003) Title 47 Code of
    /// Federal Regulations 73.682 (a) (20)
    BT470M = 4,
    /// - Rec. ITU-R BT.470-6 System B, G (historical)
    /// - Rec. ITU-R BT.601-7 625
    /// - Rec. ITU-R BT.1358-0 625 (historical)
    /// - Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM
    /// - IEC 61966-2-1 sYCC
    /// - IEC 61966-2-4 xvYCC601
    ///
    /// (functionally the same as the value 6)
    BT470BG = 5,
    /// - Rec. ITU-R BT.601-7 525
    /// - Rec. ITU-R BT.1358-1 525 or 625 (historical)
    /// - Rec. ITU-R BT.1700-0 NTSC
    /// - SMPTE ST 170 (2004)
    ///
    /// (functionally the same as the value 5)
    ST170M = 6,
    /// SMPTE ST 240 (1999)
    ST240M = 7,
    /// The YCoCg color model, also known as the YCgCo color model,
    /// is the color space formed from a simple transformation of
    /// an associated RGB color space into a luma value and
    /// two chroma values called chrominance green and chrominance orange.
    YCgCo = 8,
    /// - Rec. ITU-R BT.2020-2 (non-constant luminance)
    /// - Rec. ITU-R BT.2100-2 Y′CbCr
    BT2020NonConstantLuminance = 9,
    /// Rec. ITU-R BT.2020-2 (constant luminance)
    BT2020ConstantLuminance = 10,
    /// SMPTE ST 2085 (2015)
    ST2085 = 11,
    /// Chromaticity-derived non-constant luminance system.
    ChromaticityDerivedNonConstantLuminance = 12,
    /// Chromaticity-derived constant luminance system.
    ChromaticityDerivedConstantLuminance = 13,
    /// Rec. ITU-R BT.2100-2 ICTCP
    ICtCp = 14,
}

impl fmt::Display for MatrixCoefficients {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MatrixCoefficients::Identity => write!(f, "Identity"),
            MatrixCoefficients::BT709 => write!(f, "ITU BT.709"),
            MatrixCoefficients::Unspecified => write!(f, "Unspecified"),
            MatrixCoefficients::Reserved => write!(f, "Reserved"),
            MatrixCoefficients::BT470M => write!(f, "ITU BT.470M"),
            MatrixCoefficients::BT470BG => write!(f, "ITU BT.470BG"),
            MatrixCoefficients::ST170M => write!(f, "SMPTE ST-170M"),
            MatrixCoefficients::ST240M => write!(f, "SMPTE ST-240M"),
            MatrixCoefficients::YCgCo => write!(f, "YCgCo"),
            MatrixCoefficients::BT2020NonConstantLuminance => {
                write!(f, "ITU BT.2020 (Non Constant Luminance)")
            }
            MatrixCoefficients::BT2020ConstantLuminance => {
                write!(f, "ITU BT.2020 (Constant Luminance)")
            }
            MatrixCoefficients::ST2085 => write!(f, "SMPTE ST-2085"),
            MatrixCoefficients::ChromaticityDerivedNonConstantLuminance => {
                write!(f, "Chromaticity Derived (Non ConstantLuminance)")
            }
            MatrixCoefficients::ChromaticityDerivedConstantLuminance => {
                write!(f, "Chromaticity Derived (Constant Luminance)")
            }
            MatrixCoefficients::ICtCp => write!(f, "ICtCp"),
        }
    }
}

/// Indicates the chromaticity coordinates of the source colour primaries as specified in Table 2 in terms
/// of the CIE 1931 definition of x and y as specified by ISO 11664-1.
///
/// Values adopted from Table 4 of ISO/IEC 23001-8:2013/DCOR1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[allow(clippy::upper_case_acronyms)]
pub enum ColorPrimaries {
    /// For future use by ITU-T | ISO/IEC.
    Reserved0 = 0,
    /// - Rec. ITU-R BT.709-6
    /// - Rec. ITU-R BT.1361-0 conventional colour gamut
    ///   system and extended colour gamut system (historical)
    /// - IEC 61966-2-1 sRGB or sYCC
    /// - IEC 61966-2-4
    /// - Society of Motion Picture and Television Engineers
    ///   (SMPTE) RP 177 (1993) Annex B
    BT709 = 1,
    /// Image characteristics are unknown or are determined by
    /// the application.
    Unspecified = 2,
    /// For future use by ITU-T | ISO/IEC.
    Reserved = 3,
    /// - Rec. ITU-R BT.470-6 System M (historical)
    /// - United States National Television System Committee
    ///   1953 Recommendation for transmission standards for
    ///   color television
    /// - United States Federal Communications Commission
    ///   (2003) Title 47 Code of Federal Regulations 73.682 (a) (20)
    BT470M = 4,
    /// - Rec. ITU-R BT.470-6 System B, G (historical)
    /// - Rec. ITU-R BT.601-7 625
    /// - Rec. ITU-R BT.1358-0 625 (historical)
    /// - Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM
    BT470BG = 5,
    /// - Rec. ITU-R BT.601-7 525
    /// - Rec. ITU-R BT.1358-1 525 or 625 (historical)
    /// - Rec. ITU-R BT.1700-0 NTSC
    /// - SMPTE ST 170 (2004)
    ///
    /// (functionally the same as the value 7)
    ST170M = 6,
    /// - SMPTE ST 240 (1999)
    ///
    /// (functionally the same as the value 6)
    ST240M = 7,
    /// Generic film (colour filters using Illuminant C)
    Film = 8,
    /// - Rec. ITU-R BT.2020-2
    /// - Rec. ITU-R BT.2100-2
    BT2020 = 9,
    /// - SMPTE ST 428-1 (2019)
    /// - (CIE 1931 XYZ as in ISO 11664-1)
    ST428 = 10,
    /// SMPTE RP 431-2 (2011)
    P3DCI = 11,
    /// SMPTE EG 432-1 (2010)
    P3Display = 12,
    /// No corresponding industry specification identified.
    Tech3213 = 22,
}

impl fmt::Display for ColorPrimaries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ColorPrimaries::Reserved0 => write!(f, "Identity"),
            ColorPrimaries::BT709 => write!(f, "ITU BT.709"),
            ColorPrimaries::Unspecified => write!(f, "Unspecified"),
            ColorPrimaries::Reserved => write!(f, "Reserved"),
            ColorPrimaries::BT470M => write!(f, "ITU BT.470M"),
            ColorPrimaries::BT470BG => write!(f, "ITU BT.470BG"),
            ColorPrimaries::ST170M => write!(f, "SMPTE ST-170M"),
            ColorPrimaries::ST240M => write!(f, "SMPTE ST-240M"),
            ColorPrimaries::Film => write!(f, "Film"),
            ColorPrimaries::BT2020 => write!(f, "ITU BT.2020"),
            ColorPrimaries::ST428 => write!(f, "SMPTE ST-428"),
            ColorPrimaries::P3DCI => write!(f, "DCI P3"),
            ColorPrimaries::P3Display => write!(f, "Display P3"),
            ColorPrimaries::Tech3213 => write!(f, "EBU Tech3213"),
        }
    }
}

/// Either indicates the reference opto-electronic transfer characteristic
/// function of the source picture as a function of a source input linear optical intensity
/// input Lc with a nominal real-valued range of 0 to 1 or indicates the inverse of the
/// reference electro-optical transfer characteristic function as a function of an
/// output linear optical intensity Lo with a nominal real-valued range of 0 to 1.
///
/// Values adopted from Table 4 of ISO/IEC 23001-8:2013/DCOR1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[allow(clippy::upper_case_acronyms)]
pub enum TransferCharacteristic {
    /// For future use by ITU-T | ISO/IEC.
    Reserved0 = 0,
    /// - Rec. ITU-R BT.709-6
    /// - Rec. ITU-R BT.1361-0 conventional
    ///   colour gamut system (historical)
    ///
    /// (functionally the same as the values 6, 14 and 15)
    BT1886 = 1,
    /// Image characteristics are unknown or
    /// are determined by the application.
    Unspecified = 2,
    /// For future use by ITU-T | ISO/IEC.
    Reserved = 3,
    /// Assumed display gamma 2.2.
    ///
    /// - Rec. ITU-R BT.470-6 System M
    ///   (historical)
    /// - United States National Television
    ///   System Committee 1953
    ///   Recommendation for transmission
    ///   standards for color television
    /// - United States Federal Communications
    ///   Commission (2003) Title 47 Code of
    ///   Federal Regulations 73.682 (a) (20)
    /// - Rec. ITU-R BT.1700-0 625 PAL and
    ///   625 SECAM
    BT470M = 4,
    /// Assumed display gamma 2.8.
    ///
    /// Rec. ITU-R BT.470-6 System B, G (historical)
    BT470BG = 5,
    /// - Rec. ITU-R BT.601-7 525 or 625
    /// - Rec. ITU-R BT.1358-1 525 or 625
    ///   (historical)
    /// - Rec. ITU-R BT.1700-0 NTSC
    /// - SMPTE ST 170 (2004)
    ///
    /// (functionally the same as the values 1, 14 and 15)
    ST170M = 6,
    /// SMPTE ST 240 (1999)
    ST240M = 7,
    /// Linear transfer characteristics
    Linear = 8,
    /// Logarithmic transfer characteristic
    /// (100:1 range)
    Logarithmic100 = 9,
    /// Logarithmic transfer characteristic
    /// (100 * Sqrt( 10 ) : 1 range)
    Logarithmic316 = 10,
    /// IEC 61966-2-4
    XVYCC = 11,
    /// Rec. ITU-R BT.1361-0 extended
    /// colour gamut system (historical)
    BT1361E = 12,
    /// - IEC 61966-2-1 sRGB (with
    ///   MatrixCoefficients equal to 0)
    /// - IEC 61966-2-1 sYCC (with
    ///   MatrixCoefficients equal to 5)
    SRGB = 13,
    /// Rec. ITU-R BT.2020-2 (10-bit system)
    ///
    /// (functionally the same as the values 1, 6 and 15)
    BT2020Ten = 14,
    /// Rec. ITU-R BT.2020-2 (12-bit system)
    ///
    /// (functionally the same as the values 1, 6 and 14)
    BT2020Twelve = 15,
    /// - SMPTE ST 2084 (2014) for 10-, 12-,
    ///   14- and 16-bit systems
    /// - Rec. ITU-R BT.2100-2 perceptual
    ///   quantization (PQ) system
    PerceptualQuantizer = 16,
    /// SMPTE ST 428-1 (2019)
    ST428 = 17,
    /// - ARIB STD-B67 (2015)
    /// - Rec. ITU-R BT.2100-2 hybrid log-
    ///   gamma (HLG) system
    HybridLogGamma = 18,
}

impl fmt::Display for TransferCharacteristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TransferCharacteristic::Reserved0 => write!(f, "Identity"),
            TransferCharacteristic::BT1886 => write!(f, "ITU BT.1886"),
            TransferCharacteristic::Unspecified => write!(f, "Unspecified"),
            TransferCharacteristic::Reserved => write!(f, "Reserved"),
            TransferCharacteristic::BT470M => write!(f, "ITU BT.470M"),
            TransferCharacteristic::BT470BG => write!(f, "ITU BT.470BG"),
            TransferCharacteristic::ST170M => write!(f, "SMPTE ST-170M"),
            TransferCharacteristic::ST240M => write!(f, "SMPTE ST-240M"),
            TransferCharacteristic::Linear => write!(f, "Linear"),
            TransferCharacteristic::Logarithmic100 => write!(f, "Logarithmic 100:1 range"),
            TransferCharacteristic::Logarithmic316 => write!(f, "Logarithmic 316:1 range"),
            TransferCharacteristic::XVYCC => write!(f, "XVYCC"),
            TransferCharacteristic::BT1361E => write!(f, "ITU BT.1361 Extended Color Gamut"),
            TransferCharacteristic::SRGB => write!(f, "sRGB"),
            TransferCharacteristic::BT2020Ten => write!(f, "ITU BT.2020 for 10bit systems"),
            TransferCharacteristic::BT2020Twelve => write!(f, "ITU BT.2020 for 12bit systems"),
            TransferCharacteristic::PerceptualQuantizer => write!(f, "Perceptual Quantizer"),
            TransferCharacteristic::ST428 => write!(f, "SMPTE ST-428"),
            TransferCharacteristic::HybridLogGamma => write!(f, "Hybrid Log-Gamma"),
        }
    }
}

/// Indicates the chroma sampling grid alignment for video fields or frames using the 4:2:0
/// colour format (in which the two chroma arrays have half the width
/// and half the height of the associated luma array)
///
/// Values adopted from Table 4 of ISO/IEC 23001-8:2013/DCOR1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ChromaLocation {
    Unspecified = 0,
    Left,
    Center,
    TopLeft,
    Top,
    BottomLeft,
    Bottom,
}

impl fmt::Display for ChromaLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ChromaLocation::*;
        match *self {
            Unspecified => write!(f, "Unspecified"),
            Left => write!(f, "Left"),
            Center => write!(f, "Center"),
            TopLeft => write!(f, "TopLeft"),
            Top => write!(f, "Top"),
            BottomLeft => write!(f, "BottomLeft"),
            Bottom => write!(f, "Bottom"),
        }
    }
}

/// All YUV color representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum YUVSystem {
    /// YCbCr is a family of color spaces used as a part of the color image pipeline
    /// in video and digital photography systems. Y′ is the luma component and CB and CR
    /// are the blue-difference and red-difference chroma components.
    YCbCr(YUVRange),
    /// The YCoCg color model, also known as the YCgCo color model,
    /// is the color space formed from a simple transformation of
    /// an associated RGB color space into a luma value and
    /// two chroma values called chrominance green and chrominance orange.
    YCoCg,
    /// ICtCp is a color representation format specified in the Rec. ITU-R BT.2100 standard
    /// that is used as a part of the color image pipeline in video and digital photography
    /// systems for high dynamic range (HDR) and wide color gamut (WCG) imagery.
    ICtCp,
}

impl fmt::Display for YUVSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::YUVSystem::*;
        match *self {
            YCbCr(range) => write!(f, "YCbCr ({})", range),
            YCoCg => write!(f, "YCbCg"),
            ICtCp => write!(f, "ICtCp"),
        }
    }
}

/// Trichromatic color encoding system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum TrichromaticEncodingSystem {
    /// Image represented by three color channels: Red, Green, and Blue.
    RGB,
    /// Image represented by a luminance (luma) channel and two chroma channels.
    YUV(YUVSystem),
    /// In the CIE 1931 model, Y is the luminance, Z is quasi-equal to blue (of CIE RGB),
    /// and X is a mix of the three CIE RGB curves chosen to be nonnegative.
    /// Setting Y as luminance has the useful result that for any given Y value,
    /// the XZ plane will contain all possible chromaticities at that luminance.
    XYZ,
}

impl fmt::Display for TrichromaticEncodingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::TrichromaticEncodingSystem::*;
        match *self {
            YUV(system) => write!(f, "{}", system),
            RGB => write!(f, "RGB"),
            XYZ => write!(f, "XYZ"),
        }
    }
}

/// All supported color models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum ColorModel {
    /// An image represented by three channels or planes: Includes RGB, YUV, and XYZ.
    Trichromatic(TrichromaticEncodingSystem),
    /// The CMYK color model is a subtractive color model, based on the CMY color model,
    /// used in color printing, and is also used to describe the printing process itself.
    /// CMYK refers to the four ink plates used in some color printing: cyan, magenta, yellow, and key.
    CMYK,
    /// HSL and HSV are alternative representations of the RGB color model,
    /// designed in the 1970s by computer graphics researchers to more closely align
    /// with the way human vision perceives color-making attributes.
    HSV,
    /// The CIELAB color space expresses color as three values:
    /// L* for perceptual lightness, and a* and b* for the four unique colors of human vision:
    /// red, green, blue, and yellow.
    LAB,
}

impl fmt::Display for ColorModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ColorModel::Trichromatic(system) => write!(f, "{}", system),
            ColorModel::CMYK => write!(f, "CMYK"),
            ColorModel::HSV => write!(f, "HSV"),
            ColorModel::LAB => write!(f, "LAB"),
        }
    }
}

impl ColorModel {
    /// Returns the number of components of a color model.
    pub fn get_default_components(self) -> usize {
        match self {
            ColorModel::CMYK => 4,
            _ => 3,
        }
    }
}

/// Single colorspace component definition.
///
/// Defines how the components of a colorspace are subsampled and
/// where and how they are stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Chromaton {
    /// Horizontal subsampling in power of two
    /// (e.g. `0` = no subsampling, `1` = only every second value is stored).
    pub h_ss: u8,
    /// Vertical subsampling in power of two
    /// (e.g. `0` = no subsampling, `1` = only every second value is stored).
    pub v_ss: u8,
    /// Tells if a component is packed.
    pub packed: bool,
    /// Bit depth of a component.
    pub depth: u8,
    /// Shift for packed components.
    pub shift: u8,
    /// Component offset for byte-packed components.
    pub comp_offs: u8,
    /// The distance to the next packed element in bytes.
    pub next_elem: u8,
}

fn align(v: usize, a: usize) -> usize {
    (v + a - 1) & !(a - 1)
}

impl Chromaton {
    /// Constructs a new `Chromaton` instance.
    pub const fn new(
        h_ss: u8,
        v_ss: u8,
        packed: bool,
        depth: u8,
        shift: u8,
        comp_offs: u8,
        next_elem: u8,
    ) -> Self {
        Chromaton {
            h_ss,
            v_ss,
            packed,
            depth,
            shift,
            comp_offs,
            next_elem,
        }
    }

    /// Constructs a specific `Chromaton` instance for `yuv8`.
    pub const fn yuv8(h_ss: u8, v_ss: u8, comp_offs: u8) -> Chromaton {
        Chromaton::new(h_ss, v_ss, false, 8, 0, comp_offs, 1)
    }

    /// Constructs a specific `Chromaton` instance for `yuvhb`.
    pub const fn yuvhb(h_ss: u8, v_ss: u8, depth: u8, comp_offs: u8) -> Chromaton {
        Chromaton::new(h_ss, v_ss, false, depth, 0, comp_offs, 1)
    }

    /// Constructs a specific `Chromaton` instance for `packrgb`.
    pub const fn packrgb(depth: u8, shift: u8, comp_offs: u8, next_elem: u8) -> Chromaton {
        Chromaton::new(0, 0, true, depth, shift, comp_offs, next_elem)
    }

    /// Constructs a specific `Chromaton` instance for `pal8`.
    pub const fn pal8(comp_offs: u8) -> Chromaton {
        Chromaton::new(0, 0, true, 8, 0, comp_offs, 3)
    }

    /// Returns the subsampling of a component.
    pub fn get_subsampling(self) -> (u8, u8) {
        (self.h_ss, self.v_ss)
    }

    /// Tells whether a component is packed.
    pub fn is_packed(self) -> bool {
        self.packed
    }

    /// Returns the bit depth of a component.
    pub fn get_depth(self) -> u8 {
        self.depth
    }

    /// Returns the bit shift of a packed component.
    pub fn get_shift(self) -> u8 {
        self.shift
    }

    /// Returns the byte offset of a packed component.
    pub fn get_offset(self) -> u8 {
        self.comp_offs
    }

    /// Returns the byte offset to the next element of a packed component.
    pub fn get_step(self) -> u8 {
        self.next_elem
    }

    /// Calculates the width for a component from general image width.
    pub fn get_width(self, width: usize) -> usize {
        (width + ((1 << self.h_ss) - 1)) >> self.h_ss
    }

    /// Calculates the height for a component from general image height.
    pub fn get_height(self, height: usize) -> usize {
        (height + ((1 << self.v_ss) - 1)) >> self.v_ss
    }

    /// Calculates the minimal stride for a component from general image width.
    pub fn get_linesize(self, width: usize, alignment: usize) -> usize {
        let d = self.depth as usize;
        align((self.get_width(width) * d + d - 1) >> 3, alignment)
    }

    /// Calculates the required image size in pixels for a component
    /// from general image width.
    pub fn get_data_size(self, width: usize, height: usize, align: usize) -> usize {
        let nh = (height + ((1 << self.v_ss) - 1)) >> self.v_ss;
        self.get_linesize(width, align) * nh
    }
}

impl fmt::Display for Chromaton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pfmt = if self.packed {
            let mask = ((1 << self.depth) - 1) << self.shift;
            format!(
                "packed(+{},{:X}, step {})",
                self.comp_offs, mask, self.next_elem
            )
        } else {
            format!("planar({},{})", self.comp_offs, self.next_elem)
        };
        write!(f, "({}x{}, {})", self.h_ss, self.v_ss, pfmt)
    }
}

/// Image colorspace representation.
///
/// Includes both definitions for each component and some common definitions.
///
/// For example, the format can be paletted, so the components describe
/// the palette storage format, while the actual data is 8-bit palette indices.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Formaton {
    /// Image color model.
    pub model: ColorModel,
    /// Image color primaries.
    pub primaries: ColorPrimaries,
    /// Image transfer characteristic.
    pub xfer: TransferCharacteristic,
    /// Image matrix coefficients.
    pub matrix: MatrixCoefficients,
    /// Image chroma location.
    pub chroma_location: ChromaLocation,

    /// Actual number of components present.
    pub components: u8,
    /// Format definition for each component.
    pub comp_info: [Option<Chromaton>; 5],
    /// Single pixel size for packed formats.
    pub elem_size: u8,
    /// Tells if data is stored as big-endian.
    pub be: bool,
    /// Tells if image has alpha component.
    pub alpha: bool,
    /// Tells if data is paletted.
    pub palette: bool,
}

impl Formaton {
    /// Constructs a new instance of `Formaton`.
    pub fn new(
        model: ColorModel,
        components: &[Chromaton],
        elem_size: u8,
        be: bool,
        alpha: bool,
        palette: bool,
    ) -> Self {
        let mut c: [Option<Chromaton>; 5] = [None; 5];

        if components.len() > 5 {
            panic!("too many components");
        }

        for (i, v) in components.iter().enumerate() {
            c[i] = Some(*v);
        }

        Formaton {
            model,

            primaries: ColorPrimaries::Unspecified,
            xfer: TransferCharacteristic::Unspecified,
            matrix: MatrixCoefficients::Unspecified,
            chroma_location: ChromaLocation::Unspecified,

            components: components.len() as u8,
            comp_info: c,
            elem_size,
            be,
            alpha,
            palette,
        }
    }

    /// Returns current color model.
    pub fn get_model(&self) -> ColorModel {
        self.model
    }

    /// Returns current image primaries.
    pub fn get_primaries(&self) -> ColorPrimaries {
        self.primaries
    }

    /// Returns the total amount of bits needed for components.
    pub fn get_total_depth(&self) -> u8 {
        let mut depth = 0;
        for chromaton in self.comp_info.iter().flatten() {
            depth += chromaton.depth;
        }
        depth
    }

    /// Sets current image primaries.
    pub fn set_primaries(mut self, pc: ColorPrimaries) {
        self.primaries = pc;
    }

    /// Sets current image primaries from `u32`.
    pub fn set_primaries_from_u32(mut self, pc: u32) -> Option<ColorPrimaries> {
        let parsed_pc = ColorPrimaries::from_u32(pc);
        if let Some(pc) = parsed_pc {
            self.primaries = pc
        }
        parsed_pc
    }

    /// Returns current image transfer characteristic.
    pub fn get_xfer(&self) -> TransferCharacteristic {
        self.xfer
    }

    /// Sets current image transfer characteristic.
    pub fn set_xfer(mut self, pc: TransferCharacteristic) {
        self.xfer = pc;
    }

    /// Sets current image transfer characteristic from `u32`.
    pub fn set_xfer_from_u32(mut self, tc: u32) -> Option<TransferCharacteristic> {
        let parsed_tc = TransferCharacteristic::from_u32(tc);
        if let Some(tc) = parsed_tc {
            self.xfer = tc
        }
        parsed_tc
    }

    /// Returns current image matrix coefficients.
    pub fn get_matrix(&self) -> MatrixCoefficients {
        self.matrix
    }

    /// Sets current image matrix coefficients.
    pub fn set_matrix(mut self, mc: MatrixCoefficients) {
        self.matrix = mc;
    }

    /// Sets current image matrix coefficients from `u32`.
    pub fn set_matrix_from_u32(mut self, mc: u32) -> Option<MatrixCoefficients> {
        let parsed_mc = MatrixCoefficients::from_u32(mc);
        if let Some(mc) = parsed_mc {
            self.matrix = mc
        }
        parsed_mc
    }

    /// Returns the number of components.
    pub fn get_num_comp(&self) -> usize {
        self.components as usize
    }
    /// Returns selected component information.
    pub fn get_chromaton(&self, idx: usize) -> Option<Chromaton> {
        if idx < self.comp_info.len() {
            return self.comp_info[idx];
        }
        None
    }

    /// Reports whether the packing format is big-endian.
    pub fn is_be(&self) -> bool {
        self.be
    }

    /// Reports whether a colorspace has an alpha component.
    pub fn has_alpha(&self) -> bool {
        self.alpha
    }

    /// Reports whether this is a paletted format.
    pub fn is_paletted(&self) -> bool {
        self.palette
    }

    /// Returns single packed pixel size.
    pub fn get_elem_size(&self) -> u8 {
        self.elem_size
    }

    /// Returns an iterator over the format definition of each component.
    pub fn iter(&self) -> slice::Iter<Option<Chromaton>> {
        self.comp_info.iter()
    }
}

impl<'a> Index<usize> for &'a Formaton {
    type Output = Option<Chromaton>;

    fn index(&self, index: usize) -> &Self::Output {
        self.comp_info.index(index)
    }
}

impl<'a> IntoIterator for &'a Formaton {
    type Item = &'a Option<Chromaton>;
    type IntoIter = slice::Iter<'a, Option<Chromaton>>;

    fn into_iter(self) -> Self::IntoIter {
        self.comp_info.iter()
    }
}

impl fmt::Display for Formaton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let end = if self.be { "BE" } else { "LE" };
        let palstr = if self.palette { "palette " } else { "" };
        let astr = if self.alpha { "alpha " } else { "" };
        let mut str = format!(
            "Formaton for {} ({}{}elem {} size {}): ",
            self.model, palstr, astr, end, self.elem_size
        );
        for &i in self.into_iter() {
            if let Some(chr) = i {
                str = format!("{} {}", str, chr);
            }
        }
        write!(f, "[{}]", str)
    }
}

pub mod formats {
    //!
    //! Ready-to-use formaton
    //!

    use self::ColorModel::*;
    use self::TrichromaticEncodingSystem::*;
    use self::YUVRange::*;
    use self::YUVSystem::*;
    use crate::pixel::*;

    /// Predefined format for planar 8-bit YUV with 4:4:4 subsampling.
    pub const YUV444: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 8, 0, 0, 1)),
            Some(Chromaton::yuv8(0, 0, 1)),
            Some(Chromaton::yuv8(0, 0, 2)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for planar 8-bit YUV with 4:2:2 subsampling.
    pub const YUV422: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 8, 0, 0, 1)),
            Some(Chromaton::yuv8(0, 1, 1)),
            Some(Chromaton::yuv8(0, 1, 2)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for planar 8-bit YUV with 4:2:0 subsampling.
    pub const YUV420: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 8, 0, 0, 1)),
            Some(Chromaton::yuv8(1, 1, 1)),
            Some(Chromaton::yuv8(1, 1, 2)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for planar 8-bit YUV with 4:1:1 subsampling.
    pub const YUV411: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 8, 0, 0, 1)),
            Some(Chromaton::yuv8(2, 0, 1)),
            Some(Chromaton::yuv8(2, 0, 2)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for planar 8-bit YUV with 4:1:0 subsampling.
    pub const YUV410: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 8, 0, 0, 1)),
            Some(Chromaton::yuv8(2, 1, 1)),
            Some(Chromaton::yuv8(2, 1, 2)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for planar 10-bit YUV with 4:4:4 subsampling.
    pub const YUV444_10: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 10, 0, 0, 1)),
            Some(Chromaton::yuvhb(0, 0, 1, 10)),
            Some(Chromaton::yuvhb(0, 0, 2, 10)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for planar 10-bit YUV with 4:2:2 subsampling.
    pub const YUV422_10: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 10, 0, 0, 1)),
            Some(Chromaton::yuvhb(0, 1, 1, 10)),
            Some(Chromaton::yuvhb(0, 1, 2, 10)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for planar 10-bit YUV with 4:2:0 subsampling.
    pub const YUV420_10: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 10, 0, 0, 1)),
            Some(Chromaton::yuvhb(1, 1, 1, 10)),
            Some(Chromaton::yuvhb(1, 1, 2, 10)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for planar 10-bit YUV with 4:1:1 subsampling.
    pub const YUV411_10: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 10, 0, 0, 1)),
            Some(Chromaton::yuvhb(2, 0, 1, 10)),
            Some(Chromaton::yuvhb(2, 0, 2, 10)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for planar 10-bit YUV with 4:1:0 subsampling.
    pub const YUV410_10: &Formaton = &Formaton {
        model: Trichromatic(YUV(YCbCr(Limited))),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::new(0, 0, false, 10, 0, 0, 1)),
            Some(Chromaton::yuvhb(2, 1, 1, 10)),
            Some(Chromaton::yuvhb(2, 1, 2, 10)),
            None,
            None,
        ],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format with RGB24 palette.
    pub const PAL8: &Formaton = &Formaton {
        model: Trichromatic(RGB),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::pal8(0)),
            Some(Chromaton::pal8(1)),
            Some(Chromaton::pal8(2)),
            None,
            None,
        ],
        elem_size: 3,
        be: false,
        alpha: false,
        palette: true,
    };

    /// Predefined format for RGB565 packed video.
    pub const RGB565: &Formaton = &Formaton {
        model: Trichromatic(RGB),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::packrgb(5, 11, 0, 2)),
            Some(Chromaton::packrgb(6, 5, 0, 2)),
            Some(Chromaton::packrgb(5, 0, 0, 2)),
            None,
            None,
        ],
        elem_size: 2,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for RGB24.
    pub const RGB24: &Formaton = &Formaton {
        model: Trichromatic(RGB),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::packrgb(8, 0, 2, 3)),
            Some(Chromaton::packrgb(8, 0, 1, 3)),
            Some(Chromaton::packrgb(8, 0, 0, 3)),
            None,
            None,
        ],
        elem_size: 3,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for RGBA.
    pub const RGBA: &Formaton = &Formaton {
        model: Trichromatic(RGB),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 4,
        comp_info: [
            Some(Chromaton::packrgb(8, 0, 3, 4)),
            Some(Chromaton::packrgb(8, 0, 2, 4)),
            Some(Chromaton::packrgb(8, 0, 1, 4)),
            Some(Chromaton::packrgb(8, 0, 0, 4)),
            None,
        ],
        elem_size: 4,
        be: false,
        alpha: true,
        palette: false,
    };

    /// Predefined format for RGB48.
    pub const RGB48: &Formaton = &Formaton {
        model: Trichromatic(RGB),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 3,
        comp_info: [
            Some(Chromaton::packrgb(16, 0, 2, 6)),
            Some(Chromaton::packrgb(16, 0, 1, 6)),
            Some(Chromaton::packrgb(16, 0, 0, 6)),
            None,
            None,
        ],
        elem_size: 6,
        be: false,
        alpha: false,
        palette: false,
    };

    /// Predefined format for RGBA64.
    pub const RGBA64: &Formaton = &Formaton {
        model: Trichromatic(RGB),
        primaries: ColorPrimaries::Unspecified,
        xfer: TransferCharacteristic::Unspecified,
        matrix: MatrixCoefficients::Unspecified,
        chroma_location: ChromaLocation::Unspecified,
        components: 4,
        comp_info: [
            Some(Chromaton::packrgb(16, 0, 3, 8)),
            Some(Chromaton::packrgb(16, 0, 2, 8)),
            Some(Chromaton::packrgb(16, 0, 1, 8)),
            Some(Chromaton::packrgb(16, 0, 0, 8)),
            None,
        ],
        elem_size: 8,
        be: false,
        alpha: true,
        palette: false,
    };
}

#[cfg(test)]
mod test {
    mod formats {
        use super::super::*;
        #[test]
        fn fmt() {
            println!("formaton yuv- {}", formats::YUV420);
            println!("formaton pal- {}", formats::PAL8);
            println!("formaton rgb565- {}", formats::RGB565);
            println!("formaton rgba- {}", formats::RGBA);
            println!("formaton rgb48- {}", formats::RGB48);
            println!("formaton rgba64- {}", formats::RGBA64);
        }

        #[test]
        fn comparison() {
            use std::sync::Arc;
            let rcf = Arc::new(*formats::YUV420);
            let cf = &formats::YUV420.clone();

            if cf != formats::YUV420 {
                panic!("cf");
            }

            if *rcf != *formats::YUV420 {
                panic!("rcf");
            }
        }
    }
}
