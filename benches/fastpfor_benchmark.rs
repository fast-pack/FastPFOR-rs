//! Benchmark suite for `FastPFOR` compression codecs.

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

#[path = "bench_utils.rs"]
mod bench_utils;
use bench_utils::*;

const SIZES: &[usize] = &[1024, 4096];

fn benchmark_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");
    for (size, fix) in compress_fixtures(SIZES) {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new(fix.name, size), &fix.data, |b, data| {
            let mut codec = FastPFOR::default();
            b.iter(|| black_box(compress_data(&mut codec, black_box(data))));
        });
    }
    group.finish();
}

fn benchmark_decompression(c: &mut Criterion) {
    let mut group = c.benchmark_group("decompression");
    for (size, fix) in compress_fixtures(SIZES) {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new(fix.name, size),
            &fix.rust_compressed,
            |b, compressed| {
                let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
                let mut decompressed = vec![0u32; size];
                b.iter(|| {
                    black_box(decompress_data(
                        &mut codec,
                        black_box(compressed),
                        &mut decompressed,
                    ))
                });
            },
        );
    }
    group.finish();
}

fn benchmark_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");
    for &size in SIZES {
        let data = generate_uniform_data_small_value_distribution(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new("compress_decompress", size),
            &data,
            |b, data| {
                let mut encoder = FastPFOR::default();
                let mut decoder = FastPFOR::default();
                let mut compressed = vec![0u32; data.len() * 2 + 1024];
                let mut decompressed = vec![0u32; data.len()];
                b.iter(|| {
                    let mut input_offset = Cursor::new(0);
                    let mut output_offset = Cursor::new(0);
                    encoder
                        .compress(
                            black_box(data),
                            data.len() as u32,
                            &mut input_offset,
                            &mut compressed,
                            &mut output_offset,
                        )
                        .unwrap();
                    input_offset.set_position(0);
                    let compressed_len = output_offset.position();
                    output_offset.set_position(0);
                    decoder
                        .uncompress(
                            &compressed,
                            data.len() as u32,
                            &mut input_offset,
                            &mut decompressed,
                            &mut output_offset,
                        )
                        .unwrap();
                    black_box((compressed_len, output_offset.position()))
                });
            },
        );
    }
    group.finish();
}

fn benchmark_block_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("block_sizes");
    let size = *SIZES.last().unwrap();
    for fix in block_size_fixtures(size) {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_function(format!("compress_{}", fix.block_size), |b| {
            let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, fix.block_size);
            b.iter(|| black_box(compress_data(&mut codec, black_box(&fix.data))));
        });
        group.bench_function(format!("decompress_{}", fix.block_size), |b| {
            let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, fix.block_size);
            let mut decompressed = vec![0u32; size];
            b.iter(|| {
                black_box(decompress_data(
                    &mut codec,
                    black_box(&fix.compressed),
                    &mut decompressed,
                ))
            });
        });
    }
    group.finish();
}

fn benchmark_compression_ratio(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_ratio");
    group.sample_size(20);
    let size = *SIZES.last().unwrap();
    for fix in ratio_fixtures(size) {
        group.bench_function(fix.name, |b| {
            let mut codec = FastPFOR::default();
            b.iter(|| {
                let compressed = compress_data(&mut codec, black_box(&fix.data));
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "Loss of precision is acceptable for compression ratio calculation"
                )]
                black_box(fix.data.len() as f64 / compressed.len() as f64)
            });
        });
    }
    group.finish();
}

/// Compare encoding and decoding speed of the C++ `FastPFor128` codec against
/// the pure-Rust `FastPFOR` codec with `BLOCK_SIZE_128`.
#[cfg(feature = "cpp")]
fn benchmark_cpp_vs_rust(c: &mut Criterion) {
    use fastpfor::cpp::FastPFor128Codec;

    let mut group = c.benchmark_group("cpp_vs_rust/encode");
    for (size, fix) in compress_fixtures(SIZES) {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new(format!("cpp/{}", fix.name), size),
            &fix.data,
            |b, data| {
                let codec = FastPFor128Codec::new();
                b.iter(|| black_box(cpp_encode(&codec, black_box(data))));
            },
        );
        group.bench_with_input(
            BenchmarkId::new(format!("rust/{}", fix.name), size),
            &fix.data,
            |b, data| {
                let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
                b.iter(|| black_box(compress_data(&mut codec, black_box(data))));
            },
        );
    }
    group.finish();

    let mut group = c.benchmark_group("cpp_vs_rust/decode");
    for (size, fix) in cpp_decode_fixtures(SIZES) {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new(format!("cpp/{}", fix.name), size),
            &fix.cpp_compressed,
            |b, compressed| {
                let codec = FastPFor128Codec::new();
                let mut out = vec![0u32; fix.original_len];
                b.iter(|| black_box(cpp_decode(&codec, black_box(compressed), &mut out)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new(format!("rust/{}", fix.name), size),
            &fix.rust_compressed,
            |b, compressed| {
                let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
                let mut decompressed = vec![0u32; fix.original_len];
                b.iter(|| {
                    black_box(decompress_data(
                        &mut codec,
                        black_box(compressed),
                        &mut decompressed,
                    ))
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_compression,
    benchmark_decompression,
    benchmark_roundtrip,
    benchmark_block_sizes,
    benchmark_compression_ratio,
);

#[cfg(feature = "cpp")]
criterion_group!(cpp_benches, benchmark_cpp_vs_rust);

#[cfg(not(feature = "cpp"))]
criterion_main!(benches);

#[cfg(feature = "cpp")]
criterion_main!(benches, cpp_benches);
