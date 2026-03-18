//! Integration tests covering encoding paths and `VariableByte` branches
//! not exercised by existing tests.
//!
//! These tests do NOT modify any production code; they only drive the public
//! API with inputs crafted to reach specific branches.

#![cfg(feature = "rust")]

use std::io::Cursor;

use fastpfor::CodecToSlice;
use fastpfor::rust::{
    BLOCK_SIZE_128, BLOCK_SIZE_256, Codec, DEFAULT_PAGE_SIZE, FastPFOR, Skippable, VariableByte,
};

// ── helper ────────────────────────────────────────────────────────────────────

/// Compress then immediately decompress, asserting round-trip correctness.
fn roundtrip(mut codec: Codec, data: &[u32]) {
    let mut compressed = vec![0u32; data.len() * 4 + 1024];
    let compressed = codec.compress_to_slice(data, &mut compressed).unwrap();

    let mut decompressed = vec![0u32; data.len() + 256];
    let decompressed = codec
        .decompress_to_slice(compressed, &mut decompressed)
        .unwrap();

    assert_eq!(decompressed, data);
}

// ── VariableByte round-trip ───────────────────────────────────────────────────

/// `VariableByte` round-trips values spanning all varint byte widths
/// (1- through 5-byte encodings).
#[test]
fn variable_byte_roundtrip_all_widths() {
    roundtrip(
        Codec::from(VariableByte::new()),
        &[1u32, 127, 128, 16383, 16384, u32::MAX],
    );
}

// ── VariableByte::headless_uncompress → Unimplemented ────────────────────────

/// `VariableByte::headless_uncompress` is intentionally unimplemented.
/// Calling it must return `Err(Unimplemented)`.
#[test]
fn variable_byte_headless_uncompress_unimplemented() {
    let result = VariableByte::new().headless_uncompress(
        &[],
        0,
        &mut Cursor::new(0u32),
        &mut [],
        &mut Cursor::new(0u32),
        0,
    );
    assert!(result.is_err());
}

// ── OutputBufferTooSmall in fast-path decompression ───────────────────────────

/// Compress enough integers so the fast path (`byte_pos + 10 <= byte_length`)
/// is entered, then decompress into a zero-capacity buffer so that
/// `tmp_outpos >= output.len()` fires in the fast-path loop.
#[test]
fn variable_byte_uncompress_fast_path_output_too_small() {
    // 20 small values → 20 compressed bytes, well above the 10-byte fast-path threshold.
    let data: Vec<u32> = (1..=20).collect();
    let mut compressed = vec![0u32; 64];
    let compressed = Codec::from(VariableByte::new())
        .compress_to_slice(&data, &mut compressed)
        .unwrap();

    let mut out: Vec<u32> = vec![]; // zero capacity → error on first decoded value
    let result = Codec::from(VariableByte::new()).decompress_to_slice(compressed, &mut out);
    assert!(result.is_err());
}

// ── OutputBufferTooSmall in slow-path decompression ───────────────────────────

/// Compress only 2 integers (2 compressed bytes < 10), so only the slow path
/// runs, then decompress into a zero-capacity buffer so that
/// `tmp_outpos >= output.len()` fires in the slow-path loop.
#[test]
fn variable_byte_uncompress_slow_path_output_too_small() {
    let data = vec![1u32, 2u32]; // 2 bytes total → slow path only
    let mut compressed = vec![0u32; 16];
    let compressed = Codec::from(VariableByte::new())
        .compress_to_slice(&data, &mut compressed)
        .unwrap();

    let mut out: Vec<u32> = vec![];
    let result = Codec::from(VariableByte::new()).decompress_to_slice(compressed, &mut out);
    assert!(result.is_err());
}

// ── FastPFOR encoding: multi-page path ───────────────────────────────────────

/// Compressing more than `page_size` integers causes `headless_compress` to
/// loop more than once, exercising the `this_size == page_size` branch.
#[test]
fn fastpfor_multi_page_encode_decode() {
    let n = DEFAULT_PAGE_SIZE.get() as usize + BLOCK_SIZE_256.get() as usize;
    let data: Vec<u32> = (0..n as u32).map(|i| i % 1024).collect();
    roundtrip(Codec::from(FastPFOR::default()), &data);
}

// ── FastPFOR encoding: all-zero block (b=0, no packing) ──────────────────────

/// A block of all zeros causes `best_b_from_data` to decrement `optimal_bits`
/// all the way to 0 — no packed words are written.
#[test]
fn fastpfor_encode_all_zeros() {
    roundtrip(Codec::from(FastPFOR::default()), &vec![0u32; 256]);
}

// ── FastPFOR encoding: bytes_container already 4-byte aligned ────────────────

/// When the metadata byte count is already a multiple of 4 the padding
/// `while (bytes_container.len() & 3) != 0` loop runs zero iterations.
/// 128 blocks × 2 metadata bytes each = 256 bytes ≡ 0 (mod 4).
#[test]
fn fastpfor_encode_metadata_already_aligned() {
    let data = vec![0u32; 32768]; // 128 blocks of 256 zeros
    roundtrip(
        Codec::from(FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_256)),
        &data,
    );
}

// ── FastPFOR encoding: best_b stays at 32 ────────────────────────────────────

/// When every value needs all 32 bits, `best_b_from_data` keeps `optimal_bits`
/// at 32 because no lower bit-width reduces the total cost.
#[test]
fn fastpfor_encode_all_max_u32() {
    roundtrip(Codec::from(FastPFOR::default()), &vec![u32::MAX; 256]);
}

// ── FastPFOR encoding: exception index == 1 ──────────────────────────────────

/// When `max_bits - optimal_bits == 1` the cost formula applies a discount
/// (`thiscost -= cexcept`).  This exercises the `if self.max_bits - b == 1`
/// branch in `best_b_from_data`.
#[test]
fn fastpfor_encode_exception_index1() {
    // Almost all values fit in 1 bit; two need exactly 2 bits.
    // Encoder picks optimal_bits=1, max_bits=2, index=1.
    let mut data = vec![1u32; 256];
    data[0] = 3;
    data[128] = 3;
    roundtrip(Codec::from(FastPFOR::default()), &data);
}

// ── FastPFOR encoding: 128-element block size ─────────────────────────────────

/// `BLOCK_SIZE_128` uses different inner-loop bounds in `encode_page`; verify
/// it compresses and decompresses correctly with exceptions present.
#[test]
fn fastpfor_encode_128_block_with_exceptions() {
    let data: Vec<u32> = (0..128)
        .map(|i| if i % 4 == 0 { 1u32 << 28 } else { 1 })
        .collect();
    roundtrip(
        Codec::from(FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128)),
        &data,
    );
}
