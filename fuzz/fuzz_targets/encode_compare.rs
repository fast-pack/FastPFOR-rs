#![no_main]

//! Fuzz target: encode the same input with Rust and C++ implementations and assert bit-identical output.
//!
//! Codec pairs (Rust vs C++) expected to produce identical compressed bytes:
//! - FastPFor128 vs CppFastPFor128
//! - FastPFor256 vs CppFastPFor256
//! - VariableByte vs CppVarInt
//! - JustCopy vs CppCopy

use libfuzzer_sys::fuzz_target;
mod common;
use common::{FuzzInput, instantiate_pair, resolve_encode_compare_pair};

#[derive(arbitrary::Arbitrary, Debug)]
struct PairSelector {
    idx: u8,
}

fuzz_target!(|data: FuzzInput<PairSelector>| {
    let Some(pair) = resolve_encode_compare_pair(data.codec.idx) else {
        return;
    };

    let (mut rust_codec, mut cpp_codec) = instantiate_pair(pair);

    let mut rust_out = Vec::new();
    rust_codec
        .encode(&data.data, &mut rust_out)
        .expect("Rust encode failed");

    let mut cpp_out = Vec::new();
    cpp_codec
        .encode(&data.data, &mut cpp_out)
        .expect("C++ encode must not fail when Rust encode succeeded");

    assert_eq!(
        rust_out, cpp_out,
        "Bit-identical output failed for pair {}: Rust and C++ compressed output differ",
        pair.name,
    );

    let mut decoded = Vec::new();
    rust_codec
        .decode(&rust_out, &mut decoded, None)
        .expect("Rust decode of self-compressed data must not fail");
    assert_eq!(
        decoded, data.data,
        "Rust roundtrip failed for pair {}: decompressed value differs from original input",
        pair.name,
    );

    decoded.truncate(0);
    cpp_codec
        .decode(&cpp_out, &mut decoded, None)
        .expect("C++ decode of self-compressed data must not fail");
    assert_eq!(
        decoded, data.data,
        "C++ roundtrip failed for pair {}: decompressed value differs from original input",
        pair.name,
    );
});
