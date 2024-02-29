//! Expose all data and methods to represent pixels.

use std::fmt;
use std::ops::Index;

use num_traits::FromPrimitive;

use crate::colorspace::{
    ChromaLocation, ColorModel, ColorPrimaries, MatrixCoefficients, TransferCharacteristic,
};

const fn align(v: usize, a: usize) -> usize {
    (v + a - 1) & !(a - 1)
}

/// Chroma definition for plane pixels.
///
/// Defines how plane pixels are subsampled and stored in memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chromaton {
    /// Horizontal subsampling in power of two
    /// (e.g. `0` = no subsampling, `1` = only every second value is stored).
    pub horizontal_subsampling: u8,
    /// Vertical subsampling in power of two
    /// (e.g. `0` = no subsampling, `1` = only every second value is stored).
    pub vertical_subsampling: u8,
    /// Tells whether plane pixels are packed.
    pub packed: bool,
    /// Bit depth for plane pixels.
    pub bit_depth: u8,
    /// Shift for a packed plane.
    pub shift: u8,
    /// Byte offset for a packed plane.
    pub plane_offset: u8,
    /// Byte offset to the next plane pixel in bytes.
    pub next_pixel: u8,
}

impl Chromaton {
    /// Constructs a new `Chromaton` instance.
    pub const fn new(
        horizontal_subsampling: u8,
        vertical_subsampling: u8,
        packed: bool,
        bit_depth: u8,
        shift: u8,
        plane_offset: u8,
        next_pixel: u8,
    ) -> Self {
        Self {
            horizontal_subsampling,
            vertical_subsampling,
            packed,
            bit_depth,
            shift,
            plane_offset,
            next_pixel,
        }
    }

    /// Constructs a specific `Chromaton` instance for `yuv8`.
    pub const fn yuv8(
        horizontal_subsampling: u8,
        vertical_subsamplig: u8,
        plane_offsetet: u8,
    ) -> Chromaton {
        Chromaton::new(
            horizontal_subsampling,
            vertical_subsamplig,
            false,
            8,
            0,
            plane_offsetet,
            1,
        )
    }

    /// Constructs a specific `Chromaton` instance for `yuvhb`.
    pub const fn yuvhb(
        horizontal_subsampling: u8,
        vertical_subsampling: u8,
        depth: u8,
        plane_offset: u8,
    ) -> Chromaton {
        Chromaton::new(
            horizontal_subsampling,
            vertical_subsampling,
            false,
            depth,
            0,
            plane_offset,
            1,
        )
    }

    /// Constructs a specific `Chromaton` instance for `packrgb`.
    pub const fn packrgb(depth: u8, shift: u8, plane_offset: u8, next_pixel: u8) -> Chromaton {
        Chromaton::new(0, 0, true, depth, shift, plane_offset, next_pixel)
    }

    /// Constructs a specific `Chromaton` instance for `pal8`.
    pub const fn pal8(plane_offset: u8) -> Chromaton {
        Chromaton::new(0, 0, true, 8, 0, plane_offset, 3)
    }

    /// Calculates plane width from frame width.
    pub const fn width(self, width: usize) -> usize {
        (width + ((1 << self.horizontal_subsampling) - 1)) >> self.horizontal_subsampling
    }

    /// Calculates plane height from frame height.
    pub const fn height(self, height: usize) -> usize {
        (height + ((1 << self.vertical_subsampling) - 1)) >> self.vertical_subsampling
    }

    /// Calculates the minimal plane stride from frame width.
    pub const fn linesize(self, width: usize, alignment: usize) -> usize {
        let d = self.bit_depth as usize;
        align((self.width(width) * d + d - 1) >> 3, alignment)
    }

    /// Calculates the required plane size in pixels from frame width, height,
    /// and a specific alignment.
    pub const fn size(self, width: usize, height: usize, align: usize) -> usize {
        let nh = (height + ((1 << self.vertical_subsampling) - 1)) >> self.vertical_subsampling;
        self.linesize(width, align) * nh
    }
}

impl fmt::Display for Chromaton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pfmt = if self.packed {
            let mask = ((1 << self.bit_depth) - 1) << self.shift;
            format!(
                "packed(+{},{:X}, step {})",
                self.plane_offset, mask, self.next_pixel
            )
        } else {
            format!("planar({},{})", self.plane_offset, self.next_pixel)
        };
        write!(
            f,
            "({}x{}, {})",
            self.horizontal_subsampling, self.vertical_subsampling, pfmt
        )
    }
}

/// Pixel representation.
///
/// It contains color space information such as color models, primaries,
/// color space conversion functions, and chroma location.
///
/// This representation also contains additional information about pixels
/// such as their chroma definition.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pixel {
    /// Pixel color model.
    pub model: ColorModel,

    /// Pixel color primaries.
    pub primaries: ColorPrimaries,
    /// Pixel transfer characteristic.
    pub xfer: TransferCharacteristic,
    /// Pixel matrix coefficients.
    pub matrix: MatrixCoefficients,
    /// Pixel chroma location.
    pub chroma_location: ChromaLocation,

    /// Pixel definition for every plane.
    ///
    /// A whole pixel is formed overlapping the pixels of each plane.
    pub pixel_chroma_info: [Option<Chromaton>; 5],

    /// Pixel size in bytes for packed frame formats.
    pub pixel_size: u8,
    /// Tells whether a pixel is stored as big-endian.
    pub is_big_endian: bool,
    /// Tells whether a pixel has an alpha plane.
    pub has_alpha_plane: bool,
    /// Tells whether a pixel is paletted.
    pub is_paletted: bool,
}

impl Pixel {
    /// Constructs a new `Pixe` instance.
    pub const fn new(model: ColorModel) -> Self {
        Self {
            model,

            primaries: ColorPrimaries::Unspecified,
            xfer: TransferCharacteristic::Unspecified,
            matrix: MatrixCoefficients::Unspecified,
            chroma_location: ChromaLocation::Unspecified,

            pixel_chroma_info: [None, None, None, None, None],

            pixel_size: 0,
            is_big_endian: false,
            has_alpha_plane: false,
            is_paletted: false,
        }
    }

    /// Sets colorspace information such as color models, primaries,
    /// color space conversion functions, and chroma location.
    pub const fn set_colorspace_info(
        mut self,
        primaries: ColorPrimaries,
        xfer: TransferCharacteristic,
        matrix: MatrixCoefficients,
        chroma_location: ChromaLocation,
    ) -> Self {
        self.primaries = primaries;
        self.xfer = xfer;
        self.matrix = matrix;
        self.chroma_location = chroma_location;
        self
    }

    /// Sets pixel size in bytes for packed formats.
    pub const fn size(mut self, pixel_size: u8) -> Self {
        self.pixel_size = pixel_size;
        self
    }

    /// Pixel data is stored as big-endian.
    pub const fn is_big_endian(mut self) -> Self {
        self.is_big_endian = true;
        self
    }

    /// Pixel has an alpha plane.
    pub const fn has_alpha_plane(mut self) -> Self {
        self.has_alpha_plane = true;
        self
    }

    /// Pixel data is paletted.
    pub const fn is_paletted(mut self) -> Self {
        self.is_paletted = true;
        self
    }

    /// Adds first chromaton pixel plane.
    pub const fn add_first_plane(mut self, chroma_data: Chromaton) -> Self {
        self.pixel_chroma_info[0] = Some(chroma_data);
        self
    }

    /// Adds second chromaton pixel plane.
    pub const fn add_second_plane(mut self, chroma_data: Chromaton) -> Self {
        self.pixel_chroma_info[1] = Some(chroma_data);
        self
    }

    /// Adds third chromaton pixel plane.
    pub const fn add_third_plane(mut self, chroma_data: Chromaton) -> Self {
        self.pixel_chroma_info[2] = Some(chroma_data);
        self
    }

    /// Adds fourth chromaton pixel plane.
    pub const fn add_fourth_plane(mut self, chroma_data: Chromaton) -> Self {
        self.pixel_chroma_info[3] = Some(chroma_data);
        self
    }

    /// Adds fifth chromaton pixel plane.
    pub const fn add_fifth_plane(mut self, chroma_data: Chromaton) -> Self {
        self.pixel_chroma_info[4] = Some(chroma_data);
        self
    }

    /// Sums every pixel plane bit depth.
    pub fn pixel_bit_depths_sum(&self) -> u8 {
        self.pixel_chroma_info
            .iter()
            .flatten()
            .map(|pixel_chroma_info| pixel_chroma_info.bit_depth)
            .sum()
    }

    /// Returns the number of planes which compose a pixel.
    pub fn planes(&self) -> usize {
        self.pixel_chroma_info.iter().count()
    }

    /// Sets current frame primaries from `u32`.
    pub fn set_primaries_from_u32(mut self, pc: u32) -> Option<ColorPrimaries> {
        let parsed_pc = ColorPrimaries::from_u32(pc);
        if let Some(pc) = parsed_pc {
            self.primaries = pc
        }
        parsed_pc
    }

    /// Sets current frame transfer characteristic from `u32`.
    pub fn set_xfer_from_u32(mut self, tc: u32) -> Option<TransferCharacteristic> {
        let parsed_tc = TransferCharacteristic::from_u32(tc);
        if let Some(tc) = parsed_tc {
            self.xfer = tc
        }
        parsed_tc
    }

    /// Sets current frame matrix coefficients from `u32`.
    pub fn set_matrix_from_u32(mut self, mc: u32) -> Option<MatrixCoefficients> {
        let parsed_mc = MatrixCoefficients::from_u32(mc);
        if let Some(mc) = parsed_mc {
            self.matrix = mc
        }
        parsed_mc
    }

    /// Returns the plane chromaton associated with the plane identifier.
    pub const fn chromaton(&self, idx: usize) -> Option<Chromaton> {
        self.pixel_chroma_info[idx]
    }

    /// Returns an iterator over the chromaton of each plane.
    pub fn iter(&self) -> std::slice::Iter<Option<Chromaton>> {
        self.pixel_chroma_info.iter()
    }
}

impl<'frame> Index<usize> for &'frame Pixel {
    type Output = Option<Chromaton>;

    fn index(&self, index: usize) -> &Self::Output {
        self.pixel_chroma_info.index(index)
    }
}

impl<'frame> IntoIterator for &'frame Pixel {
    type Item = &'frame Option<Chromaton>;
    type IntoIter = std::slice::Iter<'frame, Option<Chromaton>>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixel_chroma_info.iter()
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = if self.is_big_endian { "BE" } else { "LE" };
        let palstr = if self.is_paletted { "palette " } else { "" };
        let astr = if self.has_alpha_plane { "alpha " } else { "" };
        let mut str = format!(
            "Formaton for {} ({}{}elem {} size {}): ",
            self.model, palstr, astr, end, self.pixel_size
        );
        for &i in self.into_iter() {
            if let Some(chr) = i {
                str = format!("{} {}", str, chr);
            }
        }
        write!(f, "[{}]", str)
    }
}

/// A series of common pixel formats.
pub mod formats {
    //!
    //! A list of `Pixel` instances.
    //!

    use crate::colorspace::ColorModel;
    use crate::colorspace::TrichromaticEncodingSystem::{RGB, YUV};
    use crate::colorspace::YUVRange::Limited;
    use crate::colorspace::YUVSystem::YCbCr;

    use super::{Chromaton, Pixel};

    /// Predefined frame format for planar 8-bit YUV with 4:4:4 subsampling.
    pub const YUV444: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 8, 0, 0, 1))
        .add_second_plane(Chromaton::yuv8(0, 0, 1))
        .add_third_plane(Chromaton::yuv8(0, 0, 2));

    /// Predefined frame format for planar 8-bit YUV with 4:2:2 subsampling.
    pub const YUV422: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 8, 0, 0, 1))
        .add_second_plane(Chromaton::yuv8(0, 1, 1))
        .add_third_plane(Chromaton::yuv8(0, 1, 2));

    /// Predefined frame format for planar 8-bit YUV with 4:2:0 subsampling.
    pub const YUV420: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 8, 0, 0, 1))
        .add_second_plane(Chromaton::yuv8(1, 1, 1))
        .add_third_plane(Chromaton::yuv8(1, 1, 2));

    /// Predefined frame format for planar 8-bit YUV with 4:1:1 subsampling.
    pub const YUV411: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 8, 0, 0, 1))
        .add_second_plane(Chromaton::yuv8(2, 0, 1))
        .add_third_plane(Chromaton::yuv8(2, 0, 2));

    /// Predefined frame format for planar 8-bit YUV with 4:1:0 subsampling.
    pub const YUV410: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 8, 0, 0, 1))
        .add_second_plane(Chromaton::yuv8(2, 1, 1))
        .add_third_plane(Chromaton::yuv8(2, 1, 2));

    /// Predefined frame format for planar 10-bit YUV with 4:4:4 subsampling.
    pub const YUV444_10: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 10, 0, 0, 1))
        .add_second_plane(Chromaton::yuvhb(0, 0, 1, 10))
        .add_third_plane(Chromaton::yuvhb(0, 0, 2, 10));

    /// Predefined frame format for planar 10-bit YUV with 4:2:2 subsampling.
    pub const YUV422_10: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 10, 0, 0, 1))
        .add_second_plane(Chromaton::yuvhb(0, 1, 1, 10))
        .add_third_plane(Chromaton::yuvhb(0, 1, 2, 10));

    /// Predefined frame format for planar 10-bit YUV with 4:2:0 subsampling.
    pub const YUV420_10: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 10, 0, 0, 1))
        .add_second_plane(Chromaton::yuvhb(1, 1, 1, 10))
        .add_third_plane(Chromaton::yuvhb(1, 1, 2, 10));

    /// Predefined frame format for planar 10-bit YUV with 4:1:1 subsampling.
    pub const YUV411_10: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 10, 0, 0, 1))
        .add_second_plane(Chromaton::yuvhb(2, 0, 1, 10))
        .add_third_plane(Chromaton::yuvhb(2, 0, 2, 10));

    /// Predefined frame format for planar 10-bit YUV with 4:1:0 subsampling.
    pub const YUV410_10: &Pixel = &Pixel::new(ColorModel::Trichromatic(YUV(YCbCr(Limited))))
        .add_first_plane(Chromaton::new(0, 0, false, 10, 0, 0, 1))
        .add_second_plane(Chromaton::yuvhb(2, 1, 1, 10))
        .add_third_plane(Chromaton::yuvhb(2, 1, 2, 10));

    /// Predefined frame format with RGB24 palette.
    pub const PAL8: &Pixel = &Pixel::new(ColorModel::Trichromatic(RGB))
        .add_first_plane(Chromaton::pal8(0))
        .add_second_plane(Chromaton::pal8(1))
        .add_third_plane(Chromaton::pal8(2))
        .size(3)
        .is_paletted();

    /// Predefined frame format for RGB565 packed video.
    pub const RGB565: &Pixel = &Pixel::new(ColorModel::Trichromatic(RGB))
        .add_first_plane(Chromaton::packrgb(5, 11, 0, 2))
        .add_second_plane(Chromaton::packrgb(6, 5, 0, 2))
        .add_third_plane(Chromaton::packrgb(5, 0, 0, 2))
        .size(2);

    /// Predefined frame format for RGB24.
    pub const RGB24: &Pixel = &Pixel::new(ColorModel::Trichromatic(RGB))
        .add_first_plane(Chromaton::packrgb(8, 0, 2, 3))
        .add_second_plane(Chromaton::packrgb(8, 0, 1, 3))
        .add_third_plane(Chromaton::packrgb(8, 0, 0, 3))
        .size(3);

    /// Predefined frame format for RGBA.
    pub const RGBA: &Pixel = &Pixel::new(ColorModel::Trichromatic(RGB))
        .add_first_plane(Chromaton::packrgb(8, 0, 3, 4))
        .add_second_plane(Chromaton::packrgb(8, 0, 2, 4))
        .add_third_plane(Chromaton::packrgb(8, 0, 1, 4))
        .add_fourth_plane(Chromaton::packrgb(8, 0, 0, 4))
        .size(4)
        .has_alpha_plane();

    /// Predefined frame format for RGB48.
    pub const RGB48: &Pixel = &Pixel::new(ColorModel::Trichromatic(RGB))
        .add_first_plane(Chromaton::packrgb(16, 0, 2, 6))
        .add_second_plane(Chromaton::packrgb(16, 0, 1, 6))
        .add_third_plane(Chromaton::packrgb(16, 0, 0, 6))
        .size(6);

    /// Predefined frame format for RGBA64.
    pub const RGBA64: &Pixel = &Pixel::new(ColorModel::Trichromatic(RGB))
        .add_first_plane(Chromaton::packrgb(16, 0, 3, 8))
        .add_second_plane(Chromaton::packrgb(16, 0, 2, 8))
        .add_third_plane(Chromaton::packrgb(16, 0, 1, 8))
        .add_fourth_plane(Chromaton::packrgb(16, 0, 0, 8))
        .size(8)
        .has_alpha_plane();
}

#[cfg(test)]
mod test {

    use super::formats;

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
