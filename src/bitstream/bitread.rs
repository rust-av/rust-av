use bitstream::byteread::*;

pub trait BitRead {
    fn consumed(&self) -> usize;
    fn available(&self) -> usize;
    fn can_refill(&self) -> bool;
    fn refill64(&mut self) -> ();
    fn get_val(&mut self, n:usize) -> u64;
    fn get_bit(&mut self) -> bool;
    fn peek_val(&mut self, n:usize) -> u64;
    fn get_bits_64(&mut self, n: usize) -> u64;
    fn refill32(&mut self) -> ();
    fn get_bits_32(&mut self, n: usize) -> u32;

    fn peek_bits_64(&mut self, n: usize) -> u64;

    fn peek_bits_32(&mut self, n: usize) -> u32;

    fn skip_rem(&mut self, n:usize) -> ();
    fn skip_bits(&mut self, size : usize) -> ();
    fn align_bits(&mut self) -> ();
}

#[derive(Debug, Clone, Copy)]
pub struct BitReadLE <'a> {
    buffer : &'a[u8], /// read buffer, 8-bytes padded
    index : usize,
    cache : u64,
    left : usize,
}

impl <'a> BitRead for BitReadLE<'a> {
    #[inline]
    fn consumed(&self) -> usize {
        self.index * 8 - self.left
    }

    #[inline]
    fn available(&self) -> usize {
        (self.buffer.len() - self.index) * 8 + self.left
    }

    #[inline]
    fn can_refill(&self) -> bool {
        self.index < self.buffer.len() - 8
    }

    #[inline]
    fn refill64(&mut self) -> () {
        if !self.can_refill() {
            return;
        }

        self.cache  = get_u64l(&self.buffer[self.index..]);
        self.index += 8;
        self.left   = 64;
    }

    #[inline]
    fn get_val(&mut self, n:usize) -> u64 {
        let ret = self.peek_val(n);

        self.cache = self.cache >> n;
        self.left -= n;

        return ret;
    }

    #[inline]
    fn peek_val(&mut self, n:usize) -> u64 {
        self.cache & ((1u64 << n) - 1)
    }

    #[inline]
    fn get_bit(&mut self) -> bool {
        if self.left <= 0 {
            self.refill64();
        }

        self.get_val(1) != 0
    }

    #[inline]
    fn get_bits_64(&mut self, n:usize) -> u64 {
        let mut left = 0;
        let mut ret = 0;

        if n == 0 {
            return 0;
        }

        if self.left < n {
            left = self.left;
            ret  = self.get_val(left);
            self.refill64();
        }

        self.get_val(n - left) << left | ret
    }

    #[inline]
    fn peek_bits_64(&mut self, n:usize) -> u64 {
        let mut tmp = self.clone();

        tmp.get_bits_64(n)
    }

    #[inline]
    fn refill32(&mut self) -> () {
        if !self.can_refill() {
            return;
        }
        let val = get_u32l(&self.buffer[self.index..]) as u64;

        self.cache  = val << self.left | self.cache;
        self.index += 4;
        self.left  += 32;
    }

    #[inline]
    fn get_bits_32(&mut self, n:usize) -> u32 {
        if n == 0 {
            return 0;
        }

        if self.left <= n {
            self.refill32();
        }

        return self.get_val(n) as u32;
    }

    #[inline]
    fn peek_bits_32(&mut self, n:usize) -> u32 {
        if n == 0 {
            return 0;
        }

        if self.left <= n {
            self.refill32();
        }

        return self.peek_val(n) as u32;
    }

    fn skip_rem(&mut self, n:usize) -> () {
        self.cache = self.cache >> n;
        self.left -= n;
    }

    #[inline]
    fn skip_bits(&mut self, mut n:usize) -> () {
        if self.left < n {
            n -= self.left;
            self.skip_rem(n);
            if n > 64 {
                let skip = n / 8;

                n -= skip * 8;
                self.index += skip;
            }
            self.refill64();
        }

        self.skip_rem(n);
    }

    #[inline]
    fn align_bits(&mut self) -> () {
        let left = self.left;

        self.skip_bits(left);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const CHECKBOARD0101: [u8; 128] = [0b01010101; 128];
    const CHECKBOARD0011: [u8; 128] = [0b00110011; 128];

#[test]
    fn get_bit() {
        let mut reader = BitReadLE {
            buffer: &CHECKBOARD0101,
            index: 0,
            cache: 0,
            left: 0
        };

        assert!(reader.get_bit());
        assert!(!reader.get_bit());
    }

#[test]
    fn get_bits_64() {
        let mut reader = BitReadLE {
            buffer: &CHECKBOARD0101,
            index: 0,
            cache: 0,
            left: 0
        };

        assert!(reader.get_bits_64(1) == 1);
        assert!(reader.get_bits_64(2) == 2);
        assert!(reader.get_bits_64(4) == 10);
        assert!(reader.get_bits_64(1) == 0);
        assert!(reader.get_bits_64(8) == 85);
    }

#[test]
    fn peek_bits_64() {
        let mut reader = BitReadLE {
            buffer: &CHECKBOARD0101,
            index: 0,
            cache: 0,
            left: 0
        };

        assert!(reader.peek_bits_64(1) == 1);
        assert!(reader.peek_bits_64(1) == 1);
        assert!(reader.peek_bits_64(2) == 1);
        assert!(reader.peek_bits_64(2) == 1);
    }
#[test]
    fn get_bits_32() {
        let mut reader = BitReadLE {
            buffer: &CHECKBOARD0101,
            index: 0,
            cache: 0,
            left: 0
        };

        assert!(reader.get_bits_64(1) == 1);
        assert!(reader.get_bits_64(2) == 2);
        assert!(reader.get_bits_64(4) == 10);
        assert!(reader.get_bits_64(1) == 0);
        assert!(reader.get_bits_64(8) == 85);
    }

#[test]
    fn peek_bits_32() {
        let mut reader = BitReadLE {
            buffer: &CHECKBOARD0101,
            index: 0,
            cache: 0,
            left: 0
        };

        assert!(reader.peek_bits_32(1) == 1);
        assert!(reader.peek_bits_32(1) == 1);
        assert!(reader.peek_bits_32(2) == 1);
        assert!(reader.peek_bits_32(2) == 1);
    }
#[test]
    fn skip_bits() {
        let mut reader = BitReadLE {
            buffer: &CHECKBOARD0101,
            index: 0,
            cache: 0,
            left: 0
        };

        reader.skip_bits(0);
        assert!(reader.peek_bits_32(1) == 1);
        reader.skip_bits(2);
        assert!(reader.peek_bits_32(1) == 1);
        reader.skip_bits(2);
        assert!(reader.peek_bits_32(1) == 1);
    }
#[test]
    fn align_bits() {
        let mut reader = BitReadLE {
            buffer: &CHECKBOARD0011,
            index: 0,
            cache: 0,
            left: 0
        };

        assert!(reader.get_bits_64(3) == 3);
        reader.align_bits();
        assert!(reader.get_bits_64(4) == 3);
        reader.skip_bits(1);
        reader.align_bits();
        assert!(reader.get_bits_64(4) == 3);
    }
}
