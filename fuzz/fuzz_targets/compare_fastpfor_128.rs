#![no_main]

//! Cross-decoder fuzz: encode with `CppFastPFor128`, match it with `FastPFor128` output,
//! then verify that `FastPFor128` (pure Rust) and `CppFastPFor128` both reproduce the original
//! input exactly.
//!
//! # Why `CppSimdFastPFor128` is excluded
//!
//! `CppSimdFastPFor128` (`SIMDFastPFor`) uses a **different wire format** for
//! block data than the scalar `CppFastPFor128` (`FastPFor`).  The scalar codec
//! calls `fastpackwithoutmask`, which stores 32 consecutive values
//! sequentially into one 32-bit word (v[0] | v[1]<<1 | … | v[31]<<31).
//!
//! The SIMD codec calls `SIMD_fastpack_32` / `simdpack`, which uses SSE2 to
//! process four 32-bit values simultaneously across the four 128-bit lanes of
//! an `__m128i`.  For `bit=1` this produces the **transposed** layout:
//!
//!   lane 0  = v[0] | v[4]<<1 | v[8]<<2 | … | v[28]<<7 | …
//!   lane 1  = v[1] | v[5]<<1 | v[9]<<2 | … | v[29]<<7 | …
//!   …
//!
//! which is entirely different from the scalar word layout.
//!
//! Because the block bit-packing formats are incompatible, a stream produced
//! by `CppFastPFor128` cannot be correctly decoded by `CppSimdFastPFor128`
//! (and vice versa).

use fastpfor::cpp::CppFastPFor128;
use fastpfor::{AnyLenCodec, FastPFor128};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<u32>| {
    let mut compressed = Vec::new();
    CppFastPFor128::default()
        .encode(&data, &mut compressed)
        .expect("any data must be encodable");

    let mut rust_compressed = Vec::new();
    FastPFor128::default()
        .encode(&data, &mut rust_compressed)
        .expect("any data must be encodable");
    assert_eq!(
        compressed, rust_compressed,
        "CppFastPFor128 and FastPFor128 produced different compressed output",
    );

    let mut rust_out = Vec::new();
    FastPFor128::default()
        .decode(&compressed, &mut rust_out, None)
        .expect("FastPFor128 (Rust) failed to decode CppFastPFor128-encoded data");
    assert_eq!(
        rust_out, data,
        "FastPFor128 (Rust) decoded output does not match original",
    );

    let mut cpp_out = Vec::new();
    CppFastPFor128::default()
        .decode(&compressed, &mut cpp_out, None)
        .expect("CppFastPFor128 failed to decode its own encoded data");
    assert_eq!(
        cpp_out, data,
        "CppFastPFor128 decoded output does not match original",
    );
});
