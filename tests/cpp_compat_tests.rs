//! Compatibility tests between Rust and C++ codec implementations.
//!
//! C++ codecs are composite (any-length); Rust block codecs produce the same wire format
//! for block-aligned data. Both sides use the same element-count header.

#![cfg(all(feature = "rust", feature = "cpp"))]

#[path = "../src/test_utils.rs"]
mod test_utils;

use fastpfor::{FastPFor128, FastPFor256, FastPForBlock128};
use test_utils::{block_compress, block_decompress, compress, roundtrip, roundtrip_full};

mod common;
use common::{get_test_cases, test_input_sizes};
use fastpfor::cpp::CppFastPFor128;

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
            let rust_decoded =
                block_decompress::<FastPForBlock128>(&cpp_compressed, Some(input.len() as u32))
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
            roundtrip_full::<FastPFor128, CppFastPFor128>(
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
                block_compress::<FastPForBlock128>(&input).unwrap(),
                "Compressed bytes differ for input len {len}",
            );
            assert_eq!(
                decompress::<CppFastPFor128>(&compressed, None).unwrap(),
                input,
                "Rust→C++ roundtrip mismatch for len {len}",
            );
            assert_eq!(
                decompress::<FastPFor128>(&compressed, None).unwrap(),
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
            roundtrip::<FastPFor256>(&input);
        }
    }
}

/// Rust 128-block `AnyLenCodec` round-trip.
#[test]
fn test_rust_anylen_128_roundtrip() {
    for n in test_input_sizes() {
        for input in get_test_cases(n) {
            roundtrip::<FastPFor128>(&input);
        }
    }
}
