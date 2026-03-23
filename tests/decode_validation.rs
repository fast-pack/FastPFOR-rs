//! Integration tests: malformed compressed input must be rejected via
//! [`fastpfor::FastPForResult::Err`] through the public [`fastpfor::AnyLenCodec`] API
//! ([`fastpfor::FastPFor128`] only).
//!
//! Error cases that previously lived in `fastpfor.rs` unit tests (`try_decode` /
//! `decode_blocks`) are exercised here via `assert_fails` and `AnyLenCodec::decode`.

#![cfg(feature = "rust")]

use bytemuck::{cast_slice, cast_slice_mut};
use fastpfor::{AnyLenCodec, BlockCodec, FastPFor128, FastPForBlock128};

/// Matches `DEFAULT_PAGE_SIZE` in `fastpfor` (64 Ki integers per page).
const DEFAULT_PAGE_SIZE: u32 = 65536;

/// `compressed` must not decode successfully. Use `Some(128)` for a single full 128-block
/// stream; `None` for arbitrary garbage.
fn assert_fails<C: AnyLenCodec + Default>(compressed: &[u32], expected_len: Option<u32>) {
    let mut codec = C::default();
    let mut out = Vec::new();
    assert!(
        codec.decode(compressed, &mut out, expected_len).is_err(),
        "expected decode to fail with Err, but it succeeded"
    );
}

fn encode<C: BlockCodec + Default>(data: &[u32]) -> Vec<u32> {
    assert_eq!(data.len() % C::size(), 0);
    let blocks: &[C::Block] = cast_slice(data);
    let mut out = Vec::new();
    C::default()
        .encode_blocks(blocks, &mut out)
        .expect("encode one or more blocks");
    out
}

fn compressed_with_exceptions() -> Vec<u32> {
    let data: Vec<u32> = (0..128u32)
        .map(|i| if i % 2 == 0 { 1u32 << 30 } else { 3 })
        .collect();
    encode::<FastPForBlock128>(&data)
}

fn compressed_with_index1_exceptions() -> Vec<u32> {
    let mut data = vec![1u32; 128];
    data[0] = 3;
    encode::<FastPForBlock128>(&data)
}

fn meta_byte_start(compressed: &[u32]) -> usize {
    let where_meta = compressed[1] as usize;
    (1 + where_meta + 1) * 4
}

fn bitmap_idx(compressed: &[u32]) -> usize {
    let where_meta = compressed[1] as usize;
    let bytesize_idx = 1 + where_meta;
    let bytesize = compressed[bytesize_idx] as usize;
    bytesize_idx + 1 + bytesize.div_ceil(4)
}

fn find_exception_block(bytes: &[u8], meta_start: usize) -> Option<(usize, usize, usize)> {
    let mut pos = meta_start;
    while pos + 1 < bytes.len() {
        if bytes[pos + 1] > 0 {
            return Some((pos, pos + 1, pos + 2));
        }
        pos += 2;
    }
    None
}

#[test]
fn decode_returns_error_for_libfuzzer_arbitrary_words() {
    let data: &[u32] = &[
        42_926_275,
        589_967,
        4_522_053,
        589_967,
        3_646_554_563,
        55_438,
        u32::MAX,
        36,
    ];
    assert_fails::<FastPFor128>(data, None);
}

#[test]
fn decode_returns_error_for_minimal_three_word_garbage() {
    assert_fails::<FastPFor128>(&[0x200, 0, 1], None);
}

#[test]
fn decode_returns_error_when_block_stream_truncated() {
    let compressed = encode::<FastPForBlock128>(&[42u32; 128]);
    for truncated_len in [1, 2, compressed.len() / 2, compressed.len() - 1] {
        assert_fails::<FastPFor128>(&compressed[..truncated_len], Some(128));
    }
}

#[test]
fn decode_returns_error_when_where_meta_word_points_past_buffer() {
    let mut compressed = encode::<FastPForBlock128>(&[1u32; 128]);
    compressed[1] = u32::MAX;
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_only_out_length_word_present() {
    assert_fails::<FastPFor128>(&[128u32], Some(128));
}

#[test]
fn decode_returns_error_when_where_meta_out_of_bounds_on_exception_stream() {
    let mut compressed = compressed_with_exceptions();
    compressed[1] = u32::MAX;
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_bytesize_points_past_end() {
    let mut compressed = compressed_with_exceptions();
    compressed[1] = compressed.len() as u32 - 1;
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_bytesize_overflows_length() {
    let mut compressed = compressed_with_exceptions();
    let bytesize_idx = 1 + compressed[1] as usize;
    compressed[bytesize_idx] = u32::MAX - 3;
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_bitmap_reads_past_end() {
    let mut compressed = compressed_with_exceptions();
    let bytesize_idx = 1 + compressed[1] as usize;
    let remaining = (compressed.len() - bytesize_idx - 1) as u32;
    compressed[bytesize_idx] = remaining * 4;
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_exception_group_size_exceeds_page() {
    let mut compressed = compressed_with_exceptions();
    let size_idx = bitmap_idx(&compressed) + 1;
    compressed[size_idx] = DEFAULT_PAGE_SIZE + 1;
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_exception_bitstream_truncated() {
    let compressed = compressed_with_exceptions();
    assert_fails::<FastPFor128>(&compressed[..compressed.len() - 2], Some(128));
}

#[test]
fn decode_returns_error_when_packed_bit_width_byte_too_large() {
    let mut compressed = compressed_with_exceptions();
    let start = meta_byte_start(&compressed);
    cast_slice_mut::<_, u8>(&mut compressed)[start] = 33;
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_packed_region_truncated_before_metadata() {
    let compressed = compressed_with_exceptions();
    let where_meta = compressed[1] as usize;
    assert_fails::<FastPFor128>(&compressed[..where_meta], Some(128));
}

#[test]
fn decode_returns_error_when_exception_maxbits_too_large() {
    let mut compressed = compressed_with_exceptions();
    let start = meta_byte_start(&compressed);
    let bytes: &mut [u8] = cast_slice_mut(&mut compressed);
    if let Some((_, _, mb_off)) = find_exception_block(bytes, start) {
        bytes[mb_off] = 33;
    }
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_exception_index_underflows_optimal_bits() {
    let mut compressed = compressed_with_exceptions();
    let start = meta_byte_start(&compressed);
    let bytes: &mut [u8] = cast_slice_mut(&mut compressed);
    if let Some((bb_off, _, mb_off)) = find_exception_block(bytes, start) {
        bytes[mb_off] = bytes[bb_off].saturating_sub(1);
    }
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_exception_index_equals_optimal_bits() {
    let mut compressed = compressed_with_exceptions();
    let start = meta_byte_start(&compressed);
    let bytes: &mut [u8] = cast_slice_mut(&mut compressed);
    if let Some((bb_off, _, mb_off)) = find_exception_block(bytes, start) {
        bytes[mb_off] = bytes[bb_off];
    }
    assert_fails::<FastPFor128>(&compressed, Some(128));
}

#[test]
fn decode_returns_error_when_index1_exception_position_byte_truncated() {
    let compressed = compressed_with_index1_exceptions();
    assert_fails::<FastPFor128>(&compressed[..compressed.len() - 1], Some(128));
}

#[test]
fn decode_returns_error_when_exception_position_byte_truncated() {
    let compressed = compressed_with_exceptions();
    assert_fails::<FastPFor128>(&compressed[..compressed.len() - 1], Some(128));
}

#[test]
fn decode_returns_error_when_index1_exception_position_out_of_block() {
    let mut data = vec![1u32; 128];
    data[0] = 3;
    let mut buf = encode::<FastPForBlock128>(&data);
    let start = meta_byte_start(&buf);
    let bytes: &mut [u8] = cast_slice_mut(&mut buf);
    if let Some((bb_off, _, mb_off)) = find_exception_block(bytes, start) {
        if bytes[mb_off].wrapping_sub(bytes[bb_off]) == 1 && mb_off + 1 < bytes.len() {
            bytes[mb_off + 1] = 200;
        }
    }
    assert_fails::<FastPFor128>(&buf, Some(128));
}

#[test]
fn decode_returns_error_when_exception_position_out_of_block() {
    let data: Vec<u32> = (0..128u32)
        .map(|i| if i % 4 == 0 { 1u32 << 30 } else { 1 })
        .collect();
    let mut buf = encode::<FastPForBlock128>(&data);
    let start = meta_byte_start(&buf);
    let bytes: &mut [u8] = cast_slice_mut(&mut buf);
    if let Some((bb_off, _, mb_off)) = find_exception_block(bytes, start) {
        if bytes[mb_off].wrapping_sub(bytes[bb_off]) > 1 && mb_off + 1 < bytes.len() {
            bytes[mb_off + 1] = 200;
        }
    }
    assert_fails::<FastPFor128>(&buf, Some(128));
}
