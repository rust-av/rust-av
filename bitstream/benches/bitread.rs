use av_bitstream::bitread::BitReadLE;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use av_bitstream::bitread::*;

pub fn bitreader(c: &mut Criterion) {
    let buffer: Vec<u8> = (0..2048).flat_map(|_| 0..128).collect();
    let rbe = BitReadBE::new(&buffer);
    let rle = BitReadLE::new(&buffer);

    bitread(c, rbe, "BE");
    bitread(c, rle, "LE");
}

pub fn bitread<'a>(c: &mut Criterion, r: impl BitRead<'a>, kind: &str) {
    c.bench_function(&format!("{kind} read bits using 32bit output"), |b| {
        b.iter(|| {
            let mut rr = black_box(r);
            for _ in 0..1024 {
                for l in 0..31 {
                    rr.get_bits_32(l);
                }
                rr.get_bit();
                for l in 0..31 {
                    rr.get_bits_32(l);
                }
            }
        })
    });

    c.bench_function(&format!("{kind} read bits using 64bit output"), |b| {
        b.iter(|| {
            let mut rr = black_box(r);
            for _ in 0..1024 {
                for l in 0..31 {
                    rr.get_bits_64(l);
                }
                rr.get_bit();
                for l in 32..63 {
                    rr.get_bits_64(l);
                }
            }
        })
    });

    c.bench_function(&format!("{kind} read bits mixing"), |b| {
        b.iter(|| {
            let mut rr = black_box(r);
            for _ in 0..1024 {
                for l in 0..31 {
                    rr.get_bits_64(l);
                    rr.get_bit();
                    rr.get_bits_32(l);
                }
            }
        })
    });
}

criterion_group!(benches, bitreader);
criterion_main!(benches);
