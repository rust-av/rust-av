use crate::bitread::*;
use std::collections::HashMap;
use std::cmp::{min, max};

#[derive(Fail, Debug)]
pub enum CodebookError {
    #[fail(display = "Invalid Codebook")]
    InvalidCodebook,
    #[fail(display = "Invalid Code")]
    InvalidCode
}

use self::CodebookError::*;

#[derive(Debug, Copy, Clone)]
pub enum CodebookMode {
    MSB,
    LSB,
}

pub struct FullCodebookDesc<S> {
    code: u32,
    bits: u8,
    sym: S,
}

pub struct ShortCodebookDesc {
    code: u32,
    bits: u8,
}

pub trait CodebookDescReader<S> {
    fn bits(&mut self, idx: usize) -> u8;
    fn code(&mut self, idx: usize) -> u32;
    fn sym(&mut self, idx: usize) -> S;
    fn len(&mut self) -> usize;
}

#[allow(dead_code)]
pub struct Codebook<S> {
    table: Vec<u32>,
    syms: Vec<S>,
    lut_bits: u8,
}

pub trait CodebookReader<S> {
    fn read_cb(&mut self, cb: &Codebook<S>) -> Result<S, CodebookError>;
}

pub fn reverse_bits(inval: u32) -> u32 {
    const REV_TAB: [u8; 16] = [0b0000, 0b1000, 0b0100, 0b1100, 0b0010, 0b1010, 0b0110, 0b1110,
                               0b0001, 0b1001, 0b0101, 0b1101, 0b0011, 0b1011, 0b0111, 0b1111];

    let mut ret = 0;
    let mut val = inval;
    for _ in 0..8 {
        ret = (ret << 4) | (REV_TAB[(val & 0xF) as usize] as u32);
        val = val >> 4;
    }
    ret
}

const TABLE_FILL_VALUE: u32 = 0x7F;
const MAX_LUT_BITS: u8 = 10;

fn fill_lut_msb(table: &mut Vec<u32>,
                off: usize,
                code: u32,
                bits: u8,
                lut_bits: u8,
                symidx: u32,
                esc: bool) {
    if !esc {
        let fill_len = lut_bits - bits;
        let fill_size = 1 << fill_len;
        let fill_code = code << (lut_bits - bits);
        let lut_value = (symidx << 8) | (bits as u32);
        for j in 0..fill_size {
            let idx = (fill_code + j) as usize;
            table[idx + off] = lut_value;
        }
    } else {
        let idx = (code as usize) + off;
        table[idx] = (symidx << 8) | 0x80 | (bits as u32);
    }
}

fn fill_lut_lsb(table: &mut Vec<u32>,
                off: usize,
                code: u32,
                bits: u8,
                lut_bits: u8,
                symidx: u32,
                esc: bool) {
    if !esc {
        let fill_len = lut_bits - bits;
        let fill_size = 1 << fill_len;
        let fill_code = code;
        let step = lut_bits - fill_len;
        for j in 0..fill_size {
            let idx = (fill_code + (j << step)) as usize;
            table[idx + off] = (symidx << 8) | (bits as u32);
        }
    } else {
        let idx = (code as usize) + off;
        table[idx] = (symidx << 8) | 0x80 | (bits as u32);
    }
}

fn fill_lut(table: &mut Vec<u32>,
            mode: CodebookMode,
            off: usize,
            code: u32,
            bits: u8,
            lut_bits: u8,
            symidx: u32,
            esc: bool)
            -> bool {
    match mode {
        CodebookMode::MSB => fill_lut_msb(table, off, code, bits, lut_bits, symidx, esc),
        CodebookMode::LSB => fill_lut_lsb(table, off, code, bits, lut_bits, symidx, esc),
    };
    bits > lut_bits
}

fn resize_table(table: &mut Vec<u32>, bits: u8) -> u32 {
    let add_size = (1 << bits) as usize;
    table.reserve(add_size);
    let cur_off = table.len() as u32;
    let new_size = table.len() + add_size;
    table.resize(new_size, TABLE_FILL_VALUE);
    cur_off
}


fn extract_lut_part(code: u32, bits: u8, lut_bits: u8, mode: CodebookMode) -> u32 {
    match mode {
        CodebookMode::MSB => code >> (bits - lut_bits),
        CodebookMode::LSB => code & ((1 << lut_bits) - 1),
    }
}

fn extract_esc_part(code: u32, bits: u8, lut_bits: u8, mode: CodebookMode) -> u32 {
    match mode {
        CodebookMode::MSB => code & ((1 << (bits - lut_bits)) - 1),
        CodebookMode::LSB => code >> lut_bits,
    }
}

#[derive(Clone,Copy)]
struct Code {
    code: u32,
    bits: u8,
    idx: usize,
}

struct CodeBucket {
    maxlen: u8,
    offset: usize,
    codes: Vec<Code>,
}

impl CodeBucket {
    fn new() -> Self {
        CodeBucket {
            maxlen: 0,
            offset: 0,
            codes: Vec::new(),
        }
    }
    fn add_code(&mut self, c: Code) {
        if c.bits > self.maxlen {
            self.maxlen = c.bits;
        }
        self.codes.push(c);
    }
}

type EscapeCodes = HashMap<u32, CodeBucket>;

fn add_esc_code(cc: &mut EscapeCodes, key: u32, code: u32, bits: u8, idx: usize) {
    if !cc.contains_key(&key) {
        cc.insert(key, CodeBucket::new());
    }
    let b = cc.get_mut(&key);
    if let Some(bucket) = b {
        bucket.add_code(Code {
            code: code,
            bits: bits,
            idx: idx,
        });
    } else {
        panic!("no bucket when expected!");
    }
}

fn build_esc_lut(table: &mut Vec<u32>, mode: CodebookMode, bucket: &CodeBucket) -> Result<(), CodebookError> {
    let mut escape_list: EscapeCodes = HashMap::new();
    let maxlen = min(bucket.maxlen, MAX_LUT_BITS);

    for code in &bucket.codes {
        let bits = code.bits;
        if code.bits <= MAX_LUT_BITS {
            fill_lut(table,
                     mode,
                     bucket.offset,
                     code.code,
                     bits,
                     maxlen,
                     code.idx as u32,
                     false);
        } else {
            let ckey = extract_lut_part(code.code, bits, MAX_LUT_BITS, mode);
            let cval = extract_esc_part(code.code, bits, MAX_LUT_BITS, mode);
            add_esc_code(&mut escape_list, ckey, cval, bits - MAX_LUT_BITS, code.idx);
        }
    }

    let cur_offset = bucket.offset;
    for (ckey, sec_bucket) in &mut escape_list {
        let key = *ckey as u32;
        let maxlen = min(sec_bucket.maxlen, MAX_LUT_BITS);
        let new_off = resize_table(table, maxlen);
        fill_lut(table,
                 mode,
                 cur_offset,
                 key,
                 maxlen,
                 MAX_LUT_BITS,
                 new_off,
                 true);
        sec_bucket.offset = new_off as usize;
    }

    for (_, sec_bucket) in &escape_list {
        build_esc_lut(table, mode, sec_bucket)?;
    }

    Ok(())
}

impl<S: Copy> Codebook<S> {
    pub fn new(cb: &mut dyn CodebookDescReader<S>, mode: CodebookMode) -> Result<Self, CodebookError> {
        let mut maxbits = 0;
        let mut nnz = 0;
        let mut escape_list: EscapeCodes = HashMap::new();

        let mut symidx: usize = 0;
        for i in 0..cb.len() {
            let bits = cb.bits(i);
            if bits > 0 {
                nnz = nnz + 1;
            }
            maxbits = max(bits, maxbits);
            if bits > MAX_LUT_BITS {
                let code = cb.code(i);
                let ckey = extract_lut_part(code, bits, MAX_LUT_BITS, mode);
                let cval = extract_esc_part(code, bits, MAX_LUT_BITS, mode);
                add_esc_code(&mut escape_list, ckey, cval, bits - MAX_LUT_BITS, symidx);
            }
            if bits > 0 {
                symidx = symidx + 1;
            }
        }
        if maxbits == 0 {
            return Err(InvalidCodebook);
        }

        if maxbits > MAX_LUT_BITS {
            maxbits = MAX_LUT_BITS;
        }

        let tab_len = 1 << maxbits;
        let mut table: Vec<u32> = Vec::with_capacity(tab_len);
        let mut syms: Vec<S> = Vec::with_capacity(nnz);
        table.resize(tab_len, TABLE_FILL_VALUE);

        let mut symidx: u32 = 0;
        for i in 0..cb.len() {
            let bits = cb.bits(i);
            let code = cb.code(i);
            if bits == 0 {
                continue;
            }
            if bits <= MAX_LUT_BITS {
                fill_lut(&mut table, mode, 0, code, bits, maxbits, symidx, false);
            } else {
                let ckey = extract_lut_part(code, bits, MAX_LUT_BITS, mode) as usize;
                if table[ckey] == TABLE_FILL_VALUE {
                    let key = ckey as u32;
                    if let Some(bucket) = escape_list.get_mut(&key) {
                        let maxlen = min(bucket.maxlen, MAX_LUT_BITS);
                        let new_off = resize_table(&mut table, maxlen);
                        fill_lut(&mut table,
                                 mode,
                                 0,
                                 key,
                                 maxlen,
                                 MAX_LUT_BITS,
                                 new_off,
                                 true);
                        bucket.offset = new_off as usize;
                    }
                }
            }
            symidx = symidx + 1;
        }

        for (_, bucket) in &escape_list {
            build_esc_lut(&mut table, mode, &bucket)?;
        }

        for i in 0..cb.len() {
            if cb.bits(i) > 0 {
                syms.push(cb.sym(i));
            }
        }

        Ok(Codebook {
            table: table,
            syms: syms,
            lut_bits: maxbits,
        })
    }
}

impl<'a, S: Copy, B: BitRead<'a>> CodebookReader<S> for B {
    fn read_cb(&mut self, cb: &Codebook<S>) -> Result<S, CodebookError> {
        let mut esc = true;
        let mut idx = 0;
        let mut lut_bits = cb.lut_bits;
        while esc {
            let lut_idx = (self.peek_bits_32(lut_bits as usize) as usize) + (idx as usize);
            if cb.table[lut_idx] == TABLE_FILL_VALUE {

                return Err(InvalidCode);
            }
            let bits = cb.table[lut_idx] & 0x7F;
            esc = (cb.table[lut_idx] & 0x80) != 0;
            idx = (cb.table[lut_idx] >> 8) as usize;
            if bits > self.left() as u32 {
                return Err(InvalidCode);
            }
            let skip_bits = if esc {
                lut_bits as usize
            } else {
                bits as usize
            };
            self.skip_bits(skip_bits);
            lut_bits = bits as u8;
        }
        Ok(cb.syms[idx])
    }
}

impl<S: Copy> CodebookDescReader<S> for Vec<FullCodebookDesc<S>> {
    fn bits(&mut self, idx: usize) -> u8 {
        self[idx].bits
    }
    fn code(&mut self, idx: usize) -> u32 {
        self[idx].code
    }
    fn sym(&mut self, idx: usize) -> S {
        self[idx].sym
    }
    fn len(&mut self) -> usize {
        Vec::len(self)
    }
}

impl CodebookDescReader<u32> for Vec<ShortCodebookDesc> {
    fn bits(&mut self, idx: usize) -> u8 {
        self[idx].bits
    }
    fn code(&mut self, idx: usize) -> u32 {
        self[idx].code
    }
    fn sym(&mut self, idx: usize) -> u32 {
        idx as u32
    }
    fn len(&mut self) -> usize {
        Vec::len(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const BITS: [u8; 8] = [0b01011011, 0b10111100, 0b11111111, 0, 0, 0, 0, 0];

    #[test]
    fn test_full_codebook_msb() {
        let mut cb_desc: Vec<FullCodebookDesc<i8>> = vec![FullCodebookDesc {
                                                              code: 0b0,
                                                              bits: 1,
                                                              sym: 16,
                                                          },
                                                          FullCodebookDesc {
                                                              code: 0b10,
                                                              bits: 2,
                                                              sym: -3,
                                                          },
                                                          FullCodebookDesc {
                                                              code: 0b110,
                                                              bits: 3,
                                                              sym: 42,
                                                          },
                                                          FullCodebookDesc {
                                                              code: 0b1110,
                                                              bits: 4,
                                                              sym: -42,
                                                          }];
        let buf = &BITS;
        let mut br = BitReadBE::new(buf);
        let cb = Codebook::new(&mut cb_desc, CodebookMode::MSB).unwrap();

        assert_eq!(br.read_cb(&cb).unwrap(), 16);
        assert_eq!(br.read_cb(&cb).unwrap(), -3);
        assert_eq!(br.read_cb(&cb).unwrap(), 42);
        assert_eq!(br.read_cb(&cb).unwrap(), -42);
        let ret = br.read_cb(&cb);
        if let Err(e) = ret {
            assert_matches!(e, InvalidCode);
        } else {
            assert_eq!(0, 1);
        }
    }

    #[test]
    fn test_short_codebook_msb() {
        let mut scb_desc: Vec<ShortCodebookDesc> = vec![ShortCodebookDesc {
                                                            code: 0b0,
                                                            bits: 1,
                                                        },
                                                        ShortCodebookDesc { code: 0, bits: 0 },
                                                        ShortCodebookDesc {
                                                            code: 0b10,
                                                            bits: 2,
                                                        },
                                                        ShortCodebookDesc { code: 0, bits: 0 },
                                                        ShortCodebookDesc { code: 0, bits: 0 },
                                                        ShortCodebookDesc {
                                                            code: 0b110,
                                                            bits: 3,
                                                        },
                                                        ShortCodebookDesc { code: 0, bits: 0 },
                                                        ShortCodebookDesc {
                                                            code: 0b11100,
                                                            bits: 5,
                                                        },
                                                        ShortCodebookDesc {
                                                            code: 0b11101,
                                                            bits: 5,
                                                        },
                                                        ShortCodebookDesc {
                                                            code: 0b1111010,
                                                            bits: 7,
                                                        },
                                                        ShortCodebookDesc {
                                                            code: 0b1111011,
                                                            bits: 7,
                                                        },
                                                        ShortCodebookDesc {
                                                            code: 0b1111110,
                                                            bits: 7,
                                                        },
                                                        ShortCodebookDesc {
                                                            code: 0b11111111,
                                                            bits: 8,
                                                        }];
        let buf = &BITS;
        let mut br2 = BitReadBE::new(buf);
        let cb = Codebook::new(&mut scb_desc, CodebookMode::MSB).unwrap();
        assert_eq!(br2.read_cb(&cb).unwrap(), 0);
        assert_eq!(br2.read_cb(&cb).unwrap(), 2);
        assert_eq!(br2.read_cb(&cb).unwrap(), 5);
        assert_eq!(br2.read_cb(&cb).unwrap(), 8);

        assert_eq!(reverse_bits(0b0000_0101_1011_1011_1101_1111_0111_1111),
                   0b1111_1110_1111_1011_1101_1101_1010_0000);
    }

    #[test]
    fn test_short_codebook_lsb() {
        const BITS_LE: [u8; 8] = [0b11101111, 0b01110010, 0b01, 0, 0, 0, 0, 0];
        let buf = &BITS_LE;
        let mut scble_desc: Vec<ShortCodebookDesc> = vec![ShortCodebookDesc {
                                                              code: 0b00,
                                                              bits: 2,
                                                          },
                                                          ShortCodebookDesc { code: 0, bits: 0 },
                                                          ShortCodebookDesc {
                                                              code: 0b01,
                                                              bits: 2,
                                                          },
                                                          ShortCodebookDesc { code: 0, bits: 0 },
                                                          ShortCodebookDesc { code: 0, bits: 0 },
                                                          ShortCodebookDesc {
                                                              code: 0b011,
                                                              bits: 3,
                                                          },
                                                          ShortCodebookDesc { code: 0, bits: 0 },
                                                          ShortCodebookDesc {
                                                              code: 0b10111,
                                                              bits: 5,
                                                          },
                                                          ShortCodebookDesc {
                                                              code: 0b00111,
                                                              bits: 5,
                                                          },
                                                          ShortCodebookDesc {
                                                              code: 0b0101111,
                                                              bits: 7,
                                                          },
                                                          ShortCodebookDesc {
                                                              code: 0b0111111,
                                                              bits: 7,
                                                          },
                                                          ShortCodebookDesc {
                                                              code: 0b1011101111,
                                                              bits: 10,
                                                          }];
        let mut brl = BitReadLE::new(buf);
        let cb = Codebook::new(&mut scble_desc, CodebookMode::LSB).unwrap();
        assert_eq!(brl.read_cb(&cb).unwrap(), 11);
        assert_eq!(brl.read_cb(&cb).unwrap(), 0);
        assert_eq!(brl.read_cb(&cb).unwrap(), 7);
        assert_eq!(brl.read_cb(&cb).unwrap(), 0);
    }
}
