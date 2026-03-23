#![no_main]

//! Fuzz the Rust FastPFOR decoder against **arbitrary** (potentially malformed) compressed bytes.
//!
//! Why this target is needed
//! -------------------------
//! The existing `encode_oracle` target only feeds *well-formed* data to the Rust
//! decoder (it first compresses valid input, then decompresses).
//! That means corrupted or truncated compressed streams never reach the decoder, so
//! out-of-bounds panics in `decode_page` are invisible to the fuzzer.
//!
//! This target removes any oracle entirely: arbitrary bytes are reinterpreted as `u32` words
//! and handed straight to the Rust decoder.  The only contract we enforce is:
//!
//!   * A successful `Ok(...)` must produce the expected output (we don't verify correctness,
//!     only that no panic occurs).
//!   * An `Err(...)` is acceptable — the decoder is allowed to reject garbage input.
//!   * A **panic** is never acceptable.

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
mod common;
use common::{AnyLenSelector, instantiate_anylen_codec};

/// Fuzz input: raw compressed bytes plus the codec selector.
#[derive(Arbitrary, Debug)]
struct FuzzInput {
    /// Raw bytes that will be reinterpreted as `&[u32]` compressed data.
    compressed_bytes: Vec<u8>,
    codec: AnyLenSelector,
}

/// Maximum number of `u32` words to feed to the decoder.
/// Keeps allocations bounded even with malicious `n_blocks` headers.
const MAX_COMPRESSED_WORDS: usize = 4096;

fuzz_target!(|data: FuzzInput| {
    // Only fuzz Rust codecs — C++ panics on malformed input are out of scope.
    if data.codec.use_cpp {
        return;
    }

    // Align the byte slice to u32 by zero-padding to the next 4-byte boundary.
    let mut bytes = data.compressed_bytes;
    let rem = bytes.len() % 4;
    if rem != 0 {
        bytes.resize(bytes.len() + (4 - rem), 0);
    }

    let compressed: Vec<u32> = bytes
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .take(MAX_COMPRESSED_WORDS)
        .collect();

    let (_name, mut codec) = instantiate_anylen_codec(data.codec);

    // The decoder must either succeed or return an error — a panic is a bug.
    let mut output = Vec::new();
    let _ = codec.decode(&compressed, &mut output, None);
});
