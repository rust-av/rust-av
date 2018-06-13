// workarounds
#![allow(unused_doc_comments)]

#[macro_use]
extern crate failure;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub mod bitread;
pub mod byteread;
pub mod bytewrite;
pub mod codebook;

#[macro_export]
macro_rules! tagl {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ($a as u32) |
        ($b as u32) << 8 |
        ($c as u32) << 16 |
        ($d as u32) << 24
    }
}

#[macro_export]
macro_rules! tagb {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        tagl!($d, $c, $b, $a)
    }
}


// TODO: write meaningful tests.
#[cfg(test)]
mod test {
    #[test]
    fn tags() {
        assert_eq!(tagl!('a', 'b', 'c', 'd'), 1684234849u32);
        assert_eq!(tagb!('a', 'b', 'c', 'd'), 1633837924u32);
    }
}
