#![no_main]

use fastpfor::{CodecToSlice, cpp, rust};
use libfuzzer_sys::fuzz_target;
mod common;
use common::*;

fuzz_target!(|data: FuzzInput<RustCodec>| {
    let input = data.data;

    // TODO: Behaviour differs
    if input.is_empty() {
        return;
    }

    // TODO: Behaviour differs
    if data.codec == RustCodec::VariableByte {
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
    let mut cpp_compressed = vec![0u32; input.len() * 2 + 1024];
    let compressed_data = match data.codec {
        RustCodec::FastPFOR256 => {
            let mut cpp_codec = cpp::FastPFor256Codec::new();
            cpp_codec
                .compress_to_slice(input, &mut cpp_compressed)
                .expect("C++ compression failed")
        }
        RustCodec::FastPFOR128 => {
            let mut cpp_codec = cpp::FastPFor128Codec::new();
            cpp_codec
                .compress_to_slice(input, &mut cpp_compressed)
                .expect("C++ compression failed")
        }
        RustCodec::VariableByte => {
            let mut cpp_codec = cpp::VByteCodec::new();
            cpp_codec
                .compress_to_slice(input, &mut cpp_compressed)
                .expect("C++ compression failed")
        }
        RustCodec::JustCopy => {
            let mut cpp_codec = cpp::CopyCodec::new();
            cpp_codec
                .compress_to_slice(input, &mut cpp_compressed)
                .expect("C++ compression failed")
        }
    };

    // Now decompress with rust
    let mut rust_decompressed = vec![0u32; input.len()];
    let mut rust_codec = rust::Codec::from(data.codec);
    let rust_result = rust_codec
        .decompress_to_slice(compressed_data, &mut rust_decompressed)
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