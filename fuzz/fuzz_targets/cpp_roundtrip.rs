#![no_main]

use libfuzzer_sys::fuzz_target;
mod common;
use common::*;

fuzz_target!(|data: FuzzInput<CppCodec>| {
    let mut codec = BoxedCppCodec::from(data.codec);
    let input = data.data;

    let mut compressed = Vec::new();
    codec.encode(&input, &mut compressed).unwrap();

    let mut decoded = Vec::new();
    codec
        .decode(&compressed, &mut decoded, None)
        .expect("decode");

    // Verify roundtrip
    if decoded.len() + input.len() < 200 {
        assert_eq!(input, decoded.as_slice(), "Decompressed output mismatches");
    } else {
        assert_eq!(decoded.len(), input.len(), "Decompressed length mismatch");
        for (i, (&original, &out)) in input.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(
                original, out,
                "Mismatch at position {i}: expected {original}, got {out}"
            );
        }
    }
});
