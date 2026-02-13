#![no_main]

use fastpfor::cpp::{Codec32 as _, SimdFastPFor128Codec};
use libfuzzer_sys::fuzz_target;
use std::num::NonZeroU32;

fuzz_target!(|data: (u32, Vec<u32>)| {
    let (block_size, input) = data;

    let Ok(block_size) = NonZeroU32::try_from(block_size) else {
      return;
    };

    // TODO: fuzz more codecs
    let mut codec = SimdFastPFor128Codec::new();

    // Limit input size to avoid timeouts
    let input: Vec<u32> = input.into_iter().take(10_000).collect();

    // Allocate output buffer with generous size
    let mut output = vec![0u32; input.len() * 2 + 1024];

    // Compress the data
    let enc_slice = codec.encode32(&input, &mut output).unwrap();
    assert!(!enc_slice.is_empty(), "compression should not be empty");

    // Now decompress
    let mut decoded = vec![0u32; input.len() * 2 + 1024];
    let dec_slice = codec.decode32(&enc_slice, &mut decoded).unwrap();

    // Verify roundtrip
    if dec_slice.len() + input.len() < 200 {
        assert_eq!(
            input,
            dec_slice,
            "Decompressed output mismatches"
        );
    } else {
        assert_eq!(dec_slice.len(), input.len(), "Decompressed length mismatch");
        for (i, (&original, &decoded)) in input.iter().zip(dec_slice.iter()).enumerate() {
            assert_eq!(
                original, decoded,
                "Mismatch at position {i}: expected {original}, got {decoded}"
            );
        }
    }
});
