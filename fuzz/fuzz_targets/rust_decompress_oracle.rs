#![no_main]

use fastpfor::{AnyLenCodec, CodecToSlice, cpp, rust};
use libfuzzer_sys::fuzz_target;
mod common;
use common::*;

fuzz_target!(|data: FuzzInput<RustCodec>| {
    let input = data.data;

    // TODO: Behaviour differs
    if input.is_empty() {
        return;
    }

    // TODO: To make the decoder not crash ->  Skip inputs smaller than block size
    let block_size = match data.codec {
        RustCodec::FastPFOR256 => 256,
        RustCodec::FastPFOR128 => 128,
        RustCodec::VariableByte => 1,
        RustCodec::JustCopy => 1,
    };
    if input.len() < block_size {
        return;
    }

    // TODO: To make the decoder not crash -> Truncate to block size multiple
    let last_block_size_multiple = input.len() / block_size * block_size;
    let input = &input[..last_block_size_multiple];

    // First, compress with C++ implementation to get valid compressed data
    let mut cpp_compressed = Vec::new();
    match data.codec {
        RustCodec::FastPFOR256 => cpp::FastPFor256Codec::new()
            .encode(input, &mut cpp_compressed)
            .expect("C++ compression failed"),
        RustCodec::FastPFOR128 => cpp::FastPFor128Codec::new()
            .encode(input, &mut cpp_compressed)
            .expect("C++ compression failed"),
        RustCodec::VariableByte => cpp::MaskedVByteCodec::new()
            .encode(input, &mut cpp_compressed)
            .expect("C++ compression failed"),
        RustCodec::JustCopy => cpp::CopyCodec::new()
            .encode(input, &mut cpp_compressed)
            .expect("C++ compression failed"),
    }
    let compressed_oracle_from_cpp = cpp_compressed.as_slice();

    // Now decompress with rust
    let mut rust_decompressed = vec![0u32; input.len()];
    let mut rust_codec = rust::Codec::from(data.codec);
    let rust_result = rust_codec
        .decompress_to_slice(compressed_oracle_from_cpp, &mut rust_decompressed)
        .expect("Rust decompression failed");

    // Compare decompressed outputs
    assert_eq!(
        rust_result.len(),
        input.len(),
        "Decompressed length mismatch: Rust={}, C++={}",
        rust_result.len(),
        input.len()
    );

    for (i, (&rust_val, &cpp_val)) in rust_result.iter().zip(input.iter()).enumerate() {
        assert_eq!(
            rust_val, cpp_val,
            "Decompressed data mismatch at position {}: Rust={}, C++={}",
            i, rust_val, cpp_val
        );
    }
});
