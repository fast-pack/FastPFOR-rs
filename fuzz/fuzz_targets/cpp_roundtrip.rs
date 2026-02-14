#![no_main]

use libfuzzer_sys::fuzz_target;
mod common;
use common::*;

fuzz_target!(|data: FuzzInput<CppCodec>| {
    let codec = BoxedCppCodec::from(data.codec);
    let input = data.data;

    // Allocate output buffer with generous size
    let mut output = vec![0u32; input.len() * 2 + 1024];

    // Compress the data
    let enc_slice = codec.encode32(&input, &mut output).unwrap();

    // Now decompress
    let mut decoded = vec![0u32; input.len() * 2 + 1024];
    let dec_slice = codec.decode32(enc_slice, &mut decoded).unwrap();

    // Verify roundtrip
    if dec_slice.len() + input.len() < 200 {
        assert_eq!(input, dec_slice, "Decompressed output mismatches");
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
