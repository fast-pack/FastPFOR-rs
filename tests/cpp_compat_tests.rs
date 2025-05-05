#![cfg(all(feature = "rust", feature = "cpp"))]

use fastpfor::cpp::Codec32;
use fastpfor::rust::Integer;
use fastpfor::{cpp, rust};
use std::io::Cursor;

#[test]
fn test_rust_decompresses_cpp_encoded_data() {
    let input: Vec<u32> = (0..256).map(|i| (i * 32) ^ (i >> 1)).collect();

    // Buffer for the C++ encoded
    let mut compressed_buffer = vec![0; input.len()];

    // C++ encoding
    let codec_cpp = cpp::FastPFor128Codec::new();
    let encoded_cpp = codec_cpp.encode32(&input, &mut compressed_buffer).unwrap();
    let compressed_len = encoded_cpp.len();

    // C++ decoding
    let mut decoded_by_cpp = vec![0; input.len()];
    let decoded_cpp = codec_cpp
        .decode32(encoded_cpp, &mut decoded_by_cpp)
        .unwrap();

    // Rust decoding
    let mut input_offset = Cursor::new(0u32);
    let mut codec_rs = rust::FastPFOR::new(rust::DEFAULT_PAGE_SIZE, rust::BLOCK_SIZE_128);
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
