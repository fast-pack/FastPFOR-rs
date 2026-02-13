#![no_main]

use std::io::Cursor;
use std::num::NonZeroU32;

use fastpfor::rust::{DEFAULT_PAGE_SIZE, FastPFOR, Integer};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (u32, Vec<u32>)| {
    let (block_size, input_data) = data;

    let Ok(block_size) = NonZeroU32::try_from(block_size) else {
        return;
    };

    // Limit input size to avoid timeouts
    let input_data: Vec<u32> = input_data.into_iter().take(10_000).collect();

    // Allocate output buffer with generous size
    let mut compressed = vec![0u32; input_data.len() * 2 + 1024];

    // Compress the data
    let mut output_offset = Cursor::new(0);
    FastPFOR::new(DEFAULT_PAGE_SIZE, block_size)
        .compress(
            &input_data,
            input_data.len() as u32,
            &mut Cursor::new(0),
            &mut compressed,
            &mut output_offset,
        )
        .unwrap();
    let compressed_size = output_offset.position() as u32;
    if !input_data.is_empty() {
        assert!(compressed_size != 0, "compression should not be empty");
    }

    // Now decompress
    let mut decompressed = vec![0u32; input_data.len()];
    let mut output_offset = Cursor::new(0);

    FastPFOR::new(DEFAULT_PAGE_SIZE, block_size)
        .uncompress(
            &compressed,
            compressed_size,
            &mut Cursor::new(0),
            &mut decompressed,
            &mut output_offset,
        )
        .unwrap();
    let decompressed_length = output_offset.position() as usize;

    // Verify roundtrip
    if decompressed_length + input_data.len() < 200 {
        assert_eq!(
            input_data,
            decompressed[..decompressed_length],
            "Decompressed length mismatch: expected {}, got {decompressed_length}",
            input_data.len()
        );
    }

    for (i, (&original, &decoded)) in input_data.iter().zip(decompressed.iter()).enumerate() {
        assert_eq!(
            original, decoded,
            "Mismatch at position {}: expected {}, got {}",
            i, original, decoded
        );
    }
});
