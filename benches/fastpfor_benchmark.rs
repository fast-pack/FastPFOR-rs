//! Benchmark suite for `FastPFOR` compression codecs.

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
#[cfg(feature = "cpp")]
use fastpfor::AnyLenCodec;
use fastpfor::{BlockCodec as _, FastPForBlock128, FastPForBlock256, slice_to_blocks};

#[path = "bench_utils.rs"]
mod bench_utils;
use bench_utils::{
    BlockSizeFixture, compress_fixtures, generate_uniform_data_small_value_distribution,
    ratio_fixtures,
};
#[cfg(feature = "cpp")]
use fastpfor::cpp::CppFastPFor128;

/// Number of blocks per benchmark run.  The element count per run is
/// `BLOCK_COUNTS[i] * C::elements_per_block()`, e.g. 8 × 128 = 1,024 or 32 × 128 = 4,096.
const BLOCK_COUNTS: &[usize] = &[8, 32];

fn benchmark_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");
    for (bc, fix) in compress_fixtures::<FastPForBlock128>(BLOCK_COUNTS) {
        let n_elem = fix.data.len();
        group.throughput(Throughput::Elements(n_elem as u64));
        group.bench_with_input(BenchmarkId::new(fix.name, bc), &fix.data, |b, data| {
            let mut codec = FastPForBlock128::default();
            let (blocks, _) = slice_to_blocks::<FastPForBlock128>(data);
            let mut out = Vec::new();
            b.iter(|| {
                out.clear();
                codec.encode_blocks(black_box(blocks), &mut out).unwrap();
                black_box(out.len())
            });
        });
    }
    group.finish();
}

fn benchmark_decompression(c: &mut Criterion) {
    let mut group = c.benchmark_group("decompression");
    for (bc, fix) in compress_fixtures::<FastPForBlock128>(BLOCK_COUNTS) {
        let n_elem = fix.data.len();
        group.throughput(Throughput::Elements(n_elem as u64));
        group.bench_with_input(BenchmarkId::new(fix.name, bc), &fix, |b, fix| {
            let mut codec = FastPForBlock128::default();
            let mut out = Vec::new();
            b.iter(|| {
                out.clear();
                codec
                    .decode_blocks(
                        black_box(&fix.compressed),
                        Some(
                            u32::try_from(fix.n_blocks * FastPForBlock128::size())
                                .expect("expected_values fits in u32"),
                        ),
                        &mut out,
                    )
                    .unwrap();
                black_box(out.len())
            });
        });
    }
    group.finish();
}

fn benchmark_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");
    for &bc in BLOCK_COUNTS {
        let data = generate_uniform_data_small_value_distribution(bc * FastPForBlock128::size());
        group.throughput(Throughput::Elements(data.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("compress_decompress", bc),
            &data,
            |b, data| {
                let mut codec = FastPForBlock128::default();
                let (blocks, _) = slice_to_blocks::<FastPForBlock128>(data);
                let mut compressed = Vec::new();
                let mut decompressed = Vec::new();
                b.iter(|| {
                    compressed.clear();
                    codec
                        .encode_blocks(black_box(blocks), &mut compressed)
                        .unwrap();
                    decompressed.clear();
                    codec
                        .decode_blocks(
                            &compressed,
                            Some(
                                u32::try_from(bc * FastPForBlock128::size())
                                    .expect("expected_values fits in u32"),
                            ),
                            &mut decompressed,
                        )
                        .unwrap();
                    black_box(decompressed.len())
                });
            },
        );
    }
    group.finish();
}

fn benchmark_block_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("block_sizes");
    let bc = *BLOCK_COUNTS.last().unwrap();

    let fix128 = BlockSizeFixture::<FastPForBlock128>::new(bc);
    let fix256 = BlockSizeFixture::<FastPForBlock256>::new(bc);

    for (label, data, compressed, n_blocks, is_256) in [
        (
            "128",
            &fix128.data,
            &fix128.compressed,
            fix128.n_blocks,
            false,
        ),
        (
            "256",
            &fix256.data,
            &fix256.compressed,
            fix256.n_blocks,
            true,
        ),
    ] {
        group.throughput(Throughput::Elements(data.len() as u64));
        group.bench_function(format!("compress_{label}"), |b| {
            if is_256 {
                let mut codec = FastPForBlock256::default();
                let (blocks, _) = slice_to_blocks::<FastPForBlock256>(data);
                let mut out = Vec::new();
                b.iter(|| {
                    out.clear();
                    codec.encode_blocks(black_box(blocks), &mut out).unwrap();
                    black_box(out.len())
                });
            } else {
                let mut codec = FastPForBlock128::default();
                let (blocks, _) = slice_to_blocks::<FastPForBlock128>(data);
                let mut out = Vec::new();
                b.iter(|| {
                    out.clear();
                    codec.encode_blocks(black_box(blocks), &mut out).unwrap();
                    black_box(out.len())
                });
            }
        });
        group.bench_function(format!("decompress_{label}"), |b| {
            if is_256 {
                let mut codec = FastPForBlock256::default();
                let mut out = Vec::new();
                let expected = u32::try_from(n_blocks * FastPForBlock256::size())
                    .expect("expected_values fits in u32");
                b.iter(|| {
                    out.clear();
                    codec
                        .decode_blocks(black_box(compressed), Some(expected), &mut out)
                        .unwrap();
                    black_box(out.len())
                });
            } else {
                let mut codec = FastPForBlock128::default();
                let mut out = Vec::new();
                let expected = (n_blocks * FastPForBlock128::size()) as u32;
                b.iter(|| {
                    out.clear();
                    codec
                        .decode_blocks(black_box(compressed), Some(expected), &mut out)
                        .unwrap();
                    black_box(out.len())
                });
            }
        });
    }
    group.finish();
}

fn benchmark_compression_ratio(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_ratio");
    group.sample_size(20);
    let bc = *BLOCK_COUNTS.last().unwrap();
    for fix in ratio_fixtures::<FastPForBlock128>(bc) {
        group.bench_function(fix.name, |b| {
            let mut codec = FastPForBlock128::default();
            let (blocks, _) = slice_to_blocks::<FastPForBlock128>(&fix.data);
            let mut out = Vec::new();
            b.iter(|| {
                out.clear();
                codec.encode_blocks(black_box(blocks), &mut out).unwrap();
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "Loss of precision is acceptable for compression ratio calculation"
                )]
                black_box(fix.data.len() as f64 / out.len() as f64)
            });
        });
    }
    group.finish();
}

/// Compare encoding and decoding speed of the C++ `CppFastPFor128` (`AnyLenCodec`) against
/// the pure-Rust `FastPForBlock128` (`BlockCodec`). Same wire format for block-aligned data.
#[cfg(feature = "cpp")]
fn benchmark_cpp_vs_rust(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpp_vs_rust/encode");
    for (bc, fix) in compress_fixtures::<FastPForBlock128>(BLOCK_COUNTS) {
        let n_elem = fix.data.len();
        group.throughput(Throughput::Elements(n_elem as u64));
        group.bench_with_input(
            BenchmarkId::new(format!("cpp/{}", fix.name), bc),
            &fix.data,
            |b, data| {
                let mut codec = CppFastPFor128::default();
                let mut out = Vec::new();
                b.iter(|| {
                    out.clear();
                    codec.encode(black_box(data), &mut out).unwrap();
                    black_box(out.len())
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new(format!("rust/{}", fix.name), bc),
            &fix.data,
            |b, data| {
                let mut codec = FastPForBlock128::default();
                let (blocks, _) = slice_to_blocks::<FastPForBlock128>(data);
                let mut out = Vec::new();
                b.iter(|| {
                    out.clear();
                    codec.encode_blocks(black_box(blocks), &mut out).unwrap();
                    black_box(out.len())
                });
            },
        );
    }
    group.finish();

    let mut group = c.benchmark_group("cpp_vs_rust/decode");
    for (bc, fix) in compress_fixtures::<FastPForBlock128>(BLOCK_COUNTS) {
        let n_elem = fix.n_blocks * FastPForBlock128::size();
        let expected_len = u32::try_from(n_elem).expect("n_elem fits in u32");
        group.throughput(Throughput::Elements(n_elem as u64));
        group.bench_with_input(
            BenchmarkId::new(format!("cpp/{}", fix.name), bc),
            &fix.compressed,
            |b, compressed| {
                let mut codec = CppFastPFor128::default();
                let mut out = Vec::new();
                b.iter(|| {
                    out.clear();
                    codec
                        .decode(black_box(compressed), &mut out, Some(expected_len))
                        .unwrap();
                    black_box(out.len())
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new(format!("rust/{}", fix.name), bc),
            &fix.compressed,
            |b, compressed| {
                let mut codec = FastPForBlock128::default();
                let mut out = Vec::new();
                b.iter(|| {
                    out.clear();
                    codec
                        .decode_blocks(
                            black_box(compressed),
                            Some(
                                u32::try_from(fix.n_blocks * FastPForBlock128::size())
                                    .expect("expected_values fits in u32"),
                            ),
                            &mut out,
                        )
                        .unwrap();
                    black_box(out.len())
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
