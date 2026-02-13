#![no_main]

use fastpfor::rust::{FastPFOR, Integer, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

fuzz_target!(|(block_size_selector, input_data): (u8, Vec<u32>)| {
    if input_data.is_empty() {
        return;
    }

    // Limit input size to avoid timeouts
    let input_data: Vec<u32> = input_data.into_iter().take(10_000).collect();

    // Select block size based on fuzzer input
    let block_size = if block_size_selector {
        BLOCK_SIZE_128
    } else {
        BLOCK_SIZE_256
    };

    let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, block_size);

    // Allocate output buffer with generous size
    let output_size = input_data.len() * 2 + 1024;
    let mut compressed = vec![0u32; output_size];

    let input_length = input_data.len() as u32;
    let mut input_offset = Cursor::new(0);
    let mut output_offset = Cursor::new(0);

    // Compress the data
    let compress_result = codec
        .compress(
            &input_data,
            input_length,
            &mut input_offset,
            &mut compressed,
            &mut output_offset,
        )
        .expect("Buffers are valid, so we can compress");

    let compressed_size = output_offset.position() as u32;

    // Now decompress
    let mut decompressed = vec![0u32; input_data.len() + 1024];
    let mut decode_input_offset = Cursor::new(0);
    let mut decode_output_offset = Cursor::new(0);

    let decompress_result = codec
        .uncompress(
            &compressed,
            compressed_size,
            &mut decode_input_offset,
            &mut decompressed,
            &mut decode_output_offset,
        )
        .expect("If we can compress it, we can decompress it");

    // Verify roundtrip
    let decompressed_length = decode_output_offset.position() as usize;
    assert_eq!(
        input_data.len(),
        decompressed_length,
        "Decompressed length mismatch: expected {}, got {}",
        input_data.len(),
        decompressed_length
    );

    for (i, (&original, &decoded)) in input_data.iter().zip(decompressed.iter()).enumerate() {
        assert_eq!(
            original, decoded,
            "Mismatch at position {}: expected {}, got {}",
            i, original, decoded
        );
    }
});
