//! Integration tests exercising every error return path in `FastPFOR::decode_page`.
//!
//! Each test:
//!   1. Compresses a valid input block to get a well-formed byte stream.
//!   2. Surgically corrupts exactly the field under test.
//!   3. Asserts that `uncompress` returns `Err(…)` rather than panicking.
//!
//! The goal is 100% branch coverage of the Rust decoding path.

#![cfg(feature = "rust")]

use std::io::Cursor;

use fastpfor::rust::{BLOCK_SIZE_128, DEFAULT_PAGE_SIZE, FastPFOR, Integer, Skippable};

// ── helpers ──────────────────────────────────────────────────────────────────

/// Compress `data` with a default (256-block, 65536-page) codec and return
/// the compressed words.
fn compress_block(data: &[u32]) -> Vec<u32> {
    let mut codec = FastPFOR::default();
    let mut buf = vec![0u32; data.len() * 8 + 1024];
    let mut in_off = Cursor::new(0u32);
    let mut out_off = Cursor::new(0u32);
    codec
        .compress(data, data.len() as u32, &mut in_off, &mut buf, &mut out_off)
        .expect("compression must succeed");
    buf.truncate(out_off.position() as usize);
    buf
}

/// Build compressed data that has at least one non-trivial exception group
/// (bit-width difference > 1) so that the full exception decode path is taken.
/// Returns `(compressed_words, original_data)`.
fn compressed_with_exceptions() -> (Vec<u32>, Vec<u32>) {
    // One 256-block where every even position carries a large value needing 31
    // bits and every odd position carries a small value needing 2 bits.
    // The encoder will choose optimal_bits ≈ 2, maxbits = 31, index = 29.
    let data: Vec<u32> = (0..256)
        .map(|i| if i % 2 == 0 { 1u32 << 30 } else { 3 })
        .collect();
    (compress_block(&data), data)
}

/// Build compressed data whose exception group uses bit-width difference == 1
/// `(maxbits - optimal_bits == 1)`, triggering the `index == 1` branch.
/// Returns `(compressed_words, original_data)`.
fn compressed_with_index1_exceptions() -> (Vec<u32>, Vec<u32>) {
    // Almost all values fit in 1 bit except one value needing exactly 2 bits.
    // Encoder picks optimal_bits=1, maxbits=2, index=1.
    let mut data = vec![1u32; 256];
    data[0] = 3; // needs 2 bits
    (compress_block(&data), data)
}

/// Decompress `compressed` with a fresh default codec.
fn try_decode(compressed: &[u32]) -> Result<(), impl std::fmt::Debug> {
    let mut codec = FastPFOR::default();
    let mut out = vec![0u32; 256 + 64];
    let mut in_off = Cursor::new(0u32);
    let mut out_off = Cursor::new(0u32);
    codec.uncompress(
        compressed,
        compressed.len() as u32,
        &mut in_off,
        &mut out,
        &mut out_off,
    )
}

// ── Wire format reference ─────────────────────────────────────────────────────
//
// `uncompress` reads word [0] as `outlength`, then delegates to
// `headless_uncompress` → `decode_page`.  Within one page (starting at
// `init_pos` in the u32 slice):
//
//   [init_pos]                              = where_meta
//   [init_pos+1 .. init_pos+where_meta-1]   = packed regular values
//   [init_pos+where_meta]                   = bytesize  (byte count of block metadata)
//   [+1 .. +ceil(bytesize/4)]               = block metadata bytes
//   [+ceil(bytesize/4)+1]                   = bitmap
//   for each set bit k (2..=32):
//     [next word]                           = size  (# of packed exceptions at width k)
//     [next ceil(size*k/32) words]          = bit-packed exception values
//
// In the full `compressed` slice (including the `outlength` prefix added by
// `compress`):
//   compressed[0]                           = outlength
//   compressed[1]                           = where_meta  (init_pos == 1 here)
//   compressed[1+where_meta]                = bytesize
//   …
//
// Helper to find the byte offset of the block-metadata region:
fn meta_byte_start(compressed: &[u32]) -> usize {
    let where_meta = compressed[1] as usize;
    let bytesize_idx = 1 + where_meta;
    (bytesize_idx + 1) * 4 // word index → byte offset
}

// Helper to find the word index of the bitmap:
fn bitmap_idx(compressed: &[u32]) -> usize {
    let where_meta = compressed[1] as usize;
    let bytesize_idx = 1 + where_meta;
    let bytesize = compressed[bytesize_idx] as usize;
    bytesize_idx + 1 + bytesize.div_ceil(4)
}

// ── Entry-point guards ────────────────────────────────────────────────────────

/// `input_length == 0` → `uncompress` returns Ok immediately.
#[test]
fn uncompress_zero_input_length_ok() {
    let mut codec = FastPFOR::default();
    let mut out = vec![];
    codec
        .uncompress(
            &[],
            0,
            &mut Cursor::new(0u32),
            &mut out,
            &mut Cursor::new(0u32),
        )
        .expect("empty uncompress must succeed");
}

/// `headless_uncompress` with `inlength == 0` and `BLOCK_SIZE_128` returns Ok immediately.
#[test]
fn headless_uncompress_zero_inlength_128_ok() {
    let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    let mut out = vec![];
    codec
        .headless_uncompress(
            &[],
            0,
            &mut Cursor::new(0u32),
            &mut out,
            &mut Cursor::new(0u32),
            0,
        )
        .expect("zero-length headless uncompress must succeed");
}

// ── decode_page error paths ───────────────────────────────────────────────────

/// Only the `outlength` word present — the page header (`where_meta`) is missing.
#[test]
fn decode_where_meta_missing() {
    assert!(try_decode(&[256u32]).is_err());
}

/// `where_meta` points past end of input.
#[test]
fn decode_where_meta_out_of_bounds() {
    let (mut compressed, _) = compressed_with_exceptions();
    compressed[1] = u32::MAX;
    assert!(try_decode(&compressed).is_err());
}

/// `init_pos + where_meta` wraps around u32.
#[test]
fn decode_where_meta_overflow() {
    let (compressed, _) = compressed_with_exceptions();
    // Prepend a dummy word and start decoding at offset 1, so init_pos = 1.
    // Set where_meta = u32::MAX so 1 + u32::MAX overflows.
    let mut padded = vec![0u32];
    padded.extend_from_slice(&compressed);
    padded[2] = u32::MAX; // where_meta field (compressed[1]) is now padded[2]
    let outlength = padded[1];
    let mut codec = FastPFOR::default();
    let mut out = vec![0u32; 320];
    let result = codec.headless_uncompress(
        &padded,
        outlength,
        &mut Cursor::new(1u32),
        &mut out,
        &mut Cursor::new(0u32),
        outlength,
    );
    assert!(result.is_err());
}

/// `where_meta` points to the last word, so reading `bytesize` goes out of bounds.
#[test]
fn decode_bytesize_out_of_bounds() {
    let (mut compressed, _) = compressed_with_exceptions();
    compressed[1] = compressed.len() as u32 - 1;
    assert!(try_decode(&compressed).is_err());
}

/// `bytesize` is so large that `inexcept + ceil(bytesize/4)` overflows u32.
#[test]
fn decode_bytesize_length_overflow() {
    let (mut compressed, _) = compressed_with_exceptions();
    let bytesize_idx = 1 + compressed[1] as usize;
    compressed[bytesize_idx] = u32::MAX - 3;
    assert!(try_decode(&compressed).is_err());
}

/// `bytesize` is crafted so that `inexcept` lands exactly at the end of the
/// slice, making the bitmap read go out of bounds.
#[test]
fn decode_bitmap_out_of_bounds() {
    let (mut compressed, _) = compressed_with_exceptions();
    let bytesize_idx = 1 + compressed[1] as usize;
    // Advance inexcept to exactly compressed.len() so the bitmap get_u32 fails.
    let remaining = (compressed.len() - bytesize_idx - 1) as u32;
    compressed[bytesize_idx] = remaining * 4;
    assert!(try_decode(&compressed).is_err());
}

/// `size` field for an exception group exceeds `page_size`.
#[test]
fn decode_exception_size_exceeds_page_size() {
    let (mut compressed, _) = compressed_with_exceptions();
    let size_idx = bitmap_idx(&compressed) + 1;
    compressed[size_idx] = DEFAULT_PAGE_SIZE.get() + 1;
    assert!(try_decode(&compressed).is_err());
}

/// Tail partial-group: not enough words remain after the full-group loop.
#[test]
fn decode_exception_partial_group_not_enough_data() {
    let (compressed, _) = compressed_with_exceptions();
    assert!(try_decode(&compressed[..compressed.len() - 2]).is_err());
}

/// `b > 32` in the per-block unpack loop.
#[test]
fn decode_block_b_too_large() {
    let (mut compressed, _) = compressed_with_exceptions();
    let start = meta_byte_start(&compressed);
    let bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut compressed);
    bytes[start] = 33; // overwrite best_b of block 0
    assert!(try_decode(&compressed).is_err());
}

/// Packed-values region is truncated: the bytesize read fails because `inexcept`
/// (= `init_pos + where_meta`) is already beyond the end of the truncated slice.
///
/// Note: the `in_start + b > input.len()` guard inside the bitunpack loop
/// (line 469 of fastpfor.rs) is structurally unreachable — the packed-values
/// region physically precedes the metadata in the stream, so any truncation
/// that removes packed words also removes the metadata needed to parse the page
/// header, and an earlier `NotEnoughData` fires first.  This test therefore
/// exercises the nearest reachable error: the `bytesize` read failing when
/// the metadata section is absent.
#[test]
fn decode_packed_region_truncated() {
    let (compressed, _) = compressed_with_exceptions();
    let where_meta = compressed[1] as usize;
    // Keep only [0 .. where_meta]: header word + part of packed-values region.
    // The bytesize read at `inexcept = 1 + where_meta` then fails OOB.
    assert!(try_decode(&compressed[..where_meta]).is_err());
}

/// `out_start + 32 > output.len()`: output buffer too small for bitunpacking.
#[test]
fn decode_output_buffer_too_small_unpack() {
    let (compressed, _) = compressed_with_exceptions();
    let mut codec = FastPFOR::default();
    let mut out = vec![0u32; 16]; // far too small for a 256-block
    let result = codec.uncompress(
        &compressed,
        compressed.len() as u32,
        &mut Cursor::new(0u32),
        &mut out,
        &mut Cursor::new(0u32),
    );
    assert!(result.is_err());
}

// ── Exception metadata validation ────────────────────────────────────────────

/// Finds the byte offset of the first block with `cexcept > 0` in the block
/// metadata region and returns `(best_b_offset, cexcept_offset, maxbits_offset)`.
fn find_exception_block(bytes: &[u8], meta_start: usize) -> Option<(usize, usize, usize)> {
    let mut pos = meta_start;
    while pos + 1 < bytes.len() {
        if bytes[pos + 1] > 0 {
            return Some((pos, pos + 1, pos + 2));
        }
        pos += 2; // skip blocks with no exceptions
    }
    None
}

/// `maxbits > 32`.
#[test]
fn decode_exception_maxbits_too_large() {
    let (mut compressed, _) = compressed_with_exceptions();
    let start = meta_byte_start(&compressed);
    let bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut compressed);
    if let Some((_, _, mb_off)) = find_exception_block(bytes, start) {
        bytes[mb_off] = 33;
    }
    assert!(try_decode(&compressed).is_err());
}

/// `maxbits < b` (`checked_sub` underflows → index is None).
#[test]
fn decode_exception_index_underflow() {
    let (mut compressed, _) = compressed_with_exceptions();
    let start = meta_byte_start(&compressed);
    let bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut compressed);
    if let Some((bb_off, _, mb_off)) = find_exception_block(bytes, start) {
        bytes[mb_off] = bytes[bb_off].saturating_sub(1); // maxbits < best_b
    }
    assert!(try_decode(&compressed).is_err());
}

/// `maxbits == b` (index == 0).
#[test]
fn decode_exception_index_zero() {
    let (mut compressed, _) = compressed_with_exceptions();
    let start = meta_byte_start(&compressed);
    let bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut compressed);
    if let Some((bb_off, _, mb_off)) = find_exception_block(bytes, start) {
        bytes[mb_off] = bytes[bb_off]; // maxbits == best_b → index 0
    }
    assert!(try_decode(&compressed).is_err());
}

// ── index == 1 branch ─────────────────────────────────────────────────────────

/// `index == 1` happy path: round-trips correctly.
#[test]
fn decode_index1_branch_valid() {
    let (compressed, data) = compressed_with_index1_exceptions();
    let mut codec = FastPFOR::default();
    let mut out = vec![0u32; data.len() + 64];
    codec
        .uncompress(
            &compressed,
            compressed.len() as u32,
            &mut Cursor::new(0u32),
            &mut out,
            &mut Cursor::new(0u32),
        )
        .expect("decompression of index-1 data must succeed");
    assert_eq!(&out[..data.len()], data.as_slice());
}

/// `index == 1`: exception position byte is missing (stream too short).
#[test]
fn decode_index1_pos_byte_missing() {
    let (compressed, _) = compressed_with_index1_exceptions();
    assert!(try_decode(&compressed[..compressed.len() - 1]).is_err());
}

/// `index == 1`: exception position `>= block_size` (use 128-block codec
/// so a u8 value of 200 exceeds the block size of 128).
#[test]
fn decode_index1_pos_out_of_block() {
    let mut data = vec![1u32; 128];
    data[0] = 3; // index == 1
    let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    let mut buf = vec![0u32; 1024];
    let mut out_off = Cursor::new(0u32);
    codec
        .compress(
            &data,
            data.len() as u32,
            &mut Cursor::new(0u32),
            &mut buf,
            &mut out_off,
        )
        .unwrap();
    buf.truncate(out_off.position() as usize);

    let start = meta_byte_start(&buf);
    let bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut buf);
    if let Some((bb_off, _, mb_off)) = find_exception_block(bytes, start) {
        if bytes[mb_off].wrapping_sub(bytes[bb_off]) == 1 && mb_off + 1 < bytes.len() {
            bytes[mb_off + 1] = 200; // position 200 >= block_size 128
        }
    }
    let _ = bytes;

    let mut dec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    let result = dec.uncompress(
        &buf,
        buf.len() as u32,
        &mut Cursor::new(0u32),
        &mut vec![0u32; 256],
        &mut Cursor::new(0u32),
    );
    assert!(result.is_err());
}

/// `index == 1`: output index out of bounds.
#[test]
fn decode_index1_output_out_of_bounds() {
    let (compressed, _) = compressed_with_index1_exceptions();
    let mut codec = FastPFOR::default();
    let result = codec.uncompress(
        &compressed,
        compressed.len() as u32,
        &mut Cursor::new(0u32),
        &mut [0u32; 16], // too small
        &mut Cursor::new(0u32),
    );
    assert!(result.is_err());
}

// ── index > 1 branch ──────────────────────────────────────────────────────────

/// `index > 1`: exception position byte is missing (stream too short).
#[test]
fn decode_exception_pos_byte_missing() {
    let (compressed, _) = compressed_with_exceptions();
    assert!(try_decode(&compressed[..compressed.len() - 1]).is_err());
}

/// `index > 1`: exception position `>= block_size` (128-block codec, value 200).
#[test]
fn decode_exception_pos_out_of_block() {
    let data: Vec<u32> = (0..128)
        .map(|i| if i % 4 == 0 { 1u32 << 30 } else { 1 })
        .collect();
    let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    let mut buf = vec![0u32; 2048];
    let mut out_off = Cursor::new(0u32);
    codec
        .compress(
            &data,
            data.len() as u32,
            &mut Cursor::new(0u32),
            &mut buf,
            &mut out_off,
        )
        .unwrap();
    buf.truncate(out_off.position() as usize);

    let start = meta_byte_start(&buf);
    let bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut buf);
    if let Some((bb_off, _, mb_off)) = find_exception_block(bytes, start) {
        if bytes[mb_off].wrapping_sub(bytes[bb_off]) > 1 && mb_off + 1 < bytes.len() {
            bytes[mb_off + 1] = 200; // position 200 >= block_size 128
        }
    }
    let _ = bytes;

    let mut dec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    let result = dec.uncompress(
        &buf,
        buf.len() as u32,
        &mut Cursor::new(0u32),
        &mut vec![0u32; 256],
        &mut Cursor::new(0u32),
    );
    assert!(result.is_err());
}

/// `index > 1`: output buffer too small (`out_idx` >= `output.len()`).
#[test]
fn decode_exception_output_out_of_bounds() {
    let (compressed, _) = compressed_with_exceptions();
    let mut codec = FastPFOR::default();
    let result = codec.uncompress(
        &compressed,
        compressed.len() as u32,
        &mut Cursor::new(0u32),
        &mut [0u32; 32], // too small for a 256-block
        &mut Cursor::new(0u32),
    );
    assert!(result.is_err());
}
