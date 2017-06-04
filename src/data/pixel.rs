use std::string::*;
use std::slice;
use std::fmt;
use std::ops::Index;

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum RGBSubmodel {
    RGB,
    SRGB,
}

impl fmt::Display for RGBSubmodel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            RGBSubmodel::RGB => "RGB".to_string(),
            RGBSubmodel::SRGB => "sRGB".to_string(),
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum YUVSubmodel {
    YCbCr,
    YIQ,
    YUVJ,
}

impl fmt::Display for YUVSubmodel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            YUVSubmodel::YCbCr => "YCbCr".to_string(),
            YUVSubmodel::YIQ => "YIQ".to_string(),
            YUVSubmodel::YUVJ => "YUVJ".to_string(),
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone,Copy,PartialEq)]
pub enum ColorModel {
    RGB(RGBSubmodel),
    YUV(YUVSubmodel),
    CMYK,
    HSV,
    LAB,
    XYZ,
}

impl ColorModel {
    pub fn get_default_components(&self) -> usize {
        match *self {
            ColorModel::CMYK => 4,
            _ => 3,
        }
    }
}

impl fmt::Display for ColorModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            ColorModel::RGB(fmt) => format!("RGB({})", fmt).to_string(),
            ColorModel::YUV(fmt) => format!("YUV({})", fmt).to_string(),
            ColorModel::CMYK => "CMYK".to_string(),
            ColorModel::HSV => "HSV".to_string(),
            ColorModel::LAB => "LAB".to_string(),
            ColorModel::XYZ => "XYZ".to_string(),
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone,Copy,PartialEq)]
pub struct Chromaton {
    h_ss: u8,
    v_ss: u8,
    packed: bool,
    depth: u8,
    shift: u8,
    comp_offs: u8,
    next_elem: u8,
}

fn align(v: usize, a: usize) -> usize {
    (v + a - 1) & !(a - 1)
}

impl Chromaton {
    pub fn get_subsampling(&self) -> (u8, u8) {
        (self.h_ss, self.v_ss)
    }
    pub fn is_packed(&self) -> bool {
        self.packed
    }
    pub fn get_depth(&self) -> u8 {
        self.depth
    }
    pub fn get_shift(&self) -> u8 {
        self.shift
    }
    pub fn get_offset(&self) -> u8 {
        self.comp_offs
    }
    pub fn get_step(&self) -> u8 {
        self.next_elem
    }

    pub fn get_width(&self, width: usize) -> usize {
        (width + ((1 << self.h_ss) - 1)) >> self.h_ss
    }
    pub fn get_height(&self, height: usize) -> usize {
        (height + ((1 << self.v_ss) - 1)) >> self.v_ss
    }
    pub fn get_linesize(&self, width: usize, alignment: usize) -> usize {
        let d = self.depth as usize;
        align((self.get_width(width) * d + d - 1) >> 3, alignment)
    }
    pub fn get_data_size(&self, width: usize, height: usize, align: usize) -> usize {
        let nh = (height + ((1 << self.v_ss) - 1)) >> self.v_ss;
        self.get_linesize(width, align) * nh
    }
}

impl fmt::Display for Chromaton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pfmt = if self.packed {
            let mask = ((1 << self.depth) - 1) << self.shift;
            format!("packed(+{},{:X}, step {})",
                    self.comp_offs,
                    mask,
                    self.next_elem)
        } else {
            format!("planar({},{})", self.comp_offs, self.next_elem)
        };
        write!(f, "({}x{}, {})", self.h_ss, self.v_ss, pfmt)
    }
}

#[derive(Clone,Copy,PartialEq)]
pub struct Formaton {
    model: ColorModel,
    components: u8,
    comp_info: [Option<Chromaton>; 5],
    elem_size: u8,
    be: bool,
    alpha: bool,
    palette: bool,
}

bitflags! {
    pub flags Flags: u8 {
        const BE       = 0x01,
        const ALPHA    = 0x02,
        const PALETTE  = 0x04,
    }
}

impl Formaton {
    pub fn new(model: ColorModel, components: &[Chromaton], flags: Flags, elem_size: u8) -> Self {
        let be = flags.contains(BE);
        let alpha = flags.contains(ALPHA);
        let palette = flags.contains(PALETTE);
        let mut c: [Option<Chromaton>; 5] = [None; 5];

        if components.len() > 5 {
            panic!("too many components");
        }

        for (i, v) in components.iter().enumerate() {
            c[i] = Some(*v);
        }

        Formaton {
            model: model,
            components: components.len() as u8,
            comp_info: c,
            elem_size: elem_size,
            be: be,
            alpha: alpha,
            palette: palette,
        }
    }

    pub fn get_model(&self) -> ColorModel {
        self.model
    }
    pub fn get_num_comp(&self) -> usize {
        self.components as usize
    }
    pub fn get_chromaton(&self, idx: usize) -> Option<Chromaton> {
        if idx < self.comp_info.len() {
            return self.comp_info[idx];
        }
        None
    }
    pub fn is_be(&self) -> bool {
        self.be
    }
    pub fn has_alpha(&self) -> bool {
        self.alpha
    }
    pub fn is_paletted(&self) -> bool {
        self.palette
    }
    pub fn get_elem_size(&self) -> u8 {
        self.elem_size
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let end = if self.be {
            "BE"
        } else {
            "LE"
        };
        let palstr = if self.palette {
            "palette "
        } else {
            ""
        };
        let astr = if self.alpha {
            "alpha "
        } else {
            ""
        };
        let mut str = format!("Formaton for {} ({}{}elem {} size {}): ",
                              self.model,
                              palstr,
                              astr,
                              end,
                              self.elem_size);
        for &i in self.into_iter() {
            if let Some(chr) = i {
                str = format!("{} {}", str, chr);
            }
        }
        write!(f, "[{}]", str)
    }
}

macro_rules! chromaton {
    ($hs: expr, $vs: expr, $pck: expr, $d: expr, $sh: expr, $co: expr, $ne: expr) => ({
        Some(Chromaton {
                h_ss: $hs,
                v_ss: $vs,
                packed: $pck,
                depth: $d,
                shift: $sh,
                comp_offs: $co,
                next_elem: $ne })
    });
    (yuv8; $hs: expr, $vs: expr, $co: expr) => ({
        chromaton!($hs, $vs, false, 8, 0, $co, 1)
    });
    (packrgb; $d: expr, $s: expr, $co: expr, $ne: expr) => ({
        chromaton!(0, 0, true, $d, $s, $co, $ne)
    });
    (pal8; $co: expr) => ({
        chromaton!(0, 0, true, 8, 0, $co, 3)
    });
}

pub mod formats {
    use data::pixel::*;

    pub const YUV420: Formaton = Formaton {
        model: ColorModel::YUV(YUVSubmodel::YUVJ),
        components: 3,
        comp_info: [chromaton!(0, 0, false, 8, 0, 0, 1),
                    chromaton!(yuv8; 1, 1, 1),
                    chromaton!(yuv8; 1, 1, 2),
                    None,
                    None],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    pub const YUV410: Formaton = Formaton {
        model: ColorModel::YUV(YUVSubmodel::YUVJ),
        components: 3,
        comp_info: [chromaton!(0, 0, false, 8, 0, 0, 1),
                    chromaton!(yuv8; 2, 2, 1),
                    chromaton!(yuv8; 2, 2, 2),
                    None,
                    None],
        elem_size: 0,
        be: false,
        alpha: false,
        palette: false,
    };

    pub const PAL8: Formaton = Formaton {
        model: ColorModel::RGB(RGBSubmodel::RGB),
        components: 3,
        comp_info: [chromaton!(pal8; 0), chromaton!(pal8; 1), chromaton!(pal8; 2), None, None],
        elem_size: 3,
        be: false,
        alpha: false,
        palette: true,
    };

    pub const RGB565: Formaton = Formaton {
        model: ColorModel::RGB(RGBSubmodel::RGB),
        components: 3,
        comp_info: [chromaton!(packrgb; 5, 11, 0, 2),
                    chromaton!(packrgb; 6,  5, 0, 2),
                    chromaton!(packrgb; 5,  0, 0, 2),
                    None,
                    None],
        elem_size: 2,
        be: false,
        alpha: false,
        palette: false,
    };

    pub const RGB24: Formaton = Formaton {
        model: ColorModel::RGB(RGBSubmodel::RGB),
        components: 3,
        comp_info: [chromaton!(packrgb; 8, 0, 2, 3),
                    chromaton!(packrgb; 8, 0, 1, 3),
                    chromaton!(packrgb; 8, 0, 0, 3),
                    None,
                    None],
        elem_size: 3,
        be: false,
        alpha: false,
        palette: false,
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fmt() {
        println!("formaton yuv- {}", formats::YUV420);
        println!("formaton pal- {}", formats::PAL8);
        println!("formaton rgb565- {}", formats::RGB565);
    }
}
