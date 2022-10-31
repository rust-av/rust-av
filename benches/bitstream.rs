use av_bitstream::bitread::*;
use criterion::{Criterion, Throughput};

const TEST_INPUT: [u8; 16] = [0b01010101; 16];

fn bitread(bytes: &[u8]) {
    let mut reader = BitReadLE::new(bytes);

    // Each iteration consumes 64-bits
    while reader.available() > 0 {
        reader.skip_bits(1);
        reader.get_bits_32(1);
        reader.skip_bits(3);
        reader.get_bits_32(3);
        reader.skip_bits(5);
        reader.get_bits_32(5);
        reader.skip_bits(7);
        reader.get_bits_32(7);
        reader.skip_bits(11);
        reader.get_bits_32(11);

        reader.peek_bits_32(10);
        reader.skip_bits(10);
    }
}

pub fn bench_bitstream(c: &mut Criterion) {
    let mut group = c.benchmark_group("av_bitstream");
    group.throughput(Throughput::Bytes(TEST_INPUT.len() as u64));
    group.bench_function("BitRead", |b| b.iter(|| bitread(&TEST_INPUT)));
    group.finish();
}
