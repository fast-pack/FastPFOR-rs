#![no_main]

use libfuzzer_sys::fuzz_target;
mod common;
use common::{FuzzAnyLen, FuzzInput, HexSlice, instantiate_pair, resolve_encode_compare_pair};

#[derive(arbitrary::Arbitrary, Debug)]
struct PairSelector {
    idx: u8,
    pass_expected_len: bool,
}

fn decode(codec: &mut FuzzAnyLen, words: &[u32], expected_len: Option<u32>) -> Vec<u32> {
    let mut out = Vec::new();
    codec
        .decode(words, &mut out, expected_len)
        .unwrap_or_else(|e| panic!("decode of {:?} failed: {e:?}", HexSlice(words)));
    out
}

fuzz_target!(|data: FuzzInput<PairSelector>| {
    let Some(pair) = resolve_encode_compare_pair(data.codec.idx) else {
        return;
    };
    let (mut rust, mut cpp) = instantiate_pair(pair);
    let input = &data.data;
    let expected_len = data
        .codec
        .pass_expected_len
        .then(|| u32::try_from(input.len()).unwrap());

    let mut rust_enc = Vec::new();
    rust.encode(input, &mut rust_enc)
        .expect("Rust encode failed");
    let mut cpp_enc = Vec::new();
    cpp.encode(input, &mut cpp_enc).expect("C++ encode failed");

    let name = pair.name;
    assert_eq!(
        decode(&mut rust, &cpp_enc, expected_len),
        *input,
        "{name}: Rust decoding C++ output"
    );
    assert_eq!(
        decode(&mut cpp, &rust_enc, expected_len),
        *input,
        "{name}: C++ decoding Rust output"
    );
    assert_eq!(
        decode(&mut rust, &rust_enc, expected_len),
        *input,
        "{name}: Rust roundtrip"
    );
    assert_eq!(
        decode(&mut cpp, &cpp_enc, expected_len),
        *input,
        "{name}: C++ roundtrip"
    );
});
