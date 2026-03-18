//! Smoke tests that execute every benchmark code path exactly once.
//!
//! All logic lives in `benches/bench_utils.rs`; this file just drives the
//! shared fixtures through assertions so `cargo test` and coverage tools
//! cover the benchmark code paths without Criterion overhead.

#![cfg(feature = "rust")]

#[path = "../benches/bench_utils.rs"]
mod bench_utils;
use bench_utils::{
    BLOCK_SIZE_128, DEFAULT_PAGE_SIZE, FastPFOR, block_size_fixtures, compress_data,
    compress_fixtures, decompress_data, ratio_fixtures,
};
#[cfg(feature = "cpp")]
use bench_utils::{cpp_decode, cpp_decode_fixtures};

const SMOKE_SIZE: usize = 256;

#[test]
fn smoke_compression() {
    for (_, fix) in compress_fixtures(&[SMOKE_SIZE]) {
        let compressed = compress_data(&mut FastPFOR::default(), &fix.data);
        assert!(
            !compressed.is_empty(),
            "{}: compressed output must be non-empty",
            fix.name
        );
    }
}

#[test]
fn smoke_decompression() {
    let mut decompressed = vec![0u32; SMOKE_SIZE];
    for (_, fix) in compress_fixtures(&[SMOKE_SIZE]) {
        let n = decompress_data(
            &mut FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128),
            &fix.rust_compressed,
            &mut decompressed,
        );
        assert_eq!(
            &decompressed[..n],
            &fix.data[..],
            "{}: roundtrip mismatch",
            fix.name
        );
    }
}

/// Mirrors `benchmark_roundtrip`: compress then immediately decompress.
#[test]
fn smoke_roundtrip() {
    for (_, fix) in compress_fixtures(&[SMOKE_SIZE]) {
        let compressed = compress_data(&mut FastPFOR::default(), &fix.data);
        let mut decompressed = vec![0u32; fix.data.len()];
        let n = decompress_data(&mut FastPFOR::default(), &compressed, &mut decompressed);
        assert_eq!(
            &decompressed[..n],
            &fix.data[..],
            "{}: roundtrip mismatch",
            fix.name
        );
    }
}

#[test]
fn smoke_block_sizes() {
    for fix in block_size_fixtures(SMOKE_SIZE) {
        // compress_data path
        let compressed = compress_data(
            &mut FastPFOR::new(DEFAULT_PAGE_SIZE, fix.block_size),
            &fix.data,
        );
        assert_eq!(compressed, fix.compressed);
        // decompress_data path using the fixture's pre-compressed buffer
        let mut decompressed = vec![0u32; fix.data.len()];
        let n = decompress_data(
            &mut FastPFOR::new(DEFAULT_PAGE_SIZE, fix.block_size),
            &fix.compressed,
            &mut decompressed,
        );
        assert_eq!(&decompressed[..n], &fix.data[..]);
    }
}

#[test]
fn smoke_compression_ratio() {
    for fix in ratio_fixtures(SMOKE_SIZE) {
        let compressed = compress_data(&mut FastPFOR::default(), &fix.data);
        assert!(
            !compressed.is_empty(),
            "{}: compressed output must be non-empty",
            fix.name
        );
        #[expect(
            clippy::cast_precision_loss,
            reason = "Loss of precision is acceptable for compression ratio calculation"
        )]
        let ratio = fix.data.len() as f64 / compressed.len() as f64;
        assert!(
            ratio > 0.0,
            "{}: compression ratio must be positive",
            fix.name
        );
    }
}

#[cfg(feature = "cpp")]
#[test]
fn smoke_cpp_vs_rust() {
    use fastpfor::cpp::FastPFor128Codec;

    for (_, fix) in cpp_decode_fixtures(&[SMOKE_SIZE]) {
        // C++ decode
        let codec = FastPFor128Codec::new();
        let mut cpp_out = vec![0u32; fix.original_len];
        let n = cpp_decode(&codec, &fix.cpp_compressed, &mut cpp_out);
        assert_eq!(
            n, fix.original_len,
            "{}: C++ decoded wrong element count",
            fix.name
        );

        // Rust decode
        let mut rust_out = vec![0u32; fix.original_len];
        let n = decompress_data(
            &mut FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128),
            &fix.rust_compressed,
            &mut rust_out,
        );
        assert_eq!(
            n, fix.original_len,
            "{}: Rust decoded wrong element count",
            fix.name
        );
    }
}
