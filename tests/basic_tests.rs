//! Basic integration tests exercising the public `BlockCodec` and `AnyLenCodec` APIs.

#![cfg(feature = "rust")]

#[path = "../src/test_utils.rs"]
mod test_utils;

use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng};

use crate::test_utils::{RNG_SEED, block_roundtrip_all, roundtrip_all};

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "cpp")]
fn saul_test() {
    roundtrip_all(&[2u32, 3, 4, 5]);
}

/// `AnyLenCodec` round-trips empty input correctly.
#[test]
fn zero_in_zero_out_test() {
    roundtrip_all(&[]);
}

#[test]
fn test_increasing_sequence() {
    let data: Vec<u32> = (0..256u32).collect();
    roundtrip_all(&data);
}

#[test]
fn test_random_numbers() {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    let data: Vec<u32> = (0..65536).map(|_| rng.random()).collect();
    roundtrip_all(&data);
}

/// `BlockCodec` round-trip using `slice_to_blocks` to split aligned input.
#[test]
fn block_codec_roundtrip() {
    let data: Vec<u32> = (0u32..512).collect();
    block_roundtrip_all(&data);
}

/// `AnyLenCodec` round-trip with random values at various lengths.
#[test]
fn random_roundtrip() {
    let mut rng = rand::rng();
    for n in [128usize, 300, 512, 1000, 4096] {
        let data: Vec<u32> = (0..n).map(|_| rng.random()).collect();
        roundtrip_all(&data);
    }
}
