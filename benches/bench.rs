use criterion::{criterion_group, criterion_main};

mod bitstream;
mod format;

criterion_group!(benches, format::bench_format, bitstream::bench_bitstream);
criterion_main!(benches);
