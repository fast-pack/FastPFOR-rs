//! Compatibility tests between Rust and C++ codec implementations.

#![cfg(all(feature = "rust", feature = "cpp"))]

use std::io::Cursor;

use fastpfor::rust::Integer as _;
use fastpfor::{AnyLenCodec as _, rust};

mod common;
use common::{get_test_cases, test_input_sizes};
use fastpfor::cpp::CppFastPFor128;

#[test]
fn test_rust_decompresses_cpp_encoded_data() {
    let mut codec_cpp = CppFastPFor128::new();
    let mut codec_rs = rust::FastPFOR::new(rust::DEFAULT_PAGE_SIZE, rust::BLOCK_SIZE_128);

    for n in test_input_sizes() {
        for input in get_test_cases(n + rust::BLOCK_SIZE_128.get() as usize) {
            let mut compressed_buffer = Vec::new();
            codec_cpp.encode(&input, &mut compressed_buffer).unwrap();
            let compressed_len = compressed_buffer.len();

            let mut decoded_by_cpp = Vec::new();
            codec_cpp
                .decode(
                    &compressed_buffer,
                    &mut decoded_by_cpp,
                    Some(input.len() as u32),
                )
                .unwrap();
            let decoded_cpp = decoded_by_cpp.as_slice();

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
            assert_eq!(decoded_cpp, decoded_by_rust.as_slice());
        }
    }
}

#[test]
fn test_rust_and_cpp_fastpfor32_compression_matches() {
    let mut codec_cpp = CppFastPFor128::new();
    let mut codec_rs = rust::FastPFOR::new(rust::DEFAULT_PAGE_SIZE, rust::BLOCK_SIZE_128);

    for n in test_input_sizes() {
        for input in get_test_cases(n + rust::BLOCK_SIZE_128.get() as usize) {
            // Rust `FastPFOR::compress` is a no-op for length 0; C++ still writes a stream header.
            if input.is_empty() {
                continue;
            }

            let mut compressed_buffer = Vec::new();
            codec_cpp.encode(&input, &mut compressed_buffer).unwrap();

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

            let compressed_len_rs = output_offset_rs.position() as usize;
            assert_eq!(
                compressed_buffer.len(),
                compressed_len_rs,
                "C++ vs Rust compressed length mismatch"
            );
            assert_eq!(&compressed_buffer, &encoded_rs[..compressed_len_rs]);
        }
    }
}
