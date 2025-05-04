#![cfg(feature = "rust")]

use fastpfor::cpp::{Codec32, FastPFor128Codec};
use fastpfor::rust::{FastPFOR, Integer, BLOCK_SIZE_128, DEFAULT_PAGE_SIZE};
use std::io::Cursor;

#[test]
fn test_rust_decompresses_cpp_encoded_data() {
    let test_input_size = 256;
    let input: Vec<u32> = (0..test_input_size).map(|i| (i * 32) ^ (i >> 1)).collect();

    // Buffer for the C++ encoded
    let mut compressed_buffer = vec![0; input.len()];

    // C++ encoding
    let codec_cpp = FastPFor128Codec::new();
    let encoded_cpp = codec_cpp
        .encode32(&input, &mut compressed_buffer)
        .expect("C++ compression failed");
    let compressed_len = encoded_cpp.len();

    // C++ decoding
    let mut decoded_by_cpp = vec![0; input.len()];
    let decoded_cpp = codec_cpp
        .decode32(encoded_cpp, &mut decoded_by_cpp)
        .expect("C++ decompression failed");

    // Rust decoding
    let mut input_offset = Cursor::new(0u32);
    let mut codec_rs = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    let mut decoded_by_rust = vec![0; input.len()];
    codec_rs
        .uncompress(
            &compressed_buffer,
            compressed_len as u32,
            &mut input_offset,
            &mut decoded_by_rust,
            &mut Cursor::new(0u32),
        )
        .expect("Rust decompression failed");

    assert_eq!(
        decoded_cpp.len(),
        decoded_by_rust.len(),
        "Mismatched output lengths"
    );
    assert_eq!(decoded_cpp, decoded_by_rust);
}
