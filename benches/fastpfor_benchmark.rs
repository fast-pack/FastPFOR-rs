//! Benchmark suite for `FastPFOR` compression codecs.

use core::ops::Range;
use std::hint::black_box;
use std::io::Cursor;
use std::num::NonZeroU32;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use fastpfor::rust::{FastPFOR, Integer, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng};

const SIZES: &[usize; 2] = &[1024, 4096];
const SEED: u64 = 456;

type DataGeneratorFn = fn(usize) -> Vec<u32>;

/// Generate uniformly distributed random data
fn generate_uniform_data_from_range(size: usize, value_range: Range<u32>) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(SEED);
    (0..size)
        .map(|_| rng.random_range(value_range.clone()))
        .collect()
}

fn generate_uniform_data_small_value_distribution(size: usize) -> Vec<u32> {
    generate_uniform_data_from_range(size, 0..1000)
}

fn generate_uniform_data_large_value_distribution(size: usize) -> Vec<u32> {
    generate_uniform_data_from_range(size, 0..u32::MAX)
}

/// Generate clustered data - values tend to cluster around changing base values
fn generate_clustered_data(size: usize) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut data = Vec::with_capacity(size);
    let mut base = 0u32;

    for _ in 0..size {
        // 10% chance to jump to a new cluster
        if rng.random_bool(0.1) {
            base = rng.random_range(0..1000);
        }
        data.push(base + rng.random_range(0..10));
    }
    data
}

/// Generate sequential/sorted data - ideal for compression
fn generate_sequential_data(size: usize) -> Vec<u32> {
    (0..size as u32).collect()
}

/// Generate sparse data - mostly zeros with occasional random values
fn generate_sparse_data(size: usize) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(SEED);
    (0..size)
        .map(|_| {
            if rng.random_bool(0.9) {
                0
            } else {
                rng.random()
            }
        })
        .collect()
}

/// Generate constant data - best case for compression
fn generate_constant_data(size: usize) -> Vec<u32> {
    vec![SEED as u32; size]
}

/// Generate data with powers of two
fn generate_geometric_data(size: usize) -> Vec<u32> {
    (0..size).map(|i| 1u32 << (i % 30)).collect()
}

/// Helper function to compress data and return the compressed size
fn compress_data(codec: &mut FastPFOR, data: &[u32]) -> usize {
    let mut compressed = vec![0u32; data.len() * 2];
    let mut input_offset = Cursor::new(0);
    let mut output_offset = Cursor::new(0);

    codec
        .compress(
            data,
            data.len() as u32,
            &mut input_offset,
            &mut compressed,
            &mut output_offset,
        )
        .unwrap();

    output_offset.position() as usize
}

/// Helper function to compress data and return compressed buffer
fn prepare_compressed_data(data: &[u32], block_size: NonZeroU32) -> Vec<u32> {
    let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, block_size);
    let mut compressed = vec![0u32; data.len() * 2];
    let mut input_offset = Cursor::new(0);
    let mut output_offset = Cursor::new(0);

    codec
        .compress(
            data,
            data.len() as u32,
            &mut input_offset,
            &mut compressed,
            &mut output_offset,
        )
        .unwrap();

    let compressed_size = output_offset.position() as usize;
    compressed.truncate(compressed_size);
    compressed
}

/// Helper function to decompress data
fn decompress_data(codec: &mut FastPFOR, compressed: &[u32], original_size: usize) -> usize {
    let mut decompressed = vec![0u32; original_size];
    let mut input_offset = Cursor::new(0);
    let mut output_offset = Cursor::new(0);

    codec
        .uncompress(
            compressed,
            compressed.len() as u32,
            &mut input_offset,
            &mut decompressed,
            &mut output_offset,
        )
        .unwrap();

    output_offset.position() as usize
}

fn benchmark_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");

    let patterns: &[(&str, DataGeneratorFn)] = &[
        (
            "uniform_small_value_distribution",
            generate_uniform_data_small_value_distribution,
        ),
        (
            "uniform_large_value_distribution",
            generate_uniform_data_large_value_distribution,
        ),
        ("clustered", generate_clustered_data),
        ("sequential", generate_sequential_data),
        ("sparse", generate_sparse_data),
    ];

    for &size in SIZES {
        for (name, generator) in patterns {
            let data = generator(size);
            group.throughput(Throughput::Elements(size as u64));
            group.bench_with_input(BenchmarkId::new(*name, size), &data, |b, data| {
                b.iter(|| {
                    let mut codec = FastPFOR::default();
                    black_box(compress_data(&mut codec, black_box(data)))
                });
            });
        }
    }

    group.finish();
}

fn benchmark_decompression(c: &mut Criterion) {
    let mut group = c.benchmark_group("decompression");

    let patterns: &[(&str, DataGeneratorFn)] = &[
        (
            "uniform_small_value_distribution",
            generate_uniform_data_small_value_distribution,
        ),
        (
            "uniform_large_value_distribution",
            generate_uniform_data_large_value_distribution,
        ),
        ("clustered", generate_clustered_data),
        ("sequential", generate_sequential_data),
        ("sparse", generate_sparse_data),
    ];

    for &size in SIZES {
        for (name, generator) in patterns {
            let data = generator(size);
            let compressed = prepare_compressed_data(&data, BLOCK_SIZE_128);

            group.throughput(Throughput::Elements(size as u64));
            group.bench_with_input(
                BenchmarkId::new(*name, size),
                &(compressed, size),
                |b, (compressed, size)| {
                    b.iter(|| {
                        let mut codec = FastPFOR::default();
                        black_box(decompress_data(&mut codec, black_box(compressed), *size))
                    });
                },
            );
        }
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
                b.iter(|| {
                    let mut codec1 = FastPFOR::default();
                    let mut codec2 = FastPFOR::default();
                    let mut compressed = vec![0u32; data.len() * 2];
                    let mut decompressed = vec![0u32; data.len()];
                    let mut input_offset = Cursor::new(0);
                    let mut output_offset = Cursor::new(0);

                    codec1
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

                    codec2
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
    let data = generate_uniform_data_small_value_distribution(size);

    let block_sizes = [BLOCK_SIZE_128, BLOCK_SIZE_256];

    for block_size in block_sizes {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_function(format!("compress_{block_size}"), |b| {
            b.iter(|| {
                let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, block_size);
                black_box(compress_data(&mut codec, black_box(&data)))
            });
        });
    }

    // Also benchmark decompression performance for each block size
    for block_size in block_sizes {
        // Pre-compress the data once for this block size so the decompression
        // benchmark measures only decompression work inside `b.iter`.
        let compressed = prepare_compressed_data(&data, block_size);

        group.throughput(Throughput::Elements(size as u64));
        group.bench_function(format!("decompress_{block_size}"), |b| {
            b.iter(|| {
                let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, block_size);
                black_box(decompress_data(&mut codec, black_box(&compressed), size));
            });
        });
    }
    group.finish();
}

fn benchmark_compression_ratio(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_ratio");
    group.sample_size(20); // Fewer samples since we're measuring ratio, not just speed

    let size = *SIZES.last().unwrap();
    let patterns: &[(&str, DataGeneratorFn)] = &[
        (
            "uniform_small_distribution",
            generate_uniform_data_small_value_distribution,
        ),
        (
            "uniform_large_distribution",
            generate_uniform_data_large_value_distribution,
        ),
        ("clustered", generate_clustered_data),
        ("sequential", generate_sequential_data),
        ("sparse", generate_sparse_data),
        ("constant", generate_constant_data),
        ("geometric", generate_geometric_data),
    ];

    for (name, data_fn) in patterns {
        let data = data_fn(size);
        group.bench_function(*name, |b| {
            b.iter(|| {
                let mut codec = FastPFOR::default();
                let compressed_size = compress_data(&mut codec, black_box(&data));
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "Loss of precision is acceptable for compression ratio calculation"
                )]
                let ratio = data.len() as f64 / compressed_size as f64;
                black_box(ratio)
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_compression,
    benchmark_decompression,
    benchmark_roundtrip,
    benchmark_block_sizes,
    benchmark_compression_ratio
);
criterion_main!(benches);
