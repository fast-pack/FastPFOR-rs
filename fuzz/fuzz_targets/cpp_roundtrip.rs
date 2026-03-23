#![no_main]

//! Fuzz C++ codec roundtrip: compress then decompress and assert equality.

use libfuzzer_sys::fuzz_target;
mod common;
use common::{AnyLenSelector, FuzzInput, instantiate_anylen_codec};

fuzz_target!(|data: FuzzInput<AnyLenSelector>| {
    // Only exercise C++ codecs in this target; Rust self-roundtrip is in compress_oracle.
    if !data.codec.use_cpp {
        return;
    }
    let (name, mut codec) = instantiate_anylen_codec(data.codec);

    let input = &data.data;
    let mut compressed = Vec::new();
    codec
        .encode(input, &mut compressed)
        .expect("C++ compression failed");

    let mut decompressed = Vec::new();
    codec
        .decode(&compressed, &mut decompressed, None)
        .expect("C++ decompression failed");

    assert_eq!(
        decompressed, *input,
        "C++ roundtrip mismatch for codec {name}",
    );
});
