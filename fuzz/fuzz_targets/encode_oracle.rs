#![no_main]

//! Fuzz the pure-Rust `AnyLenCodec` implementations for self-consistency.
//!
//! Every codec must satisfy the fundamental invariant: compress(x) then
//! decompress(compress(x)) == x.  This target exercises all Rust codecs
//! against arbitrary input.

use libfuzzer_sys::fuzz_target;
mod common;
use common::{AnyLenSelector, FuzzInput, instantiate_anylen_codec};

fuzz_target!(|data: FuzzInput<AnyLenSelector>| {
    // Only exercise Rust codecs in this target; C++ roundtrip is in cpp_roundtrip.
    if data.codec.use_cpp {
        return;
    }
    let (name, mut codec) = instantiate_anylen_codec(data.codec);

    let input = &data.data;
    let mut compressed = Vec::new();
    if codec.encode(input, &mut compressed).is_err() {
        return;
    }

    let mut decompressed = Vec::new();
    codec
        .decode(&compressed, &mut decompressed, None)
        .expect("Rust decompress of self-compressed data must not fail");

    assert_eq!(
        decompressed, *input,
        "Rust roundtrip mismatch for codec {name}",
    );
});
