//! Common test utilities for codec compatibility testing.

#![cfg(any(feature = "rust", feature = "cpp"))]
#![allow(dead_code, reason = "This file is shared by several test modules")]

#[path = "../src/test_utils.rs"]
mod test_utils;

use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng as _};
use test_utils::RNG_SEED;

/// Returns various input sizes to test codec behavior (multiples of 128).
#[must_use]
pub fn test_input_sizes() -> Vec<usize> {
    (1..=8).map(|exp| (1usize << exp) * 128).collect()
}

/// Generates test data vectors of size `n` with various patterns.
#[must_use]
pub fn get_test_cases(n: usize) -> Vec<Vec<u32>> {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);

    vec![
        // Zeroes
        vec![0u32; n],
        // Same non-zero
        vec![14u32; n],
        // Ascending values
        (0..n).map(|i| i as u32).collect::<Vec<u32>>(),
        // Descending values
        (0..n).rev().map(|i| i as u32).collect::<Vec<u32>>(),
        // Bit-flipping pattern
        (0..n)
            .map(|i| ((i as u32) * 32) ^ ((i as u32) >> 1))
            .collect::<Vec<u32>>(),
        // Alternating large and small values
        (0..n)
            .map(|i| {
                let ui = i as u32;
                if ui % 2 == 0 { 1 << 30 } else { 3 }
            })
            .collect::<Vec<u32>>(),
        // Random u32 values
        (0..n)
            .map(|_| rng.random_range(0..(1 << 31)))
            .collect::<Vec<u32>>(),
        // Spike in the middle
        (0..n)
            .map(|i| if i == n / 2 { u32::MAX } else { 1 })
            .collect::<Vec<u32>>(),
        // An empty vector
        Vec::new(),
    ]
}
