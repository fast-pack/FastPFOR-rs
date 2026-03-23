//! Compatibility tests between Rust and C++ codec implementations.
//!
//! C++ codecs are composite (any-length); Rust block codecs produce the same wire format
//! for block-aligned data. Both sides use the same element-count header.

#![cfg(all(feature = "rust", feature = "cpp"))]

use fastpfor::{FastPFor128, FastPFor256, FastPForBlock128};

mod common;
use common::{get_test_cases, test_input_sizes};
use fastpfor::cpp::CppFastPFor128;
use fastpfor::{AnyLenCodec, BlockCodec, slice_to_blocks};

/// C++ `AnyLenCodec` encode → Rust `BlockCodec` decode (same wire format for block-aligned data).
#[test]
fn test_rust_decompresses_cpp_encoded_data() {
    let mut codec_cpp = CppFastPFor128::new();
    let mut codec_rs = FastPForBlock128::default();

    for n in test_input_sizes() {
        for input in get_test_cases(n + 128) {
            if input.len() % 128 != 0 || input.is_empty() {
                continue;
            }
            let n_blocks = input.len() / 128;

            let mut cpp_compressed = Vec::new();
            codec_cpp.encode(&input, &mut cpp_compressed).unwrap();

            let mut rust_decoded = Vec::new();
            codec_rs
                .decode_blocks(
                    &cpp_compressed,
                    Some(u32::try_from(n_blocks * 128).expect("block count fits in u32")),
                    &mut rust_decoded,
                )
                .unwrap_or_else(|e| panic!("Rust decompress of C++ data failed: {e:?}"));

            assert_eq!(
                rust_decoded,
                input,
                "C++→Rust roundtrip mismatch for len {}",
                input.len()
            );
        }
    }
}

/// Rust `BlockCodec` encode → C++ `AnyLenCodec` decode (same wire format).
#[test]
fn test_cpp_decompresses_rust_block_encoded_data() {
    let mut codec_cpp = CppFastPFor128::new();
    let mut codec_rs = FastPForBlock128::default();

    for n in test_input_sizes() {
        for input in get_test_cases(n + 128) {
            if input.len() % 128 != 0 || input.is_empty() {
                continue;
            }
            let (blocks, _) = slice_to_blocks::<FastPForBlock128>(&input);
            let n_blocks = blocks.len();
            let expected_len = n_blocks * 128;

            let mut rs_compressed = Vec::new();
            codec_rs.encode_blocks(blocks, &mut rs_compressed).unwrap();

            let mut cpp_decoded = Vec::new();
            codec_cpp
                .decode(
                    &rs_compressed,
                    &mut cpp_decoded,
                    Some(u32::try_from(expected_len).expect("expected len fits in u32")),
                )
                .unwrap_or_else(|e| panic!("C++ decompress of Rust data failed: {e:?}"));

            assert_eq!(
                cpp_decoded,
                input,
                "Rust→C++ roundtrip mismatch for len {}",
                input.len()
            );
        }
    }
}

/// Cross-check: Rust block encode and C++ any-length encode produce identical bytes for block-aligned input.
#[test]
fn test_rust_and_cpp_compression_matches() {
    let mut codec_cpp = CppFastPFor128::new();
    let mut codec_rs = FastPForBlock128::default();

    for n in test_input_sizes() {
        for input in get_test_cases(n + 128) {
            if input.len() % 128 != 0 || input.is_empty() {
                continue;
            }
            let (blocks_rs, _) = slice_to_blocks::<FastPForBlock128>(&input);

            let mut cpp_compressed = Vec::new();
            codec_cpp.encode(&input, &mut cpp_compressed).unwrap();

            let mut rs_compressed = Vec::new();
            codec_rs
                .encode_blocks(blocks_rs, &mut rs_compressed)
                .unwrap();

            assert_eq!(
                cpp_compressed,
                rs_compressed,
                "Compressed bytes differ for input len {}",
                input.len()
            );
        }
    }
}

/// Rust `AnyLenCodec` (`CompositeCodec`) encoder → round-trip.
#[test]
fn test_rust_anylen_roundtrip() {
    for n in test_input_sizes() {
        let mut codec = FastPFor256::default();
        for input in get_test_cases(n) {
            let mut compressed = Vec::new();
            codec.encode(&input, &mut compressed).unwrap();
            let mut decoded = Vec::new();
            codec.decode(&compressed, &mut decoded, None).unwrap();
            assert_eq!(decoded, input, "Rust AnyLenCodec round-trip failed");
        }
    }
}

/// Rust 128-block `AnyLenCodec` round-trip.
#[test]
fn test_rust_anylen_128_roundtrip() {
    for n in test_input_sizes() {
        let mut codec = FastPFor128::default();
        for input in get_test_cases(n) {
            let mut compressed = Vec::new();
            codec.encode(&input, &mut compressed).unwrap();
            let mut decoded = Vec::new();
            codec.decode(&compressed, &mut decoded, None).unwrap();
            assert_eq!(decoded, input, "Rust AnyLenCodec 128 round-trip failed");
        }
    }
}
