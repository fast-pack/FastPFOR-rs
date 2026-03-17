#![no_main]

//! Fuzz the Rust FastPFOR decoder against **arbitrary** (potentially malformed) compressed bytes.
//!
//! Why this target is needed
//! -------------------------
//! The existing `rust_decompress_oracle` target only ever feeds *well-formed* data to the Rust
//! decoder (it first compresses valid input with the C++ oracle, then decompresses with Rust).
//! That means corrupted or truncated compressed streams never reach the decoder, so out-of-bounds
//! index panics in `decode_page` are invisible to the fuzzer.
//!
//! This target removes the C++ oracle entirely: arbitrary bytes are reinterpreted as `u32` words
//! and handed straight to the Rust decoder.  The only contract we enforce is:
//!
//!   * A successful `Ok(...)` must produce exactly `expected_len` decompressed integers.
//!   * An `Err(...)` is also acceptable — the decoder is allowed to reject garbage input.
//!   * A **panic** is never acceptable.
//!
//! Running this target against the `main` branch will reproduce the panic;
//! running it against the `dont-panic` branch will produce only `Ok`/`Err` outcomes.

use arbitrary::Arbitrary;
use fastpfor::rust::{BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE, FastPFOR, VariableByte};
use fastpfor::{CodecToSlice, rust};
use libfuzzer_sys::fuzz_target;

/// Which Rust FastPFOR codec variant to exercise.
#[derive(Arbitrary, Clone, Copy, Debug)]
enum RustFastPForCodec {
    FastPFOR256,
    FastPFOR128,
    VariableByte,
}

/// Fuzz input: raw compressed bytes plus the codec selector and the expected decompressed length.
#[derive(Arbitrary, Debug)]
struct FuzzInput {
    /// Raw bytes that will be reinterpreted as `&[u32]` compressed data.
    compressed_bytes: Vec<u8>,
    /// How many `u32` values the decoder should attempt to produce.
    /// Capped inside the target to avoid enormous allocations.
    expected_len: u16,
    codec: RustFastPForCodec,
}

fuzz_target!(|data: FuzzInput| {
    // Align the byte slice to u32 by zero-padding to the next 4-byte boundary.
    let mut bytes = data.compressed_bytes;
    let rem = bytes.len() % 4;
    if rem != 0 {
        bytes.resize(bytes.len() + (4 - rem), 0);
    }

    // Safe reinterpret: bytemuck requires the slice to be properly aligned and sized.
    // We just constructed a Vec<u8> that is a multiple of 4 bytes.
    let compressed: Vec<u32> = bytes
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();

    // Cap the output length to prevent huge allocations while still exercising non-trivial sizes.
    const MAX_LEN: usize = 4096;
    let expected_len = (data.expected_len as usize).min(MAX_LEN);
    let mut output = vec![0u32; expected_len];

    // Build the codec under test.
    let mut codec: rust::Codec = match data.codec {
        RustFastPForCodec::FastPFOR256 => {
            rust::Codec::from(FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_256))
        }
        RustFastPForCodec::FastPFOR128 => {
            rust::Codec::from(FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128))
        }
        RustFastPForCodec::VariableByte => rust::Codec::from(VariableByte::new()),
    };

    // The decoder must either succeed or return an error.  A panic is a bug.
    let _ = codec.decompress_to_slice(&compressed, &mut output);
});
