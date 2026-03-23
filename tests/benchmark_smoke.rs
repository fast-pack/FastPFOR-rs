//! Smoke tests that execute every benchmark code path exactly once.
//!
//! All logic lives in `benches/bench_utils.rs`; this file just drives the
//! shared fixtures through assertions so `cargo test` and coverage tools
//! cover the benchmark code paths without Criterion overhead.

#![cfg(feature = "rust")]

#[path = "../benches/bench_utils.rs"]
mod bench_utils;

#[cfg(feature = "cpp")]
use fastpfor::BlockCodec;
#[cfg(feature = "cpp")]
use fastpfor::cpp::CppFastPFor128;
use fastpfor::{FastPForBlock128, FastPForBlock256};

#[cfg(feature = "cpp")]
use crate::bench_utils::decompress;
use crate::bench_utils::{
    BlockSizeFixture, block_compress, block_decompress, block_roundtrip, compress_fixtures,
    ratio_fixtures,
};

const SMOKE_BLOCK_COUNT: usize = 2;

#[test]
fn smoke_compression() {
    for (_, fix) in compress_fixtures::<FastPForBlock128>(&[SMOKE_BLOCK_COUNT]) {
        assert!(
            !fix.original.is_empty(),
            "{}: compressed output must be non-empty",
            fix.name
        );
    }
}

#[test]
fn smoke_decompression() {
    for (_, fix) in compress_fixtures::<FastPForBlock128>(&[SMOKE_BLOCK_COUNT]) {
        let decompressed =
            block_decompress::<FastPForBlock128>(&fix.compressed, Some(fix.original.len() as u32));
        assert_eq!(
            decompressed.len(),
            fix.original.len(),
            "{}: decompressed length mismatch",
            fix.name
        );
        assert_eq!(
            decompressed, fix.original,
            "{}: roundtrip mismatch",
            fix.name
        );
    }
}

/// Mirrors `benchmark_roundtrip`: compress then immediately decompress.
#[test]
fn smoke_roundtrip() {
    for (_, fix) in compress_fixtures::<FastPForBlock128>(&[SMOKE_BLOCK_COUNT]) {
        block_roundtrip::<FastPForBlock128>(&fix.original);
    }
}

#[test]
fn smoke_block_sizes() {
    let fix128 = BlockSizeFixture::<FastPForBlock128>::new(SMOKE_BLOCK_COUNT);
    let fix256 = BlockSizeFixture::<FastPForBlock256>::new(SMOKE_BLOCK_COUNT);

    // 128-element blocks
    {
        let compressed = block_compress::<FastPForBlock128>(&fix128.original);
        assert_eq!(
            compressed, fix128.compressed,
            "128: compress output mismatch"
        );
        let decompressed =
            block_decompress::<FastPForBlock128>(&compressed, Some(fix128.original.len() as u32));
        assert_eq!(
            decompressed.len(),
            fix128.original.len(),
            "128: decompressed length mismatch"
        );
        assert_eq!(decompressed, fix128.original, "128: roundtrip mismatch");
    }

    // 256-element blocks
    {
        let compressed = block_compress::<FastPForBlock256>(&fix256.original);
        assert_eq!(
            compressed, fix256.compressed,
            "256: compress output mismatch"
        );
        let decompressed =
            block_decompress::<FastPForBlock256>(&compressed, Some(fix256.original.len() as u32));
        assert_eq!(
            decompressed.len(),
            fix256.original.len(),
            "256: decompressed length mismatch"
        );
        assert_eq!(decompressed, fix256.original, "256: roundtrip mismatch");
    }
}

#[test]
fn smoke_compression_ratio() {
    for fix in ratio_fixtures::<FastPForBlock128>(SMOKE_BLOCK_COUNT) {
        let out = block_compress::<FastPForBlock128>(&fix.original);
        assert!(
            !out.is_empty(),
            "{}: compressed output must be non-empty",
            fix.name
        );
        #[expect(
            clippy::cast_precision_loss,
            reason = "Loss of precision is acceptable for compression ratio calculation"
        )]
        let ratio = fix.original.len() as f64 / out.len() as f64;
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
    for (_, fix) in compress_fixtures::<FastPForBlock128>(&[SMOKE_BLOCK_COUNT]) {
        let expected_len = fix.n_blocks * FastPForBlock128::size();

        let out = decompress::<CppFastPFor128>(&fix.compressed, Some(expected_len as u32));
        assert_eq!(out, fix.original, "{}: Bad C++ roundtrip", fix.name);

        let out =
            block_decompress::<FastPForBlock128>(&fix.compressed, Some(fix.original.len() as u32));
        assert_eq!(out, fix.original, "{}: Bad Rust roundtrip", fix.name);
    }
}
