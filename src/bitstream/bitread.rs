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
/*
    fn peek_bit1(size : u8) -> bool;

    fn get_bits32(size : u8) -> u32;


    fn peek_bits32(size : u8) -> u32;

    fn peek_bits64(size : u8) -> u64;

    fn skip_bits(size : u8) -> ();
*/
}

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
}


#[cfg(test)]
mod test {
    use super::*;

    const CHECKBOARD: [u8; 128] = [0b01010101; 128];

#[test]
    fn get_bit() {
        let mut reader = BitReadLE {
            buffer: &CHECKBOARD,
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
            buffer: &CHECKBOARD,
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
}
