#![no_main]

use fastpfor::{AnyLenCodec, CodecToSlice, rust};
use libfuzzer_sys::fuzz_target;
mod common;
use common::*;
use fastpfor::cpp::*;

fuzz_target!(|data: FuzzInput<RustCodec>| {
    let input = data.data;

    // TODO: Behaviour differs
    if input.is_empty() {
        return;
    }

    // TODO: To make the encoder not crash ->  Skip inputs smaller than block size
    let block_size = match data.codec {
        RustCodec::FastPFOR256 => 256,
        RustCodec::FastPFOR128 => 128,
        RustCodec::VariableByte => 1,
        RustCodec::JustCopy => 1,
    };
    if input.len() < block_size {
        return;
    }

    // TODO: To make the encoder not crash ->  Truncate to block size multiple
    let last_block_size_multiple = input.len() / block_size * block_size;
    let input = &input[..last_block_size_multiple];

    // Allocate output buffer for Rust (slice API)
    let mut rust_compressed = vec![0u32; input.len() * 2 + 1024];

    // Compress with Rust implementation using Codec wrapper
    let mut rust_codec = rust::Codec::from(data.codec);
    let rust_result = rust_codec
        .compress_to_slice(input, &mut rust_compressed)
        .expect("Rust compression failed");

    // Compress with C++ implementation (`AnyLenCodec` / Vec API)
    let mut cpp_compressed = Vec::new();
    match data.codec {
        RustCodec::FastPFOR256 => CppFastPFor256::new()
            .encode(input, &mut cpp_compressed)
            .expect("C++ compression failed"),
        RustCodec::FastPFOR128 => CppFastPFor128::new()
            .encode(input, &mut cpp_compressed)
            .expect("C++ compression failed"),
        RustCodec::VariableByte => CppMaskedVByte::new()
            .encode(input, &mut cpp_compressed)
            .expect("C++ compression failed"),
        RustCodec::JustCopy => CppCopy::new()
            .encode(input, &mut cpp_compressed)
            .expect("C++ compression failed"),
    }
    let compressed_oracle_from_cpp = cpp_compressed.as_slice();

    // Compare compressed outputs
    assert_eq!(
        rust_result.len(),
        compressed_oracle_from_cpp.len(),
        "Compressed length mismatch: Rust={}, C++={}",
        rust_result.len(),
        compressed_oracle_from_cpp.len()
    );

    for (i, (&rust_val, &cpp_val)) in rust_result
        .iter()
        .zip(compressed_oracle_from_cpp.iter())
        .enumerate()
    {
        assert_eq!(
            rust_val, cpp_val,
            "Compressed data mismatch at position {}: Rust={}, C++={}",
            i, rust_val, cpp_val
        );
    }
});
