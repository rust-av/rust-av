//! Expose all data and methods to interact with color spaces.

use std::fmt;

use num_derive::{FromPrimitive, ToPrimitive};

/// YUV color range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YUVRange {
    /// Pixels in the range [16, 235].
    Limited,
    /// Pixels in the range [0, 255].
    Full,
}

impl fmt::Display for YUVRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            YUVRange::Limited => "Limited range",
            YUVRange::Full => "Full range",
        };
        s.fmt(f)
    }
}

/// Describes the matrix coefficients used in deriving
/// luma and chroma signals from the green, blue and red or X, Y and Z primaries.
///
/// Values adopted from Table 4 of ISO/IEC 23001-8:2013/DCOR1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
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
    /// Frame characteristics are unknown or are determined by the application.
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
        let s = match self {
            MatrixCoefficients::Identity => "Identity",
            MatrixCoefficients::BT709 => "ITU BT.709",
            MatrixCoefficients::Unspecified => "Unspecified",
            MatrixCoefficients::Reserved => "Reserved",
            MatrixCoefficients::BT470M => "ITU BT.470M",
            MatrixCoefficients::BT470BG => "ITU BT.470BG",
            MatrixCoefficients::ST170M => "SMPTE ST-170M",
            MatrixCoefficients::ST240M => "SMPTE ST-240M",
            MatrixCoefficients::YCgCo => "YCgCo",
            MatrixCoefficients::BT2020NonConstantLuminance => {
                "ITU BT.2020 (Non Constant Luminance)"
            }
            MatrixCoefficients::BT2020ConstantLuminance => "ITU BT.2020 (Constant Luminance)",
            MatrixCoefficients::ST2085 => "SMPTE ST-2085",
            MatrixCoefficients::ChromaticityDerivedNonConstantLuminance => {
                "Chromaticity Derived (Non ConstantLuminance)"
            }
            MatrixCoefficients::ChromaticityDerivedConstantLuminance => {
                "Chromaticity Derived (Constant Luminance)"
            }
            MatrixCoefficients::ICtCp => "ICtCp",
        };
        s.fmt(f)
    }
}

/// Indicates the chromaticity coordinates of the source colour primaries as specified in Table 2 in terms
/// of the CIE 1931 definition of x and y as specified by ISO 11664-1.
///
/// Values adopted from Table 4 of ISO/IEC 23001-8:2013/DCOR1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
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
    /// Frame characteristics are unknown or are determined by
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
        let s = match self {
            ColorPrimaries::Reserved0 => "Identity",
            ColorPrimaries::BT709 => "ITU BT.709",
            ColorPrimaries::Unspecified => "Unspecified",
            ColorPrimaries::Reserved => "Reserved",
            ColorPrimaries::BT470M => "ITU BT.470M",
            ColorPrimaries::BT470BG => "ITU BT.470BG",
            ColorPrimaries::ST170M => "SMPTE ST-170M",
            ColorPrimaries::ST240M => "SMPTE ST-240M",
            ColorPrimaries::Film => "Film",
            ColorPrimaries::BT2020 => "ITU BT.2020",
            ColorPrimaries::ST428 => "SMPTE ST-428",
            ColorPrimaries::P3DCI => "DCI P3",
            ColorPrimaries::P3Display => "Display P3",
            ColorPrimaries::Tech3213 => "EBU Tech3213",
        };
        s.fmt(f)
    }
}

/// Either indicates the reference opto-electronic transfer characteristic
/// function of the source picture as a function of a source input linear optical intensity
/// input Lc with a nominal real-valued range of 0 to 1 or indicates the inverse of the
/// reference electro-optical transfer characteristic function as a function of an
/// output linear optical intensity Lo with a nominal real-valued range of 0 to 1.
///
/// Values adopted from Table 4 of ISO/IEC 23001-8:2013/DCOR1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum TransferCharacteristic {
    /// For future use by ITU-T | ISO/IEC.
    Reserved0 = 0,
    /// - Rec. ITU-R BT.709-6
    /// - Rec. ITU-R BT.1361-0 conventional
    ///   colour gamut system (historical)
    ///
    /// (functionally the same as the values 6, 14 and 15)
    BT1886 = 1,
    /// Frame characteristics are unknown or
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
        let s = match self {
            TransferCharacteristic::Reserved0 => "Identity",
            TransferCharacteristic::BT1886 => "ITU BT.1886",
            TransferCharacteristic::Unspecified => "Unspecified",
            TransferCharacteristic::Reserved => "Reserved",
            TransferCharacteristic::BT470M => "ITU BT.470M",
            TransferCharacteristic::BT470BG => "ITU BT.470BG",
            TransferCharacteristic::ST170M => "SMPTE ST-170M",
            TransferCharacteristic::ST240M => "SMPTE ST-240M",
            TransferCharacteristic::Linear => "Linear",
            TransferCharacteristic::Logarithmic100 => "Logarithmic 100:1 range",
            TransferCharacteristic::Logarithmic316 => "Logarithmic 316:1 range",
            TransferCharacteristic::XVYCC => "XVYCC",
            TransferCharacteristic::BT1361E => "ITU BT.1361 Extended Color Gamut",
            TransferCharacteristic::SRGB => "sRGB",
            TransferCharacteristic::BT2020Ten => "ITU BT.2020 for 10bit systems",
            TransferCharacteristic::BT2020Twelve => "ITU BT.2020 for 12bit systems",
            TransferCharacteristic::PerceptualQuantizer => "Perceptual Quantizer",
            TransferCharacteristic::ST428 => "SMPTE ST-428",
            TransferCharacteristic::HybridLogGamma => "Hybrid Log-Gamma",
        };
        s.fmt(f)
    }
}

/// Chroma sampling grid alignment for frames using the 4:2:0
/// color format (in which the two chroma arrays have half the width
/// and half the height of the associated luma array)
///
/// Values adopted from Table 4 of ISO/IEC 23001-8:2013/DCOR1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromaLocation {
    /// Unspecified grid alignment.
    Unspecified = 0,
    /// Chroma left sampling grid alignment.
    Left,
    /// Chroma center sampling grid alignment.
    Center,
    /// Chroma top left sampling grid alignment.
    TopLeft,
    /// Chroma top sampling grid alignment.
    Top,
    /// Chroma bottom left sampling grid alignment.
    BottomLeft,
    /// Chroma bottom sampling grid alignment.
    Bottom,
}

impl fmt::Display for ChromaLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ChromaLocation::Unspecified => "Unspecified",
            ChromaLocation::Left => "Left",
            ChromaLocation::Center => "Center",
            ChromaLocation::TopLeft => "TopLeft",
            ChromaLocation::Top => "Top",
            ChromaLocation::BottomLeft => "BottomLeft",
            ChromaLocation::Bottom => "Bottom",
        };
        s.fmt(f)
    }
}

/// All YUV color representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YUVSystem {
    /// YCbCr is a family of color spaces used as a part of the color frame pipeline
    /// in video and digital photography systems. Y′ is the luma plane and CB and CR
    /// are the blue-difference and red-difference chroma planes.
    YCbCr(YUVRange),
    /// The YCoCg color model, also known as the YCgCo color model,
    /// is the color space formed from a simple transformation of
    /// an associated RGB color space into a luma value and
    /// two chroma values called chrominance green and chrominance orange.
    YCoCg,
    /// ICtCp is a color representation format specified in the Rec. ITU-R BT.2100 standard
    /// that is used as a part of the color frame pipeline in video and digital photography
    /// systems for high dynamic range (HDR) and wide color gamut (WCG) imagery.
    ICtCp,
}

impl fmt::Display for YUVSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YUVSystem::YCbCr(range) => write!(f, "YCbCr ({})", range),
            YUVSystem::YCoCg => write!(f, "YCbCg"),
            YUVSystem::ICtCp => write!(f, "ICtCp"),
        }
    }
}

/// Trichromatic color encoding system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrichromaticEncodingSystem {
    /// Frame represented by three color channels: Red, Green, and Blue.
    RGB,
    /// Frame represented by a luminance (luma) channel and two chroma channels.
    YUV(YUVSystem),
    /// In the CIE 1931 model, Y is the luminance, Z is quasi-equal to blue (of CIE RGB),
    /// and X is a mix of the three CIE RGB curves chosen to be nonnegative.
    /// Setting Y as luminance has the useful result that for any given Y value,
    /// the XZ plane will contain all possible chromaticities at that luminance.
    XYZ,
}

impl fmt::Display for TrichromaticEncodingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrichromaticEncodingSystem::YUV(system) => write!(f, "{}", system),
            TrichromaticEncodingSystem::RGB => write!(f, "RGB"),
            TrichromaticEncodingSystem::XYZ => write!(f, "XYZ"),
        }
    }
}

/// All supported color models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorModel {
    /// A frame represented by three channels (or planes).
    ///
    /// Includes RGB, YUV, and XYZ.
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
        match self {
            ColorModel::Trichromatic(system) => write!(f, "{}", system),
            ColorModel::CMYK => write!(f, "CMYK"),
            ColorModel::HSV => write!(f, "HSV"),
            ColorModel::LAB => write!(f, "LAB"),
        }
    }
}

impl ColorModel {
    /// Returns the number of components of a color model.
    pub const fn default_components(self) -> usize {
        match self {
            ColorModel::CMYK => 4,
            _ => 3,
        }
    }
}
