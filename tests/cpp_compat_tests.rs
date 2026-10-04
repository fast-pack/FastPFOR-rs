//! Compatibility tests between Rust and C++ codec implementations.
//!
//! C++ codecs are composite (any-length); Rust block codecs produce the same wire format
//! for block-aligned data. Both sides use the same element-count header.

#![cfg(all(feature = "rust", feature = "cpp"))]

#[path = "../src/test_utils.rs"]
mod test_utils;

use fastpfor::cpp::CppFastPFor128;
use fastpfor::{FastPForBlock, FastPForSequential32x128, FastPForSequential32x256, Sequential};
use test_utils::{
    block_compress, block_decompress, compress, get_test_cases, roundtrip, roundtrip_full,
    test_input_sizes,
};

use crate::test_utils::decompress;

/// C++ `AnyLenCodec` encode → Rust `BlockCodec` decode (same wire format for block-aligned data).
#[test]
fn test_rust_decompresses_cpp_encoded_data() {
    for n in test_input_sizes() {
        for input in get_test_cases(n + 128) {
            if input.len() % 128 != 0 || input.is_empty() {
                continue;
            }
            let cpp_compressed = compress::<CppFastPFor128>(&input).unwrap();
            let rust_decoded = block_decompress::<FastPForBlock<Sequential, u32, 128>>(
                &cpp_compressed,
                Some(input.len() as u32),
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
    for n in test_input_sizes() {
        for input in get_test_cases(n + 128) {
            if input.len() % 128 != 0 || input.is_empty() {
                continue;
            }
            roundtrip_full::<FastPForSequential32x128, CppFastPFor128>(
                &input,
                Some(input.len().try_into().unwrap()),
            );
        }
    }
}

/// Cross-check: Rust block encode and C++ any-length encode produce identical bytes for block-aligned input.
#[test]
fn test_rust_and_cpp_compression_matches() {
    for n in test_input_sizes() {
        for input in get_test_cases(n + 128) {
            let len = input.len();
            if len % 128 != 0 || input.is_empty() {
                continue;
            }
            let compressed = compress::<CppFastPFor128>(&input).unwrap();
            assert_eq!(
                compressed,
                block_compress::<FastPForBlock<Sequential, u32, 128>>(&input).unwrap(),
                "Compressed bytes differ for input len {len}",
            );
            assert_eq!(
                decompress::<CppFastPFor128>(&compressed, None).unwrap(),
                input,
                "Rust→C++ roundtrip mismatch for len {len}",
            );
            assert_eq!(
                decompress::<FastPForSequential32x128>(&compressed, None).unwrap(),
                input,
                "Rust→C++ roundtrip mismatch for len {len}",
            );
        }
    }
}

/// Rust `AnyLenCodec` (`CompositeCodec`) encoder → round-trip.
#[test]
fn test_rust_anylen_roundtrip() {
    for n in test_input_sizes() {
        for input in get_test_cases(n) {
            roundtrip::<FastPForSequential32x256>(&input);
        }
    }
}

/// Rust 128-block `AnyLenCodec` round-trip.
#[test]
fn test_rust_anylen_128_roundtrip() {
    for n in test_input_sizes() {
        for input in get_test_cases(n) {
            roundtrip::<FastPForSequential32x128>(&input);
        }
    }
}

#[test]
fn test_rust_and_cpp_match_across_pages_with_exceptions() {
    let mut input: Vec<u32> = (0..65536u32 + 1024)
        .map(|i| match i % 7 {
            0 => 0xF000_0000 | i,
            3 => 0x0010_0000 | i,
            _ => i % 5,
        })
        .collect();
    input[65536 + 9] = u32::MAX;
    assert_eq!(
        compress::<CppFastPFor128>(&input).unwrap(),
        compress::<FastPForSequential32x128>(&input).unwrap()
    );
    assert_eq!(
        compress::<fastpfor::cpp::CppFastPFor256>(&input).unwrap(),
        compress::<FastPForSequential32x256>(&input).unwrap()
    );
}
