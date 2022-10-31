use std::io::Cursor;
use std::io::Read;

use av_format::buffer::AccReader;
use criterion::{Criterion, Throughput};

const TEST_INPUT: [u8; 16] = [0b01010101; 16];

fn bench_accreader(bytes: &[u8]) {
    let cursor = Cursor::new(&bytes[..]);
    let mut reader = AccReader::with_capacity(5, cursor);
    let mut read_buffer = [0];

    for _ in 0..bytes.len() {
        reader.read_exact(&mut read_buffer).unwrap();
    }
}

pub fn bench_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("av_format");
    group.throughput(Throughput::Bytes(TEST_INPUT.len() as u64));
    group.bench_function("AccReader", |b| b.iter(|| bench_accreader(&TEST_INPUT)));
    group.finish();
}
