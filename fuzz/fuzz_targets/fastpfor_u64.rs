#![no_main]

use fastpfor::cpp::{CppFastPFor128, CppFastPFor256};
use fastpfor::{BlockCodec64, FastPFor128, FastPFor256};
use libfuzzer_sys::fuzz_target;

#[derive(arbitrary::Arbitrary, Debug)]
struct Input {
    data: Vec<u64>,
    use_256: bool,
}

fn check(rust: &mut impl BlockCodec64, cpp: &mut impl BlockCodec64, data: &[u64], name: &str) {
    let mut rust_enc = Vec::new();
    rust.encode64(data, &mut rust_enc).expect("Rust encode64 failed");

    let mut cpp_enc = Vec::new();
    cpp.encode64(data, &mut cpp_enc).expect("C++ encode64 failed");

    assert_eq!(rust_enc, cpp_enc, "{name}: Rust and C++ encode64 bytes differ");

    let mut rust_dec = Vec::new();
    rust.decode64(&rust_enc, &mut rust_dec)
        .expect("Rust decode64 of own output failed");
    assert_eq!(rust_dec, data, "{name}: Rust roundtrip mismatch");

    let mut cross = Vec::new();
    rust.decode64(&cpp_enc, &mut cross)
        .expect("Rust decode64 of C++ output failed");
    assert_eq!(cross, data, "{name}: Rust could not decode C++ output");

    let mut cpp_dec = Vec::new();
    cpp.decode64(&rust_enc, &mut cpp_dec)
        .expect("C++ decode64 of Rust output failed");
    assert_eq!(cpp_dec, data, "{name}: C++ could not decode Rust output");
}

fuzz_target!(|input: Input| {
    if input.use_256 {
        check(
            &mut FastPFor256::default(),
            &mut CppFastPFor256::default(),
            &input.data,
            "FastPFor256",
        );
    } else {
        check(
            &mut FastPFor128::default(),
            &mut CppFastPFor128::default(),
            &input.data,
            "FastPFor128",
        );
    }
});
