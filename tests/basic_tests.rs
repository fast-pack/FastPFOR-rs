//! Basic integration tests exercising the public `BlockCodec` and `AnyLenCodec` APIs.

#![cfg(feature = "rust")]

#[path = "../src/test_utils.rs"]
mod test_utils;

use fastpfor::{BlockCodec, FastPForBlock128, FastPForBlock256, slice_to_blocks};
use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng};

use crate::test_utils::{block_roundtrip_all, roundtrip_all};

mod common;

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "cpp")]
fn saul_test() {
    roundtrip_all(&[2u32, 3, 4, 5]);
}

/// Sub-block-sized inputs produce no output via `BlockCodec`.
#[test]
fn spurious_out_test() {
    fn check<C: BlockCodec + Default>(len: usize) {
        let x = vec![0u32; 1024];
        let (blocks, _) = slice_to_blocks::<C>(&x[..len]);
        let mut out = Vec::new();
        C::default().encode_blocks(blocks, &mut out).unwrap();
        assert!(out.is_empty() || blocks.is_empty());
    }
    for len in 0..32usize {
        check::<FastPForBlock256>(len);
        check::<FastPForBlock128>(len);
    }
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
    let data: Vec<u32> = (0..65536)
        .map(|_| StdRng::seed_from_u64(123456).random())
        .collect();
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
