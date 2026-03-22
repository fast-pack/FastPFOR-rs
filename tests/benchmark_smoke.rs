//! Smoke tests that execute every benchmark code path exactly once.
//!
//! All logic lives in `benches/bench_utils.rs`; this file just drives the
//! shared fixtures through assertions so `cargo test` and coverage tools
//! cover the benchmark code paths without Criterion overhead.

#![cfg(feature = "rust")]

#[path = "../benches/bench_utils.rs"]
mod bench_utils;

#[cfg(feature = "cpp")]
use bench_utils::decompress_anylen;
use bench_utils::{BlockSizeFixture, compress, compress_fixtures, decompress, ratio_fixtures};
#[cfg(feature = "cpp")]
use fastpfor::BlockCodec;
#[cfg(feature = "cpp")]
use fastpfor::cpp::CppFastPFor128;
use fastpfor::{FastPForBlock128, FastPForBlock256};

const SMOKE_BLOCK_COUNT: usize = 2;

#[test]
fn smoke_compression() {
    for (_, fix) in compress_fixtures::<FastPForBlock128>(&[SMOKE_BLOCK_COUNT]) {
        let mut out = Vec::new();
        compress::<FastPForBlock128>(&fix.data, &mut out);
        assert!(
            !out.is_empty(),
            "{}: compressed output must be non-empty",
            fix.name
        );
    }
}

#[test]
fn smoke_decompression() {
    for (_, fix) in compress_fixtures::<FastPForBlock128>(&[SMOKE_BLOCK_COUNT]) {
        let mut decompressed = Vec::new();
        let n = decompress::<FastPForBlock128>(&fix.compressed, fix.n_blocks, &mut decompressed);
        assert_eq!(
            n,
            fix.data.len(),
            "{}: decompressed length mismatch",
            fix.name
        );
        assert_eq!(decompressed, fix.data, "{}: roundtrip mismatch", fix.name);
    }
}

/// Mirrors `benchmark_roundtrip`: compress then immediately decompress.
#[test]
fn smoke_roundtrip() {
    for (_, fix) in compress_fixtures::<FastPForBlock128>(&[SMOKE_BLOCK_COUNT]) {
        let mut compressed = Vec::new();
        compress::<FastPForBlock128>(&fix.data, &mut compressed);
        let mut decompressed = Vec::new();
        let n = decompress::<FastPForBlock128>(&compressed, fix.n_blocks, &mut decompressed);
        assert_eq!(n, fix.data.len(), "{}: roundtrip length mismatch", fix.name);
        assert_eq!(decompressed, fix.data, "{}: roundtrip mismatch", fix.name);
    }
}

#[test]
fn smoke_block_sizes() {
    let fix128 = BlockSizeFixture::<FastPForBlock128>::new(SMOKE_BLOCK_COUNT);
    let fix256 = BlockSizeFixture::<FastPForBlock256>::new(SMOKE_BLOCK_COUNT);

    // 128-element blocks
    {
        let mut compressed = Vec::new();
        compress::<FastPForBlock128>(&fix128.data, &mut compressed);
        assert_eq!(
            compressed, fix128.compressed,
            "128: compress output mismatch"
        );
        let mut decompressed = Vec::new();
        let n = decompress::<FastPForBlock128>(&compressed, fix128.n_blocks, &mut decompressed);
        assert_eq!(n, fix128.data.len(), "128: decompressed length mismatch");
        assert_eq!(decompressed, fix128.data, "128: roundtrip mismatch");
    }

    // 256-element blocks
    {
        let mut compressed = Vec::new();
        compress::<FastPForBlock256>(&fix256.data, &mut compressed);
        let mut decompressed = Vec::new();
        let n = decompress::<FastPForBlock256>(&compressed, fix256.n_blocks, &mut decompressed);
        assert_eq!(n, fix256.data.len(), "256: decompressed length mismatch");
        assert_eq!(decompressed, fix256.data, "256: roundtrip mismatch");
    }
}

#[test]
fn smoke_compression_ratio() {
    for fix in ratio_fixtures::<FastPForBlock128>(SMOKE_BLOCK_COUNT) {
        let mut out = Vec::new();
        compress::<FastPForBlock128>(&fix.data, &mut out);
        assert!(
            !out.is_empty(),
            "{}: compressed output must be non-empty",
            fix.name
        );
        #[expect(
            clippy::cast_precision_loss,
            reason = "Loss of precision is acceptable for compression ratio calculation"
        )]
        let ratio = fix.data.len() as f64 / out.len() as f64;
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

        // C++ decode (same wire format as Rust; C++ uses AnyLenCodec)
        let mut cpp_out = Vec::new();
        let n = decompress_anylen::<CppFastPFor128>(&fix.compressed, expected_len, &mut cpp_out);
        assert_eq!(
            n, expected_len,
            "{}: C++ decoded wrong element count",
            fix.name
        );
        assert_eq!(cpp_out, fix.data, "{}: C++ roundtrip mismatch", fix.name);

        // Rust decode
        let mut rust_out = Vec::new();
        let n = decompress::<FastPForBlock128>(&fix.compressed, fix.n_blocks, &mut rust_out);
        assert_eq!(
            n, expected_len,
            "{}: Rust decoded wrong element count",
            fix.name
        );
        assert_eq!(rust_out, fix.data, "{}: Rust roundtrip mismatch", fix.name);
    }
}
