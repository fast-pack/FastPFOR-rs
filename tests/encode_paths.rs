//! Integration tests covering encoding paths and `VariableByte` branches
//! not exercised by existing tests.
//!
//! These tests do NOT modify any production code; they only drive the public
//! API with inputs crafted to reach specific branches.

#![cfg(feature = "rust")]

#[path = "../src/test_utils.rs"]
mod test_utils;

use fastpfor::{FastPFor128, FastPFor256, FastPForBlock256, JustCopy, VariableByte};
use test_utils::{block_roundtrip, decompress, roundtrip};

// ── VariableByte round-trip ───────────────────────────────────────────────────

/// `VariableByte` round-trips values spanning all varint byte widths
/// (1- through 5-byte encodings).
#[test]
fn variable_byte_roundtrip_all_widths() {
    roundtrip::<VariableByte>(&[1u32, 127, 128, 16383, 16384, u32::MAX]);
}

#[test]
fn variable_byte_roundtrip_empty() {
    roundtrip::<VariableByte>(&[]);
}

// ── JustCopy via AnyLenCodec ─────────────────────────────────────────────────

#[test]
fn justcopy_roundtrip() {
    roundtrip::<JustCopy>(&[1u32, 2, 3, 42, u32::MAX]);
}

// ── BlockCodec: FastPForBlock256 — block-exact input ───────────────────────────────

#[test]
fn fastpfor256_block_roundtrip() {
    block_roundtrip::<FastPForBlock256>(&(0u32..512).collect::<Vec<_>>());
}

// ── CompositeCodec (FastPForBlock256 + VByte) ─────────────────────────────────────

/// Compressing more than the default page size (65536) causes `compress_blocks`
/// to loop more than once, exercising the `this_size == page_size` branch.
#[test]
fn fastpfor_multi_page_encode_decode() {
    // 65536 (default page size) + 256 (one block) — enough to span two pages
    let data: Vec<u32> = (0..65792u32).map(|i| i % 1024).collect();
    roundtrip::<FastPFor256>(&data);
}

/// A block of all zeros causes `best_b_from_data` to decrement `optimal_bits`
/// all the way to 0 — no packed words are written.
#[test]
fn fastpfor_encode_all_zeros() {
    roundtrip::<FastPFor256>(&vec![0u32; 256]);
}

/// When the metadata byte count is already a multiple of 4 the padding loop
/// runs zero iterations.
#[test]
fn fastpfor_encode_metadata_already_aligned() {
    let data = vec![0u32; 32768]; // 128 blocks of 256 zeros
    roundtrip::<FastPFor256>(&data);
}

/// When every value needs all 32 bits.
#[test]
fn fastpfor_encode_all_max_u32() {
    roundtrip::<FastPFor256>(&vec![u32::MAX; 256]);
}

/// Exception index == 1 branch.
#[test]
fn fastpfor_encode_exception_index1() {
    let mut data = vec![1u32; 256];
    data[0] = 3;
    data[128] = 3;
    roundtrip::<FastPFor256>(&data);
}

/// 128-element block size with exceptions.
#[test]
fn fastpfor_encode_128_block_with_exceptions() {
    let data: Vec<u32> = (0..128)
        .map(|i| if i % 4 == 0 { 1u32 << 28 } else { 1 })
        .collect();
    roundtrip::<FastPFor128>(&data);
}

// ── VariableByte AnyLenCodec edge cases ──────────────────────────────────────

/// Decompressing an empty stream succeeds with empty output.
#[test]
fn variable_byte_anylen_decompress_short_input() {
    let out = decompress::<VariableByte>(&[], None).unwrap();
    assert!(out.is_empty());
}

/// Decompressing into a `Vec` that starts empty is fine — it grows as needed.
#[test]
fn variable_byte_anylen_decompress_into_small_vec() {
    let data: Vec<u32> = (1..=20).collect();
    roundtrip::<VariableByte>(&data);
}
