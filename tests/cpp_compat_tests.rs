//! Compatibility tests between Rust and C++ codec implementations.

#![cfg(all(feature = "rust", feature = "cpp"))]

use std::io::Cursor;

use fastpfor::cpp::Codec32 as _;
use fastpfor::rust::Integer as _;
use fastpfor::{cpp, rust};

mod common;
use common::{get_test_cases, test_input_sizes};

#[test]
fn test_rust_decompresses_cpp_encoded_data() {
    let codec_cpp = cpp::FastPFor128Codec::new();
    let mut codec_rs = rust::FastPFOR::new(rust::DEFAULT_PAGE_SIZE, rust::BLOCK_SIZE_128);

    for n in test_input_sizes() {
        for input in get_test_cases(n + rust::BLOCK_SIZE_128.get() as usize) {
            // Buffer for the C++ encoded
            let mut compressed_buffer = vec![0; input.len()];

            // C++ encoding
            let encoded_cpp = codec_cpp.encode32(&input, &mut compressed_buffer).unwrap();
            let compressed_len = encoded_cpp.len();

            // C++ decoding
            let mut decoded_by_cpp = vec![0; input.len()];
            let decoded_cpp = codec_cpp
                .decode32(encoded_cpp, &mut decoded_by_cpp)
                .unwrap();

            // Rust decoding
            let mut input_offset = Cursor::new(0u32);
            let mut decoded_by_rust = vec![0; input.len()];
            codec_rs
                .uncompress(
                    &compressed_buffer,
                    compressed_len as u32,
                    &mut input_offset,
                    &mut decoded_by_rust,
                    &mut Cursor::new(0u32),
                )
                .unwrap();

            assert_eq!(
                decoded_cpp.len(),
                decoded_by_rust.len(),
                "Mismatched output lengths"
            );
            assert_eq!(decoded_cpp, decoded_by_rust);
        }
    }
}

#[test]
fn test_rust_and_cpp_fastpfor32_compression_matches() {
    let codec_cpp = cpp::FastPFor128Codec::new();
    let mut codec_rs = rust::FastPFOR::new(rust::DEFAULT_PAGE_SIZE, rust::BLOCK_SIZE_128);

    for n in test_input_sizes() {
        for input in get_test_cases(n + rust::BLOCK_SIZE_128.get() as usize) {
            // Buffer for the C++ encoded
            let mut compressed_buffer = vec![0; input.len()];

            // C++ encoding
            let encoded_cpp = codec_cpp.encode32(&input, &mut compressed_buffer).unwrap();
            let compressed_len = encoded_cpp.len();

            // Rust encoding
            let mut input_offset_rs = Cursor::new(0u32);
            let mut encoded_rs: Vec<u32> = vec![0; input.len()];
            let mut output_offset_rs = Cursor::new(0u32);
            codec_rs
                .compress(
                    &input,
                    input.len() as u32,
                    &mut input_offset_rs,
                    &mut encoded_rs,
                    &mut output_offset_rs,
                )
                .unwrap();

            assert_eq!(
                &encoded_cpp[..compressed_len],
                &encoded_rs[..compressed_len]
            );
        }
    }
}
