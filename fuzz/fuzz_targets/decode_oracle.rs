#![no_main]

//! Cross-codec oracle: independent Rust and C++ roundtrips, decompressed values must match.
//!
//! Uses matching Rust/C++ pairs from [`RUST`] and [`CPP`] (by the same index).
//! Both sides compress the input independently and decompress independently;
//! the decompressed output from each must equal the original input.
//!
//! Both sides use the same wire format so only the final decompressed values are compared.

use libfuzzer_sys::fuzz_target;
mod common;
use common::{CPP, FuzzInput, RUST};

/// Selects a matching Rust/C++ pair by a single index into the shorter of the
/// two lists.
#[derive(arbitrary::Arbitrary, Clone, Copy, Debug)]
struct CompatSelector {
    idx: u8,
}

fuzz_target!(|data: FuzzInput<CompatSelector>| {
    let input = &data.data;
    if input.is_empty() {
        return;
    }

    // Use the same index for both lists; clamp to the shorter list.
    let n = RUST.len().min(CPP.len());
    let i = data.codec.idx as usize % n;
    let (rust_name, make_rust) = RUST[i];
    let (cpp_name, make_cpp) = CPP[i];

    let mut rust_codec = make_rust();
    let mut cpp_codec = make_cpp();

    // Rust roundtrip
    let mut rust_compressed = Vec::new();
    if rust_codec.encode(input, &mut rust_compressed).is_err() {
        return;
    }
    let mut rust_decompressed = Vec::new();
    rust_codec
        .decode(&rust_compressed, &mut rust_decompressed, None)
        .expect("Rust decompress of self-compressed data must not fail");

    // C++ roundtrip (independent oracle)
    let mut cpp_compressed = Vec::new();
    cpp_codec
        .encode(input, &mut cpp_compressed)
        .expect("C++ compression failed");
    let mut cpp_decompressed = Vec::new();
    cpp_codec
        .decode(&cpp_compressed, &mut cpp_decompressed, None)
        .expect("C++ decompression failed");

    assert_eq!(
        rust_decompressed, *input,
        "Rust roundtrip failed for codec {rust_name}",
    );
    assert_eq!(
        cpp_decompressed, *input,
        "C++ roundtrip failed for codec {cpp_name}",
    );
});
