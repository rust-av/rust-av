pub trait BitRead {
    fn get_bit1(&mut self) -> bool;
/*
    fn peek_bit1(size : u8) -> bool;

    fn get_bits32(size : u8) -> u32;

    fn get_bits64(size : u8) -> u64;

    fn peek_bits32(size : u8) -> u32;

    fn peek_bits64(size : u8) -> u64;

    fn skip_bits(size : u8) -> ();
*/
}

pub struct BitReadLE <'a> {
    buffer : &'a[u8],
    index : usize,
    cache : u64,
    left : usize,
}

impl <'a> BitRead for BitReadLE<'a> {
    fn get_bit1(&mut self) -> bool {
        let val = (self.cache & 1) == 1;
        self.cache = self.cache >> 1;
        return val;
    }
}


#[cfg(test)]
mod test {
    use super::*;


#[test]
    fn get_bit1() {
        let mut reader = BitReadLE {
            buffer : &b""[..],
            index : 0,
            cache: 0b10,
            left: 2
        };

        assert!(!reader.get_bit1());
        assert!(reader.get_bit1());
    }

}
