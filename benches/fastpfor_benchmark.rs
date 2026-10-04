//! Benchmark suite for `FastPFOR` compression codecs.

#![allow(clippy::unwrap_used)]

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
#[cfg(feature = "cpp")]
use fastpfor::AnyLenCodec;
use fastpfor::{
    Auto, BlockCodec, FastPForBlock, Interleaved, Portable, Sequential, slice_to_blocks,
};

// Shared helpers live in `src/bench_utils.rs` (library exposes the same file only under `cfg(test)`).
#[path = "../src/test_utils.rs"]
mod test_utils;
#[cfg(feature = "cpp")]
use fastpfor::cpp::CppFastPFor128;
use test_utils::{
    BlockSizeFixture, compress_fixtures, generate_uniform_data_small_value_distribution,
    ratio_fixtures,
};

/// Number of blocks per benchmark run.  The element count per run is
/// `BLOCK_COUNTS[i] * C::elements_per_block()`, e.g. 8 × 128 = 1,024 or 32 × 128 = 4,096.
const BLOCK_COUNTS: &[usize] = &[8, 32];

fn benchmark_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");
    for (bc, fix) in
        compress_fixtures::<FastPForBlock<Sequential, u32, 128, Portable>>(BLOCK_COUNTS)
    {
        let n_elem = fix.original.len();
        group.throughput(Throughput::Elements(n_elem as u64));
        group.bench_with_input(BenchmarkId::new(fix.name, bc), &fix.original, |b, data| {
            let mut codec = FastPForBlock::<Sequential, u32, 128, Portable>::default();
            let (blocks, _) =
                slice_to_blocks::<FastPForBlock<Sequential, u32, 128, Portable>>(data);
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
    for (bc, fix) in
        compress_fixtures::<FastPForBlock<Sequential, u32, 128, Portable>>(BLOCK_COUNTS)
    {
        let n_elem = fix.original.len();
        group.throughput(Throughput::Elements(n_elem as u64));
        group.bench_with_input(BenchmarkId::new(fix.name, bc), &fix, |b, fix| {
            let mut codec = FastPForBlock::<Sequential, u32, 128, Portable>::default();
            let mut out = Vec::new();
            b.iter(|| {
                out.clear();
                codec
                    .decode_blocks(
                        black_box(&fix.compressed),
                        Some(
                            u32::try_from(
                                fix.n_blocks
                                    * FastPForBlock::<Sequential, u32, 128, Portable>::size(),
                            )
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
        let data = generate_uniform_data_small_value_distribution(
            bc * FastPForBlock::<Sequential, u32, 128, Portable>::size(),
        );
        group.throughput(Throughput::Elements(data.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("compress_decompress", bc),
            &data,
            |b, data| {
                let mut codec = FastPForBlock::<Sequential, u32, 128, Portable>::default();
                let (blocks, _) =
                    slice_to_blocks::<FastPForBlock<Sequential, u32, 128, Portable>>(data);
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
                                u32::try_from(
                                    bc * FastPForBlock::<Sequential, u32, 128, Portable>::size(),
                                )
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

    let fix128 = BlockSizeFixture::<FastPForBlock<Sequential, u32, 128, Portable>>::new(bc);
    let fix256 = BlockSizeFixture::<FastPForBlock<Sequential, u32, 256, Portable>>::new(bc);

    for (label, data, compressed, n_blocks, is_256) in [
        (
            "128",
            &fix128.original,
            &fix128.compressed,
            fix128.n_blocks,
            false,
        ),
        (
            "256",
            &fix256.original,
            &fix256.compressed,
            fix256.n_blocks,
            true,
        ),
    ] {
        group.throughput(Throughput::Elements(data.len() as u64));
        group.bench_function(format!("compress_{label}"), |b| {
            if is_256 {
                let mut codec = FastPForBlock::<Sequential, u32, 256, Portable>::default();
                let (blocks, _) =
                    slice_to_blocks::<FastPForBlock<Sequential, u32, 256, Portable>>(data);
                let mut out = Vec::new();
                b.iter(|| {
                    out.clear();
                    codec.encode_blocks(black_box(blocks), &mut out).unwrap();
                    black_box(out.len())
                });
            } else {
                let mut codec = FastPForBlock::<Sequential, u32, 128, Portable>::default();
                let (blocks, _) =
                    slice_to_blocks::<FastPForBlock<Sequential, u32, 128, Portable>>(data);
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
                let mut codec = FastPForBlock::<Sequential, u32, 256, Portable>::default();
                let mut out = Vec::new();
                let expected = u32::try_from(
                    n_blocks * FastPForBlock::<Sequential, u32, 256, Portable>::size(),
                )
                .expect("expected_values fits in u32");
                b.iter(|| {
                    out.clear();
                    codec
                        .decode_blocks(black_box(compressed), Some(expected), &mut out)
                        .unwrap();
                    black_box(out.len())
                });
            } else {
                let mut codec = FastPForBlock::<Sequential, u32, 128, Portable>::default();
                let mut out = Vec::new();
                let expected =
                    (n_blocks * FastPForBlock::<Sequential, u32, 128, Portable>::size()) as u32;
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
    for fix in ratio_fixtures::<FastPForBlock<Sequential, u32, 128, Portable>>(bc) {
        group.bench_function(fix.name, |b| {
            let mut codec = FastPForBlock::<Sequential, u32, 128, Portable>::default();
            let (blocks, _) =
                slice_to_blocks::<FastPForBlock<Sequential, u32, 128, Portable>>(&fix.original);
            let mut out = Vec::new();
            b.iter(|| {
                out.clear();
                codec.encode_blocks(black_box(blocks), &mut out).unwrap();
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "Loss of precision is acceptable for compression ratio calculation"
                )]
                black_box(fix.original.len() as f64 / out.len() as f64)
            });
        });
    }
    group.finish();
}

/// Compare encoding and decoding speed of the C++ `CppFastPFor128` (`AnyLenCodec`) against
/// the pure-Rust `FastPForBlock<Sequential, u32, 128, Portable>` (`BlockCodec`). Same wire format for block-aligned data.
#[cfg(feature = "cpp")]
fn benchmark_cpp_vs_rust(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpp_vs_rust/encode");
    for (bc, fix) in
        compress_fixtures::<FastPForBlock<Sequential, u32, 128, Portable>>(BLOCK_COUNTS)
    {
        let n_elem = fix.original.len();
        group.throughput(Throughput::Elements(n_elem as u64));
        group.bench_with_input(
            BenchmarkId::new(format!("cpp/{}", fix.name), bc),
            &fix.original,
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
            &fix.original,
            |b, data| {
                let mut codec = FastPForBlock::<Sequential, u32, 128, Portable>::default();
                let (blocks, _) =
                    slice_to_blocks::<FastPForBlock<Sequential, u32, 128, Portable>>(data);
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
    for (bc, fix) in
        compress_fixtures::<FastPForBlock<Sequential, u32, 128, Portable>>(BLOCK_COUNTS)
    {
        let n_elem = fix.n_blocks * FastPForBlock::<Sequential, u32, 128, Portable>::size();
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
                let mut codec = FastPForBlock::<Sequential, u32, 128, Portable>::default();
                let mut out = Vec::new();
                b.iter(|| {
                    out.clear();
                    codec
                        .decode_blocks(
                            black_box(compressed),
                            Some(
                                u32::try_from(
                                    fix.n_blocks
                                        * FastPForBlock::<Sequential, u32, 128, Portable>::size(),
                                )
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

fn bench_kernel<C: BlockCodec>(
    c: &mut Criterion,
    group_name: &str,
    name: &str,
    kernel: &str,
    data: &[C::Elem],
) {
    let (blocks, _) = slice_to_blocks::<C>(data);
    let n_values = blocks.len() * C::size();
    let expected = Some(u32::try_from(n_values).expect("value count fits in u32"));
    let mut codec = C::default();
    let mut compressed = Vec::new();
    codec.encode_blocks(blocks, &mut compressed).unwrap();

    let mut group = c.benchmark_group(format!("{group_name}/encode"));
    group.throughput(Throughput::Elements(n_values as u64));
    group.bench_function(BenchmarkId::new(name, kernel), |b| {
        let mut out = Vec::new();
        b.iter(|| {
            out.clear();
            codec.encode_blocks(black_box(blocks), &mut out).unwrap();
            black_box(out.len())
        });
    });
    group.finish();

    let mut group = c.benchmark_group(format!("{group_name}/decode"));
    group.throughput(Throughput::Elements(n_values as u64));
    group.bench_function(BenchmarkId::new(name, kernel), |b| {
        let mut out = Vec::new();
        b.iter(|| {
            out.clear();
            codec
                .decode_blocks(black_box(&compressed), expected, &mut out)
                .unwrap();
            black_box(out.len())
        });
    });
    group.finish();
}

fn benchmark_scalar_vs_simd(c: &mut Criterion) {
    let bc = *BLOCK_COUNTS.last().unwrap();
    for (_, fix) in compress_fixtures::<FastPForBlock<Sequential, u32, 256, Portable>>(&[bc]) {
        let data = &fix.original;
        let wide: Vec<u64> = data
            .iter()
            .map(|&v| u64::from(v) << 24 | u64::from(v))
            .collect();
        let name = fix.name;
        bench_kernel::<FastPForBlock<Sequential, u32, 128, Portable>>(
            c,
            "kernels/u32x128",
            name,
            "scalar",
            data,
        );
        bench_kernel::<FastPForBlock<Sequential, u32, 128, Auto>>(
            c,
            "kernels/u32x128",
            name,
            "simd",
            data,
        );
        // The interleaved layout (C++ `SIMDFastPFor`): a different wire format, not a different speed of the same one.
        bench_kernel::<FastPForBlock<Interleaved, u32, 128, Portable>>(
            c,
            "kernels/u32x128",
            name,
            "interleaved-portable",
            data,
        );
        bench_kernel::<FastPForBlock<Interleaved, u32, 128, Auto>>(
            c,
            "kernels/u32x128",
            name,
            "interleaved",
            data,
        );
        bench_kernel::<FastPForBlock<Sequential, u32, 256, Portable>>(
            c,
            "kernels/u32x256",
            name,
            "scalar",
            data,
        );
        bench_kernel::<FastPForBlock<Sequential, u32, 256, Auto>>(
            c,
            "kernels/u32x256",
            name,
            "simd",
            data,
        );
        bench_kernel::<FastPForBlock<Interleaved, u32, 256, Auto>>(
            c,
            "kernels/u32x256",
            name,
            "interleaved",
            data,
        );
        bench_kernel::<FastPForBlock<Sequential, u64, 128, Portable>>(
            c,
            "kernels/u64x128",
            name,
            "scalar",
            &wide,
        );
        bench_kernel::<FastPForBlock<Sequential, u64, 128, Auto>>(
            c,
            "kernels/u64x128",
            name,
            "simd",
            &wide,
        );
        bench_kernel::<FastPForBlock<Interleaved, u64, 128, Auto>>(
            c,
            "kernels/u64x128",
            name,
            "interleaved",
            &wide,
        );
        bench_kernel::<FastPForBlock<Sequential, u64, 256, Portable>>(
            c,
            "kernels/u64x256",
            name,
            "scalar",
            &wide,
        );
        bench_kernel::<FastPForBlock<Sequential, u64, 256, Auto>>(
            c,
            "kernels/u64x256",
            name,
            "simd",
            &wide,
        );
        bench_kernel::<FastPForBlock<Interleaved, u64, 256, Auto>>(
            c,
            "kernels/u64x256",
            name,
            "interleaved",
            &wide,
        );
    }
}

criterion_group!(
    benches,
    benchmark_compression,
    benchmark_decompression,
    benchmark_roundtrip,
    benchmark_block_sizes,
    benchmark_compression_ratio,
    benchmark_scalar_vs_simd,
);

#[cfg(feature = "cpp")]
criterion_group!(cpp_benches, benchmark_cpp_vs_rust);

#[cfg(not(feature = "cpp"))]
criterion_main!(benches);

#[cfg(feature = "cpp")]
criterion_main!(benches, cpp_benches);
